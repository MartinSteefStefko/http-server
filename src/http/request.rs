use super::method::{Method, MethodError};
use super::QueryString; // Assuming QueryString is a proper struct handling query parameters
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str::{self, Utf8Error};

#[derive(Debug)]
pub struct Request {
    method: Method,
    path: String,
    query_string: Option<QueryString>, // Use QueryString directly without a lifetime
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl<'buf> Request {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref() // Correctly reference the inner QueryString
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        let request_str = str::from_utf8(buf)?;
        let (method, path, query_string) = parse_request_line(request_str)?;

        Ok(Self {
            method,
            path: path.to_string(),
            query_string: query_string.map(|qs| QueryString::from(qs)),
            headers: HashMap::new(),
            body: Vec::new(),
        })
    }
}

fn parse_request_line<'a>(
    request: &'a str,
) -> Result<(Method, &'a str, Option<&'a str>), ParseError> {
    let (method, rest) = parse_next_word(request).ok_or(ParseError::InvalidRequest)?;
    let (raw_path, rest) = parse_next_word(rest).ok_or(ParseError::InvalidRequest)?;
    let (protocol, _) = parse_next_word(rest).ok_or(ParseError::InvalidRequest)?;

    validate_protocol(protocol)?;

    let method: Method = method.parse()?;
    let (path, query_string) = separate_path_and_query(raw_path);

    Ok((method, path, query_string))
}

fn separate_path_and_query(raw_path: &str) -> (&str, Option<&str>) {
    if let Some(index) = raw_path.find('?') {
        let (path, qs) = raw_path.split_at(index);
        (path, Some(&qs[1..])) // Skip '?' for query string
    } else {
        (raw_path, None)
    }
}

fn parse_next_word(request: &str) -> Option<(&str, &str)> {
    request
        .find(|c: char| c == ' ' || c == '\r')
        .map(|index| (request[..index].trim(), &request[index + 1..]))
}

fn validate_protocol(protocol: &str) -> Result<(), ParseError> {
    match protocol.split_once('/') {
        Some(("HTTP", "1.1")) => Ok(()),
        _ => Err(ParseError::InvalidProtocol),
    }
}

impl<'buf> Default for Request {
    fn default() -> Self {
        Request {
            method: Method::GET,
            path: "/".to_string(),
            query_string: None::<QueryString>, // Provide type annotation for None
            headers: HashMap::new(),
            body: Vec::new(),
        }
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
