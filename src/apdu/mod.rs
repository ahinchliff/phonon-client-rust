pub mod identify;
pub mod install_certificate;
pub mod select;

pub struct CommandApdu {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct ResponseApdu {
    pub sw: SW,
    pub data: Vec<u8>,
}

struct KnownError<Error> {
    sw: SW,
    error: Error,
}

#[derive(Debug)]
pub enum ResponseError<T> {
    KnownApduError(T),
    UnknownApduError(u8, u8),
}

fn check_for_apdu_errors<T>(
    response: &crate::apdu::ResponseApdu,
    known_errors: Vec<KnownError<T>>,
) -> Result<(), ResponseError<T>>
where
    T: Copy,
{
    if response.sw.sw1 == SW_SUCCESS.sw1 && response.sw.sw2 == SW_SUCCESS.sw2 {
        return Ok(());
    }

    for possible_error in known_errors.iter() {
        if possible_error.sw.sw1 == response.sw.sw1 && possible_error.sw.sw2 == response.sw.sw2 {
            return Err(ResponseError::KnownApduError(possible_error.error));
        }
    }

    Err(ResponseError::UnknownApduError(
        response.sw.sw1,
        response.sw.sw2,
    ))
}

#[derive(Debug)]
pub struct SW {
    pub sw1: u8,
    pub sw2: u8,
}

const SW_SUCCESS: SW = SW {
    sw1: 0x90,
    sw2: 0x00,
};

const SW_APPLET_NOT_FOUND: SW = SW {
    sw1: 0x6a,
    sw2: 0x82,
};

const SW_INVALID_DATA: SW = SW {
    sw1: 0x69,
    sw2: 0x84,
};

const SW_DATA_WRONG_LENGTH: SW = SW {
    sw1: 0x67,
    sw2: 0x00,
};

const SW_COMMAND_NOT_ALLOWED: SW = SW {
    sw1: 0x69,
    sw2: 0x86,
};

const SW_SECURITY_CONDITIONS_NOT_SATISFIED: SW = SW {
    sw1: 0x69,
    sw2: 0x82,
};
