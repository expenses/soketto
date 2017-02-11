use httparse::{EMPTY_HEADER, Request};
use std::collections::HashMap;
use std::fmt;
use std::io;
use tokio_core::io::EasyBuf;
use util;

#[derive(Debug, Clone)]
pub struct HandshakeFrame {
    method: String,
    path: String,
    version: u8,
    /// Host header (Required)
    host: Option<String>,
    /// Upgrade header (Required)
    upgrade: Option<String>,
    /// Connection header (Required)
    conn: Option<String>,
    /// Sec-WebSocket-Key header (Required)
    ws_key: Option<String>,
    /// Sec-WebSocket-Version header (Required)
    ws_version: Option<String>,
    /// Origin header (Optional)
    origin: Option<String>,
    /// Sec-WebSocket-Protocol header (Optional)
    protocol: Option<String>,
    /// Sec-WebSocket-Extensions header (Optional)
    extensions: Option<String>,
    /// Any other remaining headers.
    others: HashMap<String, String>,
}

// TODO: Convert to return result with reason code.
impl HandshakeFrame {
    fn validate(&mut self, handshake: &HandshakeFrame) -> bool {
        if handshake.method != "GET" {
            return false;
        }

        if handshake.version != 1 {
            return false;
        }

        // TODO: Host Validation

        if let Some(ref val) = handshake.upgrade {
            if val.to_lowercase() != "websocket" {
                return false;
            }
        } else {
            return false;
        }

        if let Some(ref val) = handshake.conn {
            if val.to_lowercase() != "upgrade" {
                return false;
            }
        } else {
            return false;
        }

        if handshake.ws_key.is_none() {
            return false;
        }

        if let Some(ref val) = handshake.ws_version {
            if val != "13" {
                return false;
            }
        } else {
            return false;
        }

        return true;
    }

    pub fn decode(&mut self, buf: &mut EasyBuf) -> Result<Option<HandshakeFrame>, io::Error> {
        let len = buf.len();
        let drained = buf.drain_to(len);
        let req_bytes = drained.as_slice();
        let mut headers = [EMPTY_HEADER; 32];
        let mut req = Request::new(&mut headers);
        let mut handshake_frame: HandshakeFrame = Default::default();

        if let Ok(res) = req.parse(req_bytes) {
            if res.is_complete() {
                if let Some(method) = req.method {
                    handshake_frame.method = method.to_string();
                }

                if let Some(path) = req.path {
                    handshake_frame.path = path.to_string();
                }

                if let Some(version) = req.version {
                    handshake_frame.version = version;
                }

                let mut headers = HashMap::new();
                for header in req.headers {
                    // I'm intentionally igonring duplicate headers here.  Is that ok?
                    let key = header.name.to_string();
                    let val = String::from_utf8_lossy(header.value).into_owned();
                    headers.insert(key, val);
                }

                // Required Headers
                handshake_frame.host = headers.remove("Host");
                handshake_frame.upgrade = headers.remove("Upgrade");
                handshake_frame.conn = headers.remove("Connection");
                handshake_frame.ws_key = headers.remove("Sec-WebSocket-Key");
                handshake_frame.ws_version = headers.remove("Sec-WebSocket-Version");

                // Optional headers
                handshake_frame.origin = headers.remove("Origin");
                handshake_frame.protocol = headers.remove("Sec-WebSocket-Protocol");
                handshake_frame.extensions = headers.remove("Sec-WebSocket-Extensions");

                if headers.len() > 0 {
                    handshake_frame.others = headers;
                }

                if self.validate(&handshake_frame) {
                    Ok(Some(handshake_frame))
                } else {
                    return Err(util::other("invalid handshake request"));
                }
            } else {
                return Err(util::other("partial client request received"));
            }
        } else {
            return Err(util::other("unable to parse client request"));
        }
    }

    pub fn to_byte_buf(&self, _buf: &mut Vec<u8>) -> Result<(), io::Error> {
        Ok(())
    }
}

impl Default for HandshakeFrame {
    fn default() -> HandshakeFrame {
        HandshakeFrame {
            method: String::new(),
            path: String::new(),
            version: 0,
            host: None,
            upgrade: None,
            conn: None,
            ws_key: None,
            ws_version: None,
            origin: None,
            protocol: None,
            extensions: None,
            others: HashMap::new(),
        }
    }
}

impl fmt::Display for HandshakeFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(writeln!(f, "HandshakeFrame {{"));
        try!(writeln!(f, "\tmethod: {}", self.method));
        try!(writeln!(f, "\tpath: {}", self.path));
        try!(writeln!(f, "\tversion: {}", self.version));
        try!(writeln!(f, "\thost: {:?}", self.host));
        try!(writeln!(f, "\tupgrade: {:?}", self.upgrade));
        try!(writeln!(f, "\tconn: {:?}", self.conn));
        try!(writeln!(f, "\tws_key: {:?}", self.ws_key));
        try!(writeln!(f, "\tws_version: {:?}", self.ws_version));
        write!(f, "}}")
    }
}