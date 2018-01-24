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

use hcore;
use protobuf::ProtobufEnum;

pub use protocols::ctl::{ErrCode, NetErr, NetOk};

pub type NetResult<T> = Result<T, NetErr>;

impl error::Error for NetErr {
    fn description(&self) -> &str {
        match self.get_code() {
            ErrCode::Internal => "Internal error",
            ErrCode::Io => "IO error",
            ErrCode::NotFound => "Entity not found",
            ErrCode::Unauthorized => "Client failed authorization with server",
            ErrCode::Conflict => "Entity exists or is unable to update with given parameters",
        }
    }
}

impl fmt::Display for NetErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Err: {}, Msg: {}]",
            self.get_code().value(),
            self.get_msg()
        )
    }
}

impl From<::error::SupError> for NetErr {
    fn from(other: ::error::SupError) -> Self {
        err(ErrCode::Internal, other)
    }
}

impl From<io::Error> for NetErr {
    fn from(other: io::Error) -> Self {
        err(ErrCode::Io, other)
    }
}

impl From<hcore::Error> for NetErr {
    fn from(other: hcore::Error) -> Self {
        err(ErrCode::Internal, other)
    }
}

pub fn err<T>(code: ErrCode, msg: T) -> NetErr
where
    T: fmt::Display,
{
    let mut err = NetErr::new();
    err.set_code(code);
    err.set_msg(format!("{}", msg));
    err
}
