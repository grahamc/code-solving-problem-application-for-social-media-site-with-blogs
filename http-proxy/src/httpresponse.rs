


use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;
use httpreader::HttpReader;
use httpheader::{Header, HeaderParser};
use httplineparser::HttpLineParsers;
use httperrors::HttpError;

use httpheader::{headers_to_body_len, prefix_and_headers_to_io_string};


pub struct HttpResponse {
    status_line: String,
    headers: Vec<Header>,
}

pub struct HttpResponseParser<'a> {
    stream: HttpReader<'a>,
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
