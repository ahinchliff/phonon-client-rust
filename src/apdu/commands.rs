use super::CommandApdu;

pub fn select() -> CommandApdu {
    CommandApdu {
        cla: 0,
        ins: 164,
        p1: 4,
        p2: 0,
        data: vec![160, 0, 0, 8, 32, 0, 3, 1],
    }
}
