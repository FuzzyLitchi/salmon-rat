#![feature(try_from)]

mod stream;
mod message;

pub use message::*;
pub use stream::*;

#[cfg(test)]
mod tests {
    #[test]
    fn stream() {
        use stream::MessageStream;
        use message::Message;
        use std::net::{TcpListener, TcpStream};

        let listner = TcpListener::bind("127.0.0.1:1337").unwrap();
        let client = TcpStream::connect("127.0.0.1:1337").unwrap();

        let (server, _) = listner.accept().unwrap();

        let mut message_server = MessageStream::new(server);
        let mut message_client = MessageStream::new(client);

        let _ = message_server.send(Message::Shutdown);

        assert_eq!(message_client.next().unwrap().to_u8(),
                   Message::Shutdown.to_u8());
    }
}
