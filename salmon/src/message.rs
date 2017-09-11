use std::convert::TryInto;

#[derive(Debug)]
pub enum Message {
    Shutdown,
    Print(String),
}

impl Message {
    pub fn from_u8(byte: u8) -> Result<Message, MessageError> {
        match byte {
            0 => Ok(Message::Shutdown),
            1 => Err(MessageError::MissingArg),
            _ => Err(MessageError::NoSuchMessage),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match *self {
            Message::Shutdown => 0,
            Message::Print(_) => 1,
        }
    }

    pub fn into_bytes(self) -> Vec<u8> {
        match self {
            Message::Shutdown => vec![0],
            Message::Print(s) => {
                let length = s.len().try_into().expect("String to long, maximum 256 chars");
                let mut bytes = s.into_bytes();

                bytes.insert(0, 1);
                bytes.insert(1, length);
                bytes
            },
        }
    }
}

#[derive(Debug)]
pub enum MessageError {
    NoSuchMessage,
    MissingArg,
}
