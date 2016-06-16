
use std::io::Error;


pub struct HttpError {
    pub code: i32,
    pub message: String,
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