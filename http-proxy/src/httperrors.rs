
use std::io::Error;
use std::fmt;

pub struct HttpError {
    pub code: i32,
    pub message: String,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HttpError({}, \"{}\")", self.code, self.message)
    }
}

impl HttpError {
    pub fn malformed_request(e: &'static str) -> HttpError {
        return HttpError {
            code: 400,
            message: format!("Malformed request: {}", e),
        };
    }

    pub fn malformed_reply(e: &'static str) -> HttpError {
        return HttpError {
            code: 502,
            message: format!("Bad gateway: {}", e),
        };
    }

    pub fn client_timeout(e: Error) -> HttpError {
        return HttpError {
            code: 408,
            message: format!("Client timeout: {}", e),
        };
    }

    pub fn gateway_timeout(e: Error) -> HttpError {
        return HttpError {
            code: 504,
            message: format!("Gateway timeout: {}", e),
        };
    }
}
