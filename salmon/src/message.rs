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
}

#[derive(Debug)]
pub enum MessageError {
    NoSuchMessage,
    MissingArg,
}
