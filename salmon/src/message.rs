#[derive(Debug)]
pub enum Message {
    Shutdown,
    Print,
}

impl Message {
    pub fn from_u8(byte: u8) -> Result<Message, MessageError> {
        match byte {
            0 => Ok(Message::Shutdown),
            1 => Ok(Message::Print),
            _ => Err(MessageError::OutOffEnum),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match *self {
            Message::Shutdown => 0,
            Message::Print => 1,
        }
    }
}

pub enum MessageError {
    OutOffEnum,
    MissingArg,
}
