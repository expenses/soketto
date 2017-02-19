//! A websocket base frame
use std::fmt;
use util;

/// Operation codes defined in [RFC6455](//! [rfc6455]: https://tools.ietf.org/html/rfc6455).
///
/// Taken from [ws-rs](https://github.com/housleyjk/ws-rs)
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum OpCode {
    /// Indicates a continuation frame of a fragmented message.
    Continue,
    /// Indicates a text data frame.
    Text,
    /// Indicates a binary data frame.
    Binary,
    /// Indicates a close control frame.
    Close,
    /// Indicates a ping control frame.
    Ping,
    /// Indicates a pong control frame.
    Pong,
    /// Indicates a reserved op code.
    Reserved,
    /// Indicates an invalid opcode was received.
    Bad,
}

impl OpCode {
    /// Is this a control opcode?
    pub fn is_control(&self) -> bool {
        match *self {
            OpCode::Close | OpCode::Ping | OpCode::Pong => true,
            _ => false,
        }
    }

    /// Is this opcode reserved or bad?
    pub fn is_invalid(&self) -> bool {
        match *self {
            OpCode::Reserved | OpCode::Bad => true,
            _ => false,
        }
    }
}

impl Default for OpCode {
    fn default() -> OpCode {
        OpCode::Close
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpCode::Continue => write!(f, "Continue"),
            OpCode::Text => write!(f, "Text"),
            OpCode::Binary => write!(f, "Binary"),
            OpCode::Close => write!(f, "Close"),
            OpCode::Ping => write!(f, "Ping"),
            OpCode::Pong => write!(f, "Pong"),
            OpCode::Reserved => write!(f, "Reserved"),
            OpCode::Bad => write!(f, "Bad"),
        }
    }
}

impl From<u8> for OpCode {
    fn from(val: u8) -> OpCode {
        match val {
            0 => OpCode::Continue,
            1 => OpCode::Text,
            2 => OpCode::Binary,
            8 => OpCode::Close,
            9 => OpCode::Ping,
            10 => OpCode::Pong,
            3 | 4 | 5 | 6 | 7 | 11 | 12 | 13 | 14 | 15 => OpCode::Reserved,
            _ => OpCode::Bad,
        }
    }
}

impl From<OpCode> for u8 {
    fn from(opcode: OpCode) -> u8 {
        match opcode {
            OpCode::Continue => 0,
            OpCode::Text => 1,
            OpCode::Binary => 2,
            OpCode::Close => 8,
            OpCode::Ping => 9,
            OpCode::Pong => 10,
            OpCode::Reserved | OpCode::Bad => 3,
        }
    }
}

/// A websocket base frame.
#[derive(Debug, Clone)]
pub struct Frame {
    /// The `fin` flag.
    fin: bool,
    /// The `rsv1` flag.
    rsv1: bool,
    /// The `rsv2` flag.
    rsv2: bool,
    /// The `rsv3` flag.
    rsv3: bool,
    /// The `opcode`
    opcode: OpCode,
    /// The `payload_length`
    payload_length: u64,
    /// The optional `extension_data`
    extension_data: Option<Vec<u8>>,
    /// The optional `application_data`
    application_data: Option<Vec<u8>>,
}

impl Frame {
    /// Get the `fin` flag.
    pub fn fin(&self) -> bool {
        self.fin
    }

    /// Set the `fin` flag.
    pub fn set_fin(&mut self, fin: bool) -> &mut Frame {
        self.fin = fin;
        self
    }

    /// Get the `rsv1` flag.
    pub fn rsv1(&self) -> bool {
        self.rsv1
    }

    /// Set the `rsv1` flag.
    pub fn set_rsv1(&mut self, rsv1: bool) -> &mut Frame {
        self.rsv1 = rsv1;
        self
    }

    /// Get the `rsv2` flag.
    pub fn rsv2(&self) -> bool {
        self.rsv2
    }

    /// Set the `rsv2` flag.
    pub fn set_rsv2(&mut self, rsv2: bool) -> &mut Frame {
        self.rsv2 = rsv2;
        self
    }

    /// Get the `rsv3` flag.
    pub fn rsv3(&self) -> bool {
        self.rsv3
    }

    /// Set the `rsv3` flag.
    pub fn set_rsv3(&mut self, rsv3: bool) -> &mut Frame {
        self.rsv3 = rsv3;
        self
    }

    /// Get the `opcode`.
    pub fn opcode(&self) -> OpCode {
        self.opcode
    }

    /// Set the `opcode`
    pub fn set_opcode(&mut self, opcode: OpCode) -> &mut Frame {
        self.opcode = opcode;
        self
    }

    /// Get the `payload_length`.
    pub fn payload_length(&self) -> u64 {
        self.payload_length
    }

    /// Set the `payload_length`
    pub fn set_payload_length(&mut self, payload_length: u64) -> &mut Frame {
        self.payload_length = payload_length;
        self
    }

    /// Get the `extension_data`.
    pub fn extension_data(&self) -> Option<&Vec<u8>> {
        if let Some(ref ed) = self.extension_data {
            Some(ed)
        } else {
            None
        }
    }

    /// Set the `extension_data`.
    pub fn set_extension_data(&mut self, extension_data: Option<Vec<u8>>) -> &mut Frame {
        self.extension_data = extension_data;
        self
    }

    /// Get the `application_data`
    pub fn application_data(&self) -> Option<&Vec<u8>> {
        if let Some(ref ad) = self.application_data {
            Some(ad)
        } else {
            None
        }
    }

    /// Set the `application_data`
    pub fn set_application_data(&mut self, application_data: Option<Vec<u8>>) -> &mut Frame {
        self.application_data = application_data;
        self
    }
}

impl Default for Frame {
    fn default() -> Frame {
        Frame {
            fin: true,
            rsv1: false,
            rsv2: false,
            rsv3: false,
            opcode: OpCode::Close,
            payload_length: 0,
            extension_data: None,
            application_data: None,
        }
    }
}

impl fmt::Display for Frame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Frame {{"));
        try!(write!(f, "\n\tfin: {}", self.fin));
        try!(write!(f, "\n\trsv1: {}", self.rsv1));
        try!(write!(f, "\n\trsv2: {}", self.rsv2));
        try!(write!(f, "\n\trsv3 {}", self.rsv3));
        try!(write!(f, "\n\trsv3 {}", self.rsv3));
        try!(write!(f, "\n\topcode {}", self.opcode));
        try!(write!(f, "\n\tpayload_length {}", self.payload_length));
        if let Some(ref ext_data) = self.extension_data {
            let len = ext_data.len();
            if len <= 256 {
                try!(write!(f, "\n\textension_data:\n"));
                try!(write!(f, "{}\n", util::hex_header()));
                try!(write!(f, "{}", util::as_hex(ext_data)));
            } else {
                try!(write!(f, "\n\textension_data: [ {} bytes ]", len));
            }
        }
        if let Some(ref app_data) = self.application_data {
            let len = app_data.len();
            if len <= 256 {
                try!(write!(f, "\n\tapplication_data:\n"));
                try!(write!(f, "{}\n", util::hex_header()));
                try!(write!(f, "{}", util::as_hex(app_data)));
            } else {
                try!(write!(f, "\n\tapplication_data: [ {} bytes ]", len));
            }
        }
        writeln!(f, "}}")
    }
}