
use httplineparser::HttpLineParsers;
use httpreader::HttpReader;

pub struct Header(pub String);

pub struct HeaderParser {
}

impl Header {
    pub fn from_string(header: String) -> Option<Header> {
        if HttpLineParsers::verify_header_line(&header) {
            return Some(Header(header));
        } else {
            return None;
        }
    }

    pub fn from_str(header: &'static str) -> Option<Header> {
        return Header::from_string(String::from(header));
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


pub fn headers_to_body_len(headers: &Vec<Header>) -> usize {
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


pub fn prefix_and_headers_to_io_string(prefix: &String, headers: &Vec<Header>) -> String {
    let mut req = String::new();
    req.push_str(prefix);

    for &Header(ref header) in headers {
        req.push_str(header.as_str());
    }

    req.push_str("\r\n");

    return req;
}

#[test]
fn test_header_from_string() {
    let val = String::from("Valid: Header\r\n");
    let header = Header::from_string(val);
    assert!(header.is_some());
}

#[test]
pub fn test_headers_to_body_len_explicit() {
    let headers = vec![Header::from_str("Foo: bar\r\n").unwrap(),
                       Header::from_str("Content-Length: 17\r\n").unwrap()];

    assert_eq!(headers_to_body_len(&headers), 17);

}


#[test]
pub fn test_headers_to_body_len_implicit() {
    let headers = vec![
        Header::from_str("Foo: bar\r\n").unwrap(),
    ];

    assert_eq!(headers_to_body_len(&headers), 0);

}


#[test]
fn test_prefix_and_headers_to_io_string() {
    let prefix = String::from("HTTP/1.0 200 Ok\r\n");
    let headers = vec![Header::from_str("Foo: bar\r\n").unwrap(),
                       Header::from_str("Baz: tux\r\n").unwrap()];

    let iostr = prefix_and_headers_to_io_string(&prefix, &headers);

    assert_eq!(iostr,
               String::from("HTTP/1.0 200 Ok\r\nFoo: bar\r\nBaz: tux\r\n\r\n"));
}
