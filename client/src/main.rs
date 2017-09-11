extern crate salmon;

use std::net::TcpStream;
use salmon::MessageStream;
use salmon::Message;

static IP: &'static str = "127.0.0.1";
static PORT: u16 = 1337;

fn main() {
    let client = MessageStream::new(TcpStream::connect((IP, PORT)).unwrap());

    for message in client {
        println!("{:?}", message);
    }
}
