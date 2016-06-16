

use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;


pub struct HttpReader<'a> {
    // <Read> (trait) says read(&self) so mutable borrow
    stream: &'a mut BufReader<TcpStream>,
}

impl<'a> HttpReader<'a> {
    pub fn new(stream: &'a mut BufReader<TcpStream>) -> HttpReader<'a> {
        return HttpReader { stream: stream };
    }

    pub fn read_line(&mut self) -> Result<String, &'static str> {
        let mut line = String::new();

        return match self.stream.read_line(&mut line) {
            Ok(0) => Err("Empty read"),
            Ok(_) => Ok(line),
            Err(_) => Err("Failed read"),
        };
    }
}
