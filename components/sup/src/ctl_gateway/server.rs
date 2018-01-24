// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::cell::RefCell;
use std::error;
use std::fmt;
use std::io;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use common::ui::UIWriter;
use futures::future::{self, Either};
use futures::prelude::*;
use futures::sync::mpsc;
use protobuf;
use tokio::net::TcpListener;
use tokio_core::reactor;
use tokio_io::AsyncRead;

use super::ListenAddr;
use super::codec::*;
use manager::{Manager, ManagerConfig};
use manager::ctl::*;
use net::{NetErr, NetResult};
use protocols;

static LOGKEY: &'static str = "CL";

pub type CtlSender = mpsc::UnboundedSender<SrvWireMessage>;
pub type CtlReceiver = mpsc::UnboundedReceiver<SrvWireMessage>;

/// Sender from the CtlGateway to the Manager to issue control commands.
pub type MgrSender = mpsc::UnboundedSender<CtlCommand>;
/// Receiver from the Manager to the CtlGateway to receive control commands.
pub type MgrReceiver = mpsc::UnboundedReceiver<CtlCommand>;

#[derive(Debug)]
pub enum HandlerError {
    Io(io::Error),
    NetErr(NetErr),
    SendError(mpsc::SendError<CtlCommand>),
}

impl error::Error for HandlerError {
    fn description(&self) -> &str {
        match *self {
            HandlerError::Io(ref err) => err.description(),
            HandlerError::NetErr(ref err) => err.description(),
            HandlerError::SendError(ref err) => err.description(),
        }
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match *self {
            HandlerError::Io(ref err) => format!("{}", err),
            HandlerError::NetErr(ref err) => format!("{}", err),
            HandlerError::SendError(ref err) => format!("{}", err),
        };
        write!(f, "{}", content)
    }
}

impl From<NetErr> for HandlerError {
    fn from(err: NetErr) -> Self {
        HandlerError::NetErr(err)
    }
}

impl From<io::Error> for HandlerError {
    fn from(err: io::Error) -> Self {
        HandlerError::Io(err)
    }
}

impl From<mpsc::SendError<CtlCommand>> for HandlerError {
    fn from(err: mpsc::SendError<CtlCommand>) -> Self {
        HandlerError::SendError(err)
    }
}

#[derive(Clone)]
pub struct CtlRequest {
    tx: CtlSender,
    transaction: Option<SrvTxn>,
}

impl CtlRequest {
    pub fn reply_partial<T>(&mut self, msg: T)
    where
        T: Into<SrvWireMessage>,
    {
        self.send_msg(msg, false)
    }

    pub fn reply_complete<T>(&mut self, msg: T)
    where
        T: Into<SrvWireMessage>,
    {
        self.send_msg(msg, true)
    }

    pub fn transactional(&self) -> bool {
        self.transaction.is_some()
    }

    fn send_msg<T>(&mut self, msg: T, complete: bool)
    where
        T: Into<SrvWireMessage>,
    {
        let mut wire: SrvWireMessage = msg.into();
        wire.reply_for(self.transaction.unwrap(), complete);
        self.tx.start_send(wire).unwrap();
    }
}

impl UIWriter for CtlRequest {
    type ProgressBar = protocols::NetProgressBar;

    fn out(&mut self) -> &mut io::Write {
        self as &mut io::Write
    }

    fn err(&mut self) -> &mut io::Write {
        self as &mut io::Write
    }

    fn is_colored(&self) -> bool {
        // Let's put this as a flag on the txn?
        true
    }

    fn is_a_terminal(&self) -> bool {
        // Let'sput this as a flag on the txn?
        true
    }

    fn progress(&self) -> Option<Self::ProgressBar> {
        Some(Self::ProgressBar::new(self.clone()))
    }
}

impl io::Write for CtlRequest {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let line = String::from_utf8_lossy(buf).into_owned();
        output!("{}", line);
        let mut msg = SrvMessage::<protocols::ctl::ConsoleLine>::new();
        msg.set_line(line);
        self.reply_partial(msg);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct CtlCommand {
    pub req: CtlRequest,
    // JW: We have to clone `opts` here because we can't
    // capture `opts` unless we use an `FnOnce` instead of
    // an `Fn`. While we actually *do* mean `FnOnce`, we
    // can't place an `FnOnce` in a `Box`. There is a new
    // type called `FnBox` which exists only on nightly
    // right now which accomplishes this but it won't
    // stabilize because the Rust core team feels that
    // they should just get `Box<FnOnce>` working. We'll
    // just deal with the minor over allocation for now.
    //
    // https://github.com/rust-lang/rust/issues/28796
    fun: Box<Fn(&ManagerConfig, &mut CtlRequest) -> NetResult<()> + Send>,
}

impl CtlCommand {
    pub fn new<F>(tx: CtlSender, txn: Option<SrvTxn>, fun: F) -> Self
    where
        F: Fn(&ManagerConfig, &mut CtlRequest) -> NetResult<()> + Send + 'static,
    {
        CtlCommand {
            fun: Box::new(fun),
            req: CtlRequest {
                tx: tx,
                transaction: txn,
            },
        }
    }

    pub fn run(&mut self, cfg: &ManagerConfig) -> NetResult<()> {
        (self.fun)(cfg, &mut self.req)
    }
}

struct Client {
    handle: reactor::Handle,
    state: Rc<RefCell<SrvState>>,
}

impl Client {
    pub fn serve(self, socket: SrvStream) -> Box<Future<Item = (), Error = HandlerError>> {
        let mgr_tx = self.state.borrow().mgr_tx.clone();
        Box::new(self.handshake(socket).and_then(|socket| {
            SrvHandler::new(socket, mgr_tx)
        }))
    }

    fn handshake(&self, socket: SrvStream) -> Box<Future<Item = SrvStream, Error = HandlerError>> {
        let auth_key = self.state.borrow().auth_key.to_string();
        let handshake = socket
            .into_future()
            .map_err(|(err, _)| HandlerError::from(err))
            .and_then(move |(m, io)| {
                m.map_or_else(
                    || {
                        Err(HandlerError::from(
                            io::Error::from(io::ErrorKind::UnexpectedEof),
                        ))
                    },
                    move |m| {
                        if m.message_id() != "Handshake" {
                            debug!("No handshake");
                            return Err(HandlerError::from(
                                io::Error::from(io::ErrorKind::ConnectionAborted),
                            ));
                        }
                        match m.parse::<protocols::ctl::Handshake>() {
                            Ok(decoded) => {
                                trace!("Received handshake, {:?}", decoded);
                                if decoded.get_auth_key() == auth_key {
                                    // JW TODO: handle a case when we receive a non-txn msg
                                    Ok((decoded.transaction.unwrap(), io))
                                } else {
                                    Err(HandlerError::from(
                                        io::Error::from(io::ErrorKind::ConnectionRefused),
                                    ))
                                }
                            }
                            Err(err) => {
                                warn!("Handshake error, {:?}", err);
                                Err(HandlerError::from(
                                    io::Error::from(io::ErrorKind::ConnectionAborted),
                                ))
                            }
                        }
                    },
                )
            })
            .and_then(|(txn, io)| {
                send_complete(io, txn, SrvMessage::<protocols::ctl::NetOk>::new())
            });
        let timeout = reactor::Timeout::new(Duration::from_millis(10_000), &self.handle).unwrap();
        Box::new(handshake.select2(timeout).then(|res| match res {
            Ok(Either::A((hs, _to))) => future::ok(hs),
            Ok(Either::B((_to, _hs))) => {
                future::err(HandlerError::from(
                    io::Error::new(io::ErrorKind::TimedOut, "client timed out"),
                ))
            }
            Err(Either::A((err, _))) => future::err(HandlerError::from(err)),
            Err(Either::B((err, _))) => future::err(HandlerError::from(err)),
        }))
    }
}

/// A `Future` that will resolve into a stream of one or more `SrvWireMessage` replies.
#[must_use = "futures do nothing unless polled"]
struct SrvHandler {
    io: SrvStream,
    state: SrvHandlerState,
    mgr_tx: MgrSender,
    rx: CtlReceiver,
    tx: CtlSender,
}

impl SrvHandler {
    fn new(io: SrvStream, mgr_tx: MgrSender) -> Self {
        let (tx, rx) = mpsc::unbounded();
        SrvHandler {
            io: io,
            state: SrvHandlerState::Receiving,
            mgr_tx: mgr_tx,
            rx: rx,
            tx: tx,
        }
    }
}

impl Future for SrvHandler {
    type Item = ();
    type Error = HandlerError;

    fn poll(&mut self) -> Poll<(), Self::Error> {
        loop {
            match self.state {
                SrvHandlerState::Receiving => {
                    match try_ready!(self.io.poll()) {
                        Some(msg) => {
                            trace!("OnMessage, {}", msg.message_id());
                            let cmd =
                                match msg.message_id() {
                                    "SvcLoad" => {
                                        let m = msg.parse::<protocols::ctl::SvcLoad>().unwrap();
                                        let opts = SvcLoadOpts::new(m);
                                        CtlCommand::new(
                                            self.tx.clone(),
                                            msg.transaction(),
                                            move |cfg, req| {
                                                Manager::service_load(cfg, req, opts.clone())
                                            },
                                        )
                                    }
                                    "SvcStart" => {
                                        let m = msg.parse::<protocols::ctl::SvcStart>().unwrap();
                                        let opts = SvcStartOpts::new(m);
                                        CtlCommand::new(
                                            self.tx.clone(),
                                            msg.transaction(),
                                            move |cfg, req| {
                                                Manager::service_start(cfg, req, opts.clone())
                                            },
                                        )
                                    }
                                    _ => {
                                        warn!("Unhandled message, {}", msg.message_id());
                                        break;
                                    }
                                };
                            match self.mgr_tx.start_send(cmd) {
                                Ok(AsyncSink::Ready) => {
                                    self.state = SrvHandlerState::Sending;
                                    continue;
                                }
                                Ok(AsyncSink::NotReady(_)) => return Ok(Async::NotReady),
                                Err(err) => {
                                    warn!("ManagerReceiver err, {:?}", err);
                                    return Err(HandlerError::from(err));
                                }
                            }
                        }
                        None => break,
                    }
                }
                SrvHandlerState::Sending => {
                    match self.rx.poll() {
                        Ok(Async::Ready(Some(msg))) => {
                            trace!("MgrSender -> SrvHandler, {:?}", msg);
                            if msg.is_complete() {
                                self.state = SrvHandlerState::Sent;
                            }
                            try_nb!(self.io.start_send(msg));
                            try_ready!(self.io.poll_complete());
                            continue;
                        }
                        Ok(Async::Ready(None)) => self.state = SrvHandlerState::Sent,
                        Ok(Async::NotReady) => return Ok(Async::NotReady),
                        Err(()) => break,
                    }
                }
                SrvHandlerState::Sent => {
                    trace!("OnMessage complete");
                    break;
                }
            }
        }
        Ok(Async::Ready(()))
    }
}

enum SrvHandlerState {
    /// Handler is Receiving/Waiting for message from client.
    Receiving,
    /// Handler has sent a request to the Manager and is streaming replies back to the client
    /// socket.
    Sending,
    /// All messages have been sent to the client and the Handler is now flushing the connection.
    Sent,
}

struct SrvState {
    auth_key: String,
    mgr_tx: MgrSender,
}

pub fn run(listen_addr: ListenAddr, mgr_tx: MgrSender) {
    thread::Builder::new()
        .name("ctl-gateway".to_string())
        .spawn(move || {
            let mut core = reactor::Core::new().unwrap();
            let handle = core.handle();
            let listener = TcpListener::bind(&listen_addr).unwrap();
            let state = SrvState {
                auth_key: "letmein".to_string(),
                mgr_tx: mgr_tx,
            };
            let state = Rc::new(RefCell::new(state));
            let clients = listener.incoming().map(|socket| {
                let addr = socket.peer_addr().unwrap();
                let io = socket.framed(SrvCodec::new());
                (
                    Client {
                        handle: handle.clone(),
                        state: state.clone(),
                    }.serve(io),
                    addr,
                )
            });
            let server = clients.for_each(|(client, addr)| {
                handle.spawn(client.then(move |res| {
                    debug!("DISCONNECTED from {:?} with result {:?}", addr, res);
                    future::ok(())
                }));
                Ok(())
            });
            core.run(server)
        })
        .expect("ctl-gateway thread start failure");
}

fn _debugf<F: Future<Item = (), Error = ()>>(_: F) {}
fn _debugs<S: Stream<Item = (), Error = ()>>(_: S) {}

fn send_complete<T>(
    socket: SrvStream,
    txn: SrvTxn,
    reply: SrvMessage<T>,
) -> Box<Future<Item = SrvStream, Error = HandlerError>>
where
    T: protobuf::Message + protobuf::MessageStatic,
{
    let mut message: SrvWireMessage = reply.into();
    message.reply_for(txn, true);
    Box::new(socket.send(message).map_err(|e| HandlerError::from(e)))
}
