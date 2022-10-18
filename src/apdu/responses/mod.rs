pub mod select;

const SUCCESS_SW1: u8 = 0x90;
const SUCCESS_SW2: u8 = 0x00;

struct KnownError<Error> {
    sw1: u8,
    sw2: u8,
    error: Error,
}

#[derive(Debug)]
pub enum ResponseError<T> {
    KnownError(T),
    UnknownError(u8, u8),
}

fn check_for_apdu_errors<T>(
    response: &crate::apdu::ResponseApdu,
    known_errors: Vec<KnownError<T>>,
) -> Result<(), ResponseError<T>>
where
    T: Copy,
{
    if response.sw1 == SUCCESS_SW1 && response.sw2 == SUCCESS_SW2 {
        return Ok(());
    }

    for possible_error in known_errors.iter() {
        if possible_error.sw1 == response.sw1 && possible_error.sw2 == response.sw2 {
            return Err(ResponseError::KnownError(possible_error.error));
        }
    }

    Err(ResponseError::UnknownError(response.sw1, response.sw1))
}
