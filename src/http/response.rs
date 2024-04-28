use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use std::io::{Result as IoResult, Write};
use std::time::SystemTime;

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new(status_code: StatusCode) -> Self {
        Response {
            status_code,
            body: None,
            headers: HashMap::new(),
        }
    }
    pub fn set_body(&mut self, body: String, content_type: &str) {
        let etag = generate_etag(&body);
        self.headers.insert("Etag".to_string(), etag);

        self.body = Some(body);
        self.headers
            .insert("Content-Type".to_string(), content_type.to_string());
        self.headers.insert(
            "Content-Length".to_string(),
            self.body.as_ref().unwrap().len().to_string(),
        );
        self.headers
            .insert("Date".to_string(), format!("{}", http_date()));
        self.headers.insert(
            "Cache-Control".to_string(),
            "public, max-age=0, must-revalidate".to_string(),
        );
    }

    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
        write!(
            stream,
            "HTTP/1.1 {} {}\r\n",
            self.status_code,
            self.status_code.reason_phrase()
        )?;
        for (key, value) in &self.headers {
            write!(stream, "{}: {}\r\n", key, value)?;
        }
        write!(
            stream,
            "\r\n{}",
            self.body.as_ref().unwrap_or(&"".to_string())
        )
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter) -> Result {
        writeln!(
            f,
            "HTTP/1.1 {} {}",
            self.status_code,
            self.status_code.reason_phrase()
        )?;
        for (key, value) in &self.headers {
            writeln!(f, "{}: {}", key, value)?;
        }
        if let Some(ref body) = self.body {
            writeln!(f, "\r\n{}", body)?;
        }
        Ok(())
    }
}

fn http_date() -> String {
    let now = SystemTime::now();
    let datetime: chrono::DateTime<chrono::Utc> = now.into();
    datetime.format("%a, %d %b %Y %H:%M:%S GMT").to_string()
}

fn generate_etag(body: &str) -> String {
    let digest = md5::compute(body);
    format!("\"{:?}\"", digest)
}
