#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate regex;

use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{BufReader, Write};
// use std::str;
pub use httpstate::{Header, HttpResponse, HttpRequestParser, HttpResponseParser,
                    HttpReader};
pub use httperrors::HttpError;
pub use proxy::do_proxy;
mod httperrors;
mod httplineparser;
mod httpstate;
mod proxy;



fn return_error_to_client(mut client_stream: BufReader<TcpStream>, error: HttpError) {
    let response = HttpResponse::from_error(error);

    let client_stream = client_stream.get_mut();

    if let Err(e) = client_stream.write(&response.as_string().as_bytes()) {
        println!("Encountered error returning an error to the client: {}", e);
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

                            println!("{} connected", ident);

                            let mut client_stream = BufReader::new(client_stream);
                            match do_proxy(&mut client_stream, backend) {
                                Err(err) => {
                                    return_error_to_client(client_stream, err);
                                }
                                Ok(_) => {}
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
