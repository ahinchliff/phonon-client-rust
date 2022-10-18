pub mod commands;
pub mod responses;

pub struct CommandApdu {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct ResponseApdu {
    pub sw1: u8,
    pub sw2: u8,
    pub data: Vec<u8>,
}
