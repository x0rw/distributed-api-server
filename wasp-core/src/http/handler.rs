use httparse;
use std::collections::HashMap;
use thiserror;
#[derive(Debug, Clone, PartialEq)]
pub struct HttpRequest<'a> {
    pub method: HttpMethod,
    pub uri: UriComponents<'a>,
    pub version: HttpVersion,
    pub headers: HeaderMap<'a>,
    pub body: &'a [u8],
}
#[derive(Debug, Clone, PartialEq, Default)]
pub struct UriComponents<'a> {
    pub path: &'a str,
    pub query: HashMap<&'a str, &'a str>,
    pub fragment: Option<&'a str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpVersion {
    HTTP1_0,
    HTTP1_1,
    HTTP2,
    HTTP3,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct HeaderMap<'a>(HashMap<&'a str, Vec<&'a str>>);

/// Parsing bytes into `HttpRequest`
/// Zero-copy(?)
impl<'a> HttpRequest<'a> {
    pub fn parse(raw: &'a [u8]) -> Result<Self, ParseError> {
        let mut headers = [httparse::EMPTY_HEADER; 32];
        let mut req = httparse::Request::new(&mut headers);
        let status = req.parse(raw)?;

        if !status.is_complete() {
            return Err(ParseError::Incomplete);
        }
        let method = HttpMethod::try_from(req.method.unwrap())?;
        let uri = UriComponents::parse(req.path.unwrap())?;
        let version = match req.version.unwrap() {
            0 => HttpVersion::HTTP1_0,
            1 => HttpVersion::HTTP1_1,
            _ => return Err(ParseError::UnsupportedVersion),
        };
        let headers = HeaderMap::from_headers(req.headers)?;
        let body_start = status.unwrap();
        let body = &raw[body_start..];
        Ok(Self {
            method,
            uri,
            version,
            headers,
            body,
        })
    }
    pub fn content_length(&self) -> Option<usize> {
        self.headers
            .get("Content-lenght")
            .and_then(|v| v.first())
            .and_then(|s| s.parse().ok())
    }

    pub fn content_type(&self) -> Option<&str> {
        self.headers
            .get("Content-type")
            .and_then(|v| v.first())
            .map(|s| s.split(';').next().unwrap_or(s))
    }
}

/// parses a URI into path and queries
/// fragment is not required in the api context
impl<'a> UriComponents<'a> {
    pub fn parse(uri: &'a str) -> Result<Self, ParseError> {
        let (rest, fragment) = uri.split_once('#').unwrap_or((uri, ""));
        let (path, query) = rest.split_once('?').unwrap_or((uri, ""));
        let query_params = query
            .split('&')
            .filter_map(|pair| {
                let (k, v) = pair.split_once('=')?;
                Some((k, v))
            })
            .collect();
        Ok(Self {
            path,
            query: query_params,
            fragment: if fragment.is_empty() {
                None
            } else {
                Some(fragment)
            },
        })
    }
}

/// parses the header into a hashmap for o(1) access
///
impl<'a> HeaderMap<'a> {
    fn from_headers(headers: &[httparse::Header<'a>]) -> Result<Self, ParseError> {
        //pre-allocation
        let mut map = HashMap::with_capacity(headers.len());
        for header in headers {
            let key = std::str::from_utf8(header.name.as_bytes())?;
            let value = std::str::from_utf8(header.value)?;
            map.entry(key).or_insert_with(Vec::new).push(value);
        }

        Ok(HeaderMap(map))
    }

    /// Get header values case-insensitively
    pub fn get(&self, key: &str) -> Option<&Vec<&'a str>> {
        self.0.get(key.to_ascii_lowercase().as_str())
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    CONNECT,
    TRACE,
}
impl From<HttpMethod> for &'static str {
    fn from(method: HttpMethod) -> &'static str {
        match method {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::CONNECT => "CONNECT",
            HttpMethod::TRACE => "TRACE",
        }
    }
}
impl TryFrom<&str> for HttpMethod {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PUT" => Ok(Self::PUT),
            "DELETE" => Ok(Self::DELETE),
            "HEAD" => Ok(Self::HEAD),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PATCH" => Ok(Self::PATCH),
            "CONNECT" => Ok(Self::CONNECT),
            "TRACE" => Ok(Self::TRACE),
            _ => Err(format!("Invalid HTTP method: {}", value)),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid URI format")]
    InvalidUri,
    #[error("Unsupported HTTP version")]
    UnsupportedVersion,
    #[error("Incomplete request")]
    Incomplete,
    #[error("Invalid UTF-8 sequence")]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error("Parse error")]
    HttpParse(#[from] httparse::Error),
    #[error("Invalid HTTP method: {0}")]
    InvalidMethod(String),
}
impl From<String> for ParseError {
    fn from(err: String) -> Self {
        ParseError::InvalidMethod(err)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn parse_simple_get_request() {
        let raw = b"GET / HTTP/1.1\r\n\r\n";
        let req = HttpRequest::parse(raw).unwrap();

        assert_eq!(req.method, HttpMethod::GET);
        assert_eq!(req.uri.path, "/");
        assert_eq!(req.version, HttpVersion::HTTP1_1);
        assert!(req.headers.0.is_empty());
        assert!(req.body.is_empty());
    }

    #[test]
    fn parse_post_request_with_headers_and_body() {
        let raw = BytesMut::from(
            "POST /api/data HTTP/1.1\r\n\
             Content-Type: application/json\r\n\
             Content-Length: 17\r\n\
             \r\n\
             {\"key\":\"value\"}",
        )
        .freeze();

        let req = HttpRequest::parse(&raw).unwrap();

        assert_eq!(req.method, HttpMethod::POST);
        assert_eq!(req.uri.path, "/api/data");
        assert_eq!(
            req.headers.get("content-type").unwrap()[0],
            "application/json"
        );
        assert_eq!(req.content_length(), Some(17));
        assert_eq!(req.body, b"{\"key\":\"value\"}");
    }

    #[test]
    fn parse_request_with_query_params() {
        let raw = b"GET /search?q=rust&page=2 HTTP/1.1\r\n\r\n";
        let req = HttpRequest::parse(raw).unwrap();

        assert_eq!(req.uri.path, "/search");
        assert_eq!(req.uri.query.get("q"), Some(&"rust"));
        assert_eq!(req.uri.query.get("page"), Some(&"2"));
    }

    #[test]
    fn parse_request_with_fragment() {
        let raw = b"GET /docs#section HTTP/1.1\r\n\r\n";
        let req = HttpRequest::parse(raw).unwrap();

        assert_eq!(req.uri.path, "/docs");
        assert_eq!(req.uri.fragment, Some("section"));
    }

    #[test]
    fn parse_invalid_method() {
        let raw = b"INVALID / HTTP/1.1\r\n\r\n";
        let result = HttpRequest::parse(raw);

        assert!(matches!(result, Err(ParseError::InvalidMethod(_))));
    }

    #[test]
    fn parse_unsupported_version() {
        let raw = b"GET / HTTP/2.0\r\n\r\n";
        let result = HttpRequest::parse(raw);

        assert!(matches!(result, Err(ParseError::UnsupportedVersion)));
    }

    #[test]
    fn parse_incomplete_request() {
        let raw = b"GET / HTTP/1.1\r\n";
        let result = HttpRequest::parse(raw);

        assert!(matches!(result, Err(ParseError::Incomplete)));
    }

    #[test]
    fn parse_multiple_headers_same_name() {
        let raw = b"GET / HTTP/1.1\r\n\
                   X-Header: first\r\n\
                   X-Header: second\r\n\
                   \r\n";

        let req = HttpRequest::parse(raw).unwrap();
        let values = req.headers.get("x-header").unwrap();

        assert_eq!(values.len(), 2);
        assert_eq!(values[0], "first");
        assert_eq!(values[1], "second");
    }

    #[test]
    fn content_type_handling() {
        let raw = b"GET / HTTP/1.1\r\n\
                   Content-Type: text/html; charset=utf-8\r\n\
                   \r\n";

        let req = HttpRequest::parse(raw).unwrap();
        assert_eq!(req.content_type(), Some("text/html"));
    }

    #[test]
    fn large_request_handling() {
        let mut raw = BytesMut::with_capacity(1024);
        raw.extend_from_slice(b"POST /data HTTP/1.1\r\n");
        raw.extend_from_slice(b"Content-Length: 1024\r\n\r\n");
        raw.extend(vec![b'X'; 1024]);
        let raw = raw.freeze();

        let req = HttpRequest::parse(&raw).unwrap();
        assert_eq!(req.body.len(), 1024);
    }
}
