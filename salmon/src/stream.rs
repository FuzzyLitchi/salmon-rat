use message::Message;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct MessageStream {
    stream: TcpStream,
}

impl MessageStream {
    pub fn new(stream: TcpStream) -> MessageStream {
        MessageStream{
            stream
        }
    }

    pub fn send(&mut self, message: Message) -> Result<usize, io::Error> {
        self.stream.write(&[message.to_u8()])
    }
}

impl Iterator for MessageStream {
    type Item = Message;
    fn next(&mut self) -> Option<Self::Item> {
        let mut byte = [0; 1];
        match self.stream.read(&mut byte) {
            Ok(0) => None,
            Ok(_) => Some(Message::from_u8(byte[0]).unwrap()),
            Err(_) => None,
        }
    }
}
