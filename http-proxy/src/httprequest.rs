
use std::io::BufReader;
use std::net::TcpStream;
use httperrors::HttpError;
use httpstate::HttpReader;
use httplineparser::HttpLineParsers;
use httpheader::{Header,HeaderParser,headers_to_body_len, prefix_and_headers_to_io_string};

pub struct HttpRequest {
    request_line: String,
    headers: Vec<Header>,
}


pub struct HttpRequestParser<'a> {
    stream: HttpReader<'a>,
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
