
use httperrors::HttpError;
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;
use httplineparser::HttpLineParsers;

pub struct Header(String);

pub struct HttpRequest {
    request_line: String,
    headers: Vec<Header>,
}

pub struct HttpResponse {
    status_line: String,
    headers: Vec<Header>,
}

pub struct HttpRequestParser<'a> {
    stream: HttpReader<'a>,
}

pub struct HttpResponseParser<'a> {
    stream: HttpReader<'a>,
}

pub struct HeaderParser {
}

pub struct HttpReader<'a> {
    // <Read> (trait) says read(&self) so mutable borrow
    stream: &'a mut BufReader<TcpStream>,
}

impl Header {
    pub fn from_string(header: String) -> Option<Header> {
        if HttpLineParsers::verify_header_line(&header) {
            return Some(Header(header));
        } else {
            return None;
        }
    }
}

fn prefix_and_headers_to_io_string(prefix: &String, headers: &Vec<Header>) -> String {
    let mut req = String::new();
    req.push_str(prefix);

    for &Header(ref header) in headers {
        req.push_str(header.as_str());
    }

    req.push_str("\r\n");

    return req;
}

fn headers_to_body_len(headers: &Vec<Header>) -> usize {
    for &Header(ref header) in headers {
        match HttpLineParsers::extract_content_length(header) {
            Some(len) => {
                return len;
            }
            None => {}
        }
    }

    return 0;
}

impl HttpRequest {
    pub fn add_header(&mut self, header: Header) {
        self.headers.push(header);
    }

    pub fn body_length(self) -> usize {
        return headers_to_body_len(&self.headers);
    }

    pub fn as_string(&self) -> String {
        return prefix_and_headers_to_io_string(&self.request_line, &self.headers);
    }
}

impl HttpResponse {
    pub fn from_error(err: HttpError) -> HttpResponse {
        return HttpResponse {
            status_line: format!("HTTP/1.0 {} {}\r\n", &err.code, &err.message),
            headers: vec![Header(String::from("Content-Length: 0\r\n"))],
        };
    }

    pub fn add_header(&mut self, header: Header) {
        self.headers.push(header);
    }

    pub fn body_length(self) -> usize {
        return headers_to_body_len(&self.headers);
    }

    pub fn as_string(&self) -> String {
        return prefix_and_headers_to_io_string(&self.status_line, &self.headers);
    }
}

impl<'a> HttpRequestParser<'a> {
    pub fn new(stream: &'a mut BufReader<TcpStream>) -> HttpRequestParser<'a> {
        HttpRequestParser { stream: HttpReader::new(stream) }
    }

    pub fn parse(&mut self) -> Result<HttpRequest, HttpError> {
        match self.parse_unwrapped() {
            Ok(req) => return Ok(req),
            Err(e) => {
                return Err(HttpError::malformed_request(e));
            }
        }
    }

    fn parse_unwrapped(&mut self) -> Result<HttpRequest, &'static str> {
        let request_line = try!(self.read_request_line());
        let headers = try!(self.read_headers());

        return Ok(HttpRequest {
            request_line: request_line,
            headers: headers,
        });
    }

    fn read_request_line(&mut self) -> Result<String, &'static str> {
        let line = try!(self.stream.read_line());

        if HttpLineParsers::verify_request_line(&line) {
            return Ok(line);
        } else {
            return Err("malformed request headers");
        }
    }

    fn read_headers(&mut self) -> Result<Vec<Header>, &'static str> {
        return HeaderParser::read_headers(&mut self.stream);
    }
}

impl<'a> HttpResponseParser<'a> {
    pub fn new(stream: &'a mut BufReader<TcpStream>) -> HttpResponseParser<'a> {
        return HttpResponseParser { stream: HttpReader::new(stream) };
    }


    pub fn parse(&mut self) -> Result<HttpResponse, HttpError> {
        match self.parse_unwrapped() {
            Ok(req) => return Ok(req),
            Err(e) => {
                return Err(HttpError::malformed_reply(e));
            }
        }
    }

    pub fn parse_unwrapped(&mut self) -> Result<HttpResponse, &'static str> {
        let status_line = try!(self.read_status_line());
        let headers = try!(self.read_headers());

        return Ok(HttpResponse {
            status_line: status_line,
            headers: headers,
        });
    }

    pub fn read_status_line(&mut self) -> Result<String, &'static str> {
        let status_line: String = try!(self.stream.read_line());

        if HttpLineParsers::verify_status_line(&status_line) {
            return Ok(status_line);
        } else {
            return Err("malformed status line");
        }
    }

    pub fn read_headers(&mut self) -> Result<Vec<Header>, &'static str> {
        return HeaderParser::read_headers(&mut self.stream);
    }
}

impl HeaderParser {
    pub fn read_headers(mut stream: &mut HttpReader) -> Result<Vec<Header>, &'static str> {
        let mut headers = vec![];

        while let Some(header) = try!(HeaderParser::read_header(&mut stream)) {
            headers.push(header)
        }

        return Ok(headers);
    }

    fn read_header(stream: &mut HttpReader) -> Result<Option<Header>, &'static str> {
        let header = try!(stream.read_line());

        if HttpLineParsers::verify_header_line(&header) {
            return Ok(Some(Header(header)));
        } else if HttpLineParsers::verify_end_of_headers(&header) {
            return Ok(None);
        } else {
            return Err("Invalid header");
        }
    }
}

impl<'a> HttpReader<'a> {
    pub fn new(stream: &'a mut BufReader<TcpStream>) -> HttpReader<'a> {
        return HttpReader { stream: stream };
    }

    fn read_line(&mut self) -> Result<String, &'static str> {
        let mut line = String::new();

        return match self.stream.read_line(&mut line) {
            Ok(0) => Err("Empty read"),
            Ok(_) => Ok(line),
            Err(_) => Err("Failed read"),
        };
    }
}
