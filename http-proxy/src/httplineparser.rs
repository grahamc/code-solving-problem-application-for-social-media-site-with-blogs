use regex::Regex;

pub struct HttpLineParsers {
}

impl HttpLineParsers {
    pub fn verify_request_line(text: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^([a-zA-Z0-9]*)\s+([^\s]*)\s+HTTP/\d.\d\r?\n$"
            ).unwrap();
        }
        return RE.is_match(text);
    }

    pub fn verify_status_line(text: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^HTTP/\d.\d\s+\d{3}\s+([\w\s]+)\r?\n$"
            ).unwrap();
        }
        return RE.is_match(text);
    }


    pub fn verify_header_line(text: &str) -> bool {
        // Note: RFC 822 says headers can be multi-line, but according
        // to my scientific survey at
        // https://twitter.com/grhmc/status/742152080261947393 nobody
        // cares, as such I'm going to pretend a header's value
        // can be literally anything but a newline.
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^([!#$%&'*+-.^_`|0-9A-Za-z]+):([^\r\n]*)\r?\n$"
            ).unwrap();
        }

        return RE.is_match(text);
    }

    pub fn verify_end_of_headers(text: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^\r?\n$").unwrap();
        }

        return RE.is_match(text);
    }

    pub fn extract_content_length(text: &str) -> Option<usize> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^(?i)content-length:\s*(\d+)\s*\r?\n$"
            ).unwrap();
        }

        match RE.captures(text) {
            Some(capture) => {
                match capture.at(1) {
                    Some(len) => {
                        return match len.parse::<usize>() {
                            Ok(len) => Some(len),
                            Err(e) => {
                                println!("HttpLineParsers::extract_content_length: Could not \
                                          decode length as int? {}",
                                         e);
                                return None;
                            }
                        }
                    }
                    None => return None,
                }
            }
            None => return None,
        }
    }
}


#[test]
fn test_http_line_verifier_request_line() {
    assert_eq!(true,
               HttpLineParsers::verify_request_line("POST /foo HTTP/1.0\r\n"));
    assert_eq!(true,
               HttpLineParsers::verify_request_line("GET /foo?foo=ban&baoeu HTTP/1.0\n"));
    assert_eq!(true,
               HttpLineParsers::verify_request_line("GET /foo?foo=ban&baoeu HTTP/1.1\n"));
    assert_eq!(true,
               HttpLineParsers::verify_request_line("GET /foo?foo=ban&baoeu HTTP/0.9\n"));
}

#[test]
fn test_http_line_verifier_end_of_headers() {
    assert_eq!(true, HttpLineParsers::verify_end_of_headers("\r\n"));
}


#[test]
fn test_http_line_verifier_header_line_sane() {
    assert_eq!(true, HttpLineParsers::verify_header_line("Foo: bar\r\n"));
}

#[test]
fn test_http_line_verifier_header_line_some_symbols_are_alright() {
    assert_eq!(true,
               HttpLineParsers::verify_header_line("!#$%&'*+-.^_`|0-9A-Foo::::::\n"));
}

#[test]
fn test_http_line_verifier_header_line_no_colon() {
    assert_eq!(false, HttpLineParsers::verify_header_line("Foeuntaoesoe"));
}

#[test]
fn test_http_line_verifier_header_line_cr_optional() {
    assert_eq!(true, HttpLineParsers::verify_header_line("Foo: \n"));
}

#[test]
fn test_http_line_verifier_header_line_lf_not_optional() {
    assert_eq!(false, HttpLineParsers::verify_header_line("Foo: \r"));
}

#[test]
fn test_http_line_verifier_header_line_tspecial_cant_be_keys() {
    assert_eq!(false,
               HttpLineParsers::verify_header_line("[]{}: notvalid\r\n"));

}
