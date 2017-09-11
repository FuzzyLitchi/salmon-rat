extern crate salmon;

use std::net::TcpListener;
use salmon::MessageStream;
use salmon::Message;

static PORT: u16 = 1337;

fn main() {
    let listener = TcpListener::bind(("127.0.0.1",PORT)).unwrap();

    let mut server = MessageStream::new(listener.accept().unwrap().0);

    server.send(Message::Print).expect("Failed to send message");
}
