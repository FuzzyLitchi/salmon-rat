use std::convert::TryInto;

pub trait Message {
    const ID: u8;

    fn into_bytes(self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct Shutdown;

impl Message for Shutdown {
    const ID: u8 = 0;

    fn into_bytes(self) -> Vec<u8> {
        vec![0]
    }
}

#[derive(Debug)]
pub struct Print {
    string: String,
}

impl Message for Print {
    const ID: u8 = 1;

    fn into_bytes(self) -> Vec<u8> {
        let length = self.string.len().try_into().expect("String to long, maximum 256 chars");
        let mut bytes = self.string.into_bytes();

        bytes.insert(0, 1);
        bytes.insert(1, length);
        bytes
    }
}

#[derive(Debug)]
pub enum MessageError {
    NoSuchMessage,
    MissingArg,
}
