use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;

pub struct Request {
        path: String,
        query_string: Option<String>,
        method: Method, //use an enum for this, since there are only a limited number of methods
    }

impl Request {
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl Error for ParseError {
    
}