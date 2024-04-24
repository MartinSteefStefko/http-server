use super::method::{Method, MethodError};
use super::QueryString;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &'buf str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        // Lifetime specifier added
        self.query_string.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request_str = str::from_utf8(buf)?;
        let (method, path, query_string) = parse_request_line(request_str)?;

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn parse_request_line<'a>(
    request: &'a str,
) -> Result<(Method, &'a str, Option<QueryString>), ParseError> {
    let (method, rest) = parse_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (raw_path, rest) = parse_next_word(rest).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) = parse_next_word(rest).ok_or(ParseError::InvalidRequest)?;

    validate_protocol(protocol)?;

    let method: Method = method.parse()?;
    let (path, query_string) = if let Some(index) = raw_path.find('?') {
        let qs = &raw_path[index + 1..];
        (&raw_path[..index], Some(QueryString::from(qs)))
    } else {
        (raw_path, None)
    };

    Ok((method, path, query_string))
}

fn parse_next_word(request: &str) -> Option<(&str, &str)> {
    request
        .find(|c: char| c == ' ' || c == '\r')
        .map(|index| (request[..index].trim(), &request[index + 1..]))
}

fn validate_protocol(protocol: &str) -> Result<(), ParseError> {
    match protocol.split_once('/') {
        Some(("HTTP", "1.1")) => Ok(()),
        Some((_, "1.1")) => Err(ParseError::InvalidProtocol),
        Some(("HTTP", _)) => Err(ParseError::UnsupportedVersion),
        Some(_) => Err(ParseError::InvalidProtocol),
        None => Err(ParseError::InvalidProtocol),
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
    UnsupportedVersion,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
            Self::UnsupportedVersion => "HTTP Version Not Supported",
        }
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
