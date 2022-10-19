#[derive(Debug)]
pub struct SelectSuccess {
    pub id: Option<Vec<u8>>,
    pub is_initialised: bool,
    pub pairing_public_key: secp256k1::PublicKey,
}
#[derive(Debug, Clone, Copy)]
pub enum SelectError {
    PhononAppletNotFound,
    InvalidCardPublicKey(secp256k1::Error),
}

pub type SelectResponse = Result<SelectSuccess, super::ResponseError<SelectError>>;

pub fn parse(response: crate::apdu::ResponseApdu) -> SelectResponse {
    let known_apdu_errors = vec![super::KnownError {
        sw1: 0x6a,
        sw2: 0x82,
        error: SelectError::PhononAppletNotFound,
    }];

    super::check_for_apdu_errors(&response, known_apdu_errors)?;

    let initialised = response.data[0] == 164;
    let public_key_length: usize = response.data[1].into();

    let id = if initialised {
        Some(response.data[4..=19].to_vec())
    } else {
        None
    };

    let public_key_bytes = if initialised {
        response.data[22..=86].to_vec()
    } else {
        response.data[2..2 + public_key_length].to_vec()
    };

    let public_key = secp256k1::PublicKey::from_slice(&public_key_bytes).or_else(|e| {
        return Err(super::ResponseError::KnownApduError(
            SelectError::InvalidCardPublicKey(e),
        ));
    })?;

    Ok(SelectSuccess {
        is_initialised: initialised,
        pairing_public_key: public_key,
        id,
    })
}
