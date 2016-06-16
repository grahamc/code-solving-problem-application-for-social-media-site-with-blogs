#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufReader, Read, Write};
// use std::str;
pub use httpstate::{Header, HttpRequest, HttpResponse, HttpRequestParser, HttpResponseParser,
                    HttpReader};
pub use httperrors::HttpError;

mod httperrors;
mod httplineparser;
mod httpstate;

fn do_proxy(client_stream: BufReader<TcpStream>,
            backend: &'static str)
            -> Result<(HttpResponse, BufReader<TcpStream>, BufReader<TcpStream>),
                      (HttpError, BufReader<TcpStream>)> {
    let mut req_parser = HttpRequestParser::new(client_stream);
    let mut parsed_request: HttpRequest;
    match req_parser.parse() {
        Ok(request) => {
            parsed_request = request;
        }
        Err(e) => return Err((e, req_parser.stream())),
    }

    let mut client_stream = req_parser.stream();

    let client_ip = client_stream.get_ref().peer_addr().unwrap();
    let forwarded_for = Header::from_string(format!("X-Forwarded-For: {}\r\n", client_ip));
    parsed_request.add_header(forwarded_for.unwrap());

    let mut server_stream: TcpStream;
    match TcpStream::connect(backend) {
        Ok(connection) => {
            server_stream = connection;
        }
        Err(e) => {
            return Err((HttpError::gateway_timeout(e), client_stream));
        }
    }

    match server_stream.write(&parsed_request.as_string().as_bytes()) {
        Ok(_) => {}
        Err(e) => {
            return Err((HttpError::gateway_timeout(e), client_stream));
        }
    }

    let mut remaining = parsed_request.body_length();
    while remaining > 0 {
        let mut buf = [0; 4096];
        match client_stream.read(&mut buf) {
            Ok(n) => {
                remaining -= n;
            }
            Err(e) => {
                return Err((HttpError::client_timeout(e), client_stream));
            }
        }

        match server_stream.write(&buf) {
            Ok(_) => {}
            Err(e) => {
                return Err((HttpError::gateway_timeout(e), client_stream));
            }
        }
    }

    let server_stream = BufReader::new(server_stream);
    let mut resp_parser = HttpResponseParser::new(server_stream);

    let mut response: HttpResponse;

    match resp_parser.parse() {
        Ok(resp) => response = resp,
        Err(e) => {
            return Err((e, client_stream));
        }
    }

    let backend = Header::from_string(format!("X-Backend: {}\r\n", &backend));
    response.add_header(backend.unwrap());

    return Ok((response, client_stream, resp_parser.stream()));
}

fn return_error_to_client(mut client_stream: BufReader<TcpStream>, error: HttpError) {
    let response = HttpResponse::from_error(error);

    let client_stream = client_stream.get_mut();
    client_stream.write(&response.as_string().as_bytes());
}

fn stream_response_to_client(mut client_stream: BufReader<TcpStream>,
                             mut server_stream: BufReader<TcpStream>,
                             response: HttpResponse) {
    let client_stream = client_stream.get_mut();
    client_stream.write(&response.as_string().as_bytes());
    let mut remaining = response.body_length();
    while remaining > 0 {
        let mut buf = [0; 4096];
        match server_stream.read(&mut buf) {
            Ok(n) => {
                remaining -= n;
            }
            Err(_) => {
                break;
            }
        }
        client_stream.write(&buf);
    }
}
fn main() {
    let threads_per_backend = 1;


    let backends = ["127.0.0.1:8001", "127.0.0.1:8002", "127.0.0.1:8003", "127.0.0.1:8004"];

    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());

    let mut threads = Vec::new();
    for idx in 0..threads_per_backend {
        for backend in backends.iter() {

            let backend = backend.clone();
            let backend_ident = format!("{}#{}", &backend, idx);
            let listener = listener.try_clone().unwrap();
            threads.push(thread::spawn(move || {
                println!("Backend {} listening", backend_ident);
                for client_stream in listener.incoming() {
                    match client_stream {
                        Ok(client_stream) => {
                            let client_ident = client_stream.peer_addr().unwrap();
                            let ident = format!("{} <--> {}", client_ident, backend_ident);

                            // println!("{} connected", ident);

                            let client_stream = BufReader::new(client_stream);
                            match do_proxy(client_stream, backend) {
                                Err((err, client_stream)) => {
                                    return_error_to_client(client_stream, err);
                                }
                                Ok((response, client_stream, server_stream)) => {
                                    stream_response_to_client(client_stream,
                                                              server_stream,
                                                              response);
                                }
                            };

                        }
                        Err(e) => {
                            println!("Error accepting client: {}", e);
                        }
                    }
                }
            }));
        }
    }

    for handle in threads {
        handle.join().unwrap();
    }
}
