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

use std::error;
use std::fmt;
use std::io;
use std::net::SocketAddr;

use futures::sink;
use futures::prelude::*;
use protobuf;
use tokio::net::TcpStream;
use tokio_io::AsyncRead;

use super::codec::*;
use net::NetErr;
use protocols;

pub type SrvSend = sink::Send<SrvStream>;

#[derive(Debug)]
pub enum SrvClientError {
    ConnectionClosed,
    Io(io::Error),
    NetErr(NetErr),
}

impl error::Error for SrvClientError {
    fn description(&self) -> &str {
        match *self {
            SrvClientError::ConnectionClosed => "Connection closed",
            SrvClientError::Io(ref err) => err.description(),
            SrvClientError::NetErr(ref err) => err.description(),
        }
    }
}

impl fmt::Display for SrvClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let content = match *self {
            SrvClientError::ConnectionClosed => format!("Connection closed"),
            SrvClientError::Io(ref err) => format!("{}", err),
            SrvClientError::NetErr(ref err) => format!("{}", err),
        };
        write!(f, "{}", content)
    }
}

impl From<NetErr> for SrvClientError {
    fn from(err: NetErr) -> Self {
        SrvClientError::NetErr(err)
    }
}

impl From<io::Error> for SrvClientError {
    fn from(err: io::Error) -> Self {
        SrvClientError::Io(err)
    }
}

pub struct SrvClient {
    socket: SrvStream,
    next_txn: u32,
}

impl SrvClient {
    pub fn connect<T>(
        addr: &SocketAddr,
        auth_key: T,
    ) -> Box<Future<Item = SrvClient, Error = SrvClientError> + 'static>
    where
        T: ToString,
    {
        let auth_key = auth_key.to_string();
        let conn = TcpStream::connect(addr)
            .map_err(SrvClientError::from)
            .and_then(move |socket| {
                let client = Self::new(socket);
                let mut request = SrvMessage::<protocols::ctl::Handshake>::new();
                request.set_auth_key(auth_key);
                client
                    .call(request)
                    .into_future()
                    .map_err(|(err, _)| err)
                    .and_then(move |(m, io)| {
                        m.map_or_else(|| Err(SrvClientError::ConnectionClosed), move |m| {
                        m.try_ok().map_err(SrvClientError::from).and_then(|()| Ok(io.into_inner()))
                    })
                    })
            });
        Box::new(conn)
    }

    fn new(socket: TcpStream) -> Self {
        SrvClient {
            socket: socket.framed(SrvCodec::new()),
            next_txn: 0,
        }
    }

    /// Send a transactional request to the connected server. The returned `SrvReply` is a Stream
    /// containing one or more `SrvWireMessage` responses for the given request.
    pub fn call<T>(mut self, mut request: SrvMessage<T>) -> SrvReply
    where
        T: protobuf::Message + protobuf::MessageStatic,
    {
        self.next_txn = SrvTxn::next_id(self.next_txn);
        request.transaction = Some(SrvTxn::new(self.next_txn));
        trace!("Sending SrvMessage -> {:?}", request);
        // JW TODO: Select & timeout here. Allow user to configure timeout on Client struct.
        let message: SrvWireMessage = request.into();
        SrvReply::new(self.socket.send(message))
    }

    /// Send a non-transactional request to the connected server.
    pub fn cast<T>(self, mut request: SrvMessage<T>) -> SrvSend
    where
        T: protobuf::Message + protobuf::MessageStatic,
    {
        request.transaction = None;
        let message: SrvWireMessage = request.into();
        self.socket.send(message)
    }
}

/// A `Future` that will resolve into a stream of one or more `SrvWireMessage` replies.
#[must_use = "futures do nothing unless polled"]
pub struct SrvReply {
    io: sink::Send<SrvStream>,
    state: SrvRequestState,
}

impl SrvReply {
    fn new(io: sink::Send<SrvStream>) -> Self {
        SrvReply {
            io: io,
            state: SrvRequestState::Sending,
        }
    }

    pub fn into_inner(self) -> SrvClient {
        match self.state {
            SrvRequestState::Receiving(io, true) => {
                // JW: Recreating the SrvClient in this manner will reset the transaction ID to 0.
                //     This will mean that, for the time being, all requests from a SrvClient will
                //     contain the transaction ID of 1. This is fine for now since SrvClient can
                //     only issue one request and receive one or more replies for it. In the
                //     future, SrvClient will also be multiplexed and this comment won't matter.
                SrvClient::new(io.into_inner())
            }
            _ => panic!("into_inner called before complete"),
        }
    }
}

enum SrvRequestState {
    Sending,
    Receiving(SrvStream, bool),
}

impl Stream for SrvReply {
    type Item = SrvWireMessage;
    type Error = SrvClientError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        loop {
            match self.state {
                SrvRequestState::Sending => {
                    let io = try_ready!(self.io.poll());
                    self.state = SrvRequestState::Receiving(io, false);
                    continue;
                }
                SrvRequestState::Receiving(_, true) => return Ok(Async::Ready(None)),
                SrvRequestState::Receiving(ref mut io, ref mut complete) => {
                    match try_ready!(io.poll()) {
                        Some(msg) => {
                            *complete = msg.is_complete();
                            return Ok(Async::Ready(Some(msg)));
                        }
                        None => return Err(SrvClientError::ConnectionClosed),
                    }
                }
            }
        }
    }
}
