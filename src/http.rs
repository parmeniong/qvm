use crate::util::{error, ErrorType};
use std::collections::HashMap;
use std::io::{Write, Read};
use std::net::TcpStream;
use http_parser::{HttpParser, HttpParserCallback, HttpParserType, ParseAction};
use std::str::from_utf8;
use std::process::exit;

pub enum HttpMethod {
    Get
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            HttpMethod::Get => formatter.write_str("GET")
        }
    }
}

fn to_status_code(reason: &str) -> u16 {
    match reason {
        "Continue" => 100,
        "Switching Protocols" => 101,
        "Processing" => 102,
        "Early Hints" => 103,
        "OK" => 200,
        "Created" => 201,
        "Accepted" => 202,
        "Non-Authoritative Information" => 203,
        "No Content" => 204,
        "Reset Content" => 205,
        "Partial Content" => 206,
        "Multi-Status" => 207,
        "Already Reported" => 208,
        "IM used" => 226,
        "Multiple Choices" => 300,
        "Moved Permanently" => 301,
        "Found" => 302,
        "See Other" => 303,
        "Not Modified" => 304,
        "Temporary Redirect" => 307,
        "Permanent Redirect" => 308,
        "Bad Request" => 400,
        "Unauthorized" => 401,
        "Payment Required" => 402,
        "Forbidden" => 403,
        "Not Found" => 404,
        "Method Not Allowed" => 405,
        "Not Acceptable" => 406,
        "Proxy Authentication Required" => 407,
        "Request Timeout" => 408,
        "Conflict" => 409,
        "Gone" => 410,
        "Length Required" => 411,
        "Precondition Failed" => 412,
        "Payload Too Large" => 413,
        "URI Too Long" => 414,
        "Unsupported Media Type" => 415,
        "Range Not Satisfiable" => 416,
        "Expectation Failed" => 417,
        "I'm a teapot" => 418,
        "Misdirected Request" => 421,
        "Unprocessable Content" => 422,
        "Locked" => 423,
        "Failed Dependency" => 424,
        "Too Early" => 425,
        "Upgrade Required" => 426,
        "Precondition Required" => 428,
        "Too Many Requests" => 429,
        "Request Header Fields Too Large" => 431,
        "Unavailable For Legal Reasons" => 451,
        "Internal Server Error" => 500,
        "Not Implemented" => 501,
        "Bad Gateway" => 502,
        "Service Unavailable" => 503,
        "Gateway Timeout" => 504,
        "HTTP Version Not Supported" => 505,
        "Variant Also Negotiates" => 506,
        "Insufficient Storage" => 507,
        "Loop Detected" => 508,
        "Not Extended" => 510,
        "Network Authentication Required" => 511,
        _ => {
            error(ErrorType::HttpError, format!("Received invalid reason phrase: {}", reason));
            exit(1);
        }
    }
}

pub struct HttpResponse {
    pub status_code: u16,
    pub reason: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    current_header: String
}

impl HttpParserCallback for HttpResponse {
    fn on_status(&mut self, _parser: &mut HttpParser, data: &[u8]) -> http_parser::CallbackResult {
        let reason = from_utf8(data).unwrap();
        self.status_code = to_status_code(reason);
        self.reason = reason.to_string();
        Ok(ParseAction::None)
    }
    
    fn on_header_field(&mut self, _parser: &mut HttpParser, data: &[u8]) -> http_parser::CallbackResult {
        self.current_header = from_utf8(data).unwrap().to_string();
        Ok(ParseAction::None)
    }

    fn on_header_value(&mut self, _parser: &mut HttpParser, data: &[u8]) -> http_parser::CallbackResult {
        self.headers.insert(self.current_header.clone(), from_utf8(data).unwrap().to_string());
        Ok(ParseAction::None)
    }
    
    fn on_body(&mut self, _parser: &mut HttpParser, data: &[u8]) -> http_parser::CallbackResult {
        self.body = from_utf8(data).unwrap().to_string();
        Ok(ParseAction::None)
    }
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        HttpResponse {
            status_code: 0,
            reason: String::new(),
            headers: HashMap::new(),
            body: String::new(),
            current_header: String::new()
        }
    }
}

pub struct HttpRequest {
    method: HttpMethod,
    headers: HashMap<String, String>,
    body: String
}

impl HttpRequest {
    pub fn new(method: HttpMethod, body: String) -> HttpRequest {
        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Content-Length".to_string(), body.len().to_string());
        
        HttpRequest {
            method,
            headers,
            body
        }
    }

    pub fn add_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_string(), value.to_string());
    }
    
    pub fn send(&self, url: String) -> HttpResponse {
        let mut stream: TcpStream;
        
        if let Ok(result) = TcpStream::connect(url) {
            stream = result;
        } else {
            error(ErrorType::NetworkError, "Failed to connect to server");
            exit(1);
        }

        if let Err(_) = stream.write_all(self.to_string().as_bytes()) {
            error(ErrorType::NetworkError, "Failed to send request to server");
            exit(1);
        }

        let mut response_text = String::new();
        if let Err(_) = stream.read_to_string(&mut response_text) {
            error(ErrorType::NetworkError, "Failed to read response from server");
        }

        let mut parser = HttpParser::new(HttpParserType::Response);

        let mut response = HttpResponse::new();

        parser.execute(&mut response, response_text.as_bytes());

        response
    }

    fn to_string(&self) -> String {
        let request_line = format!("{} {} HTTP/1.1", self.method, "/");

        let mut headers = String::new();
        for key in self.headers.keys() {
            headers.push_str(format!("{}: {}\r\n", key, self.headers.get(key).unwrap()).as_str());
        }

        let request = format!("{}\r\n{}\r\n{}", request_line, headers, self.body);

        request
    }
}