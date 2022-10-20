use super::*;

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

pub type SelectResponse = Result<SelectSuccess, ResponseError<SelectError>>;

pub fn command() -> CommandApdu {
    CommandApdu {
        cla: 0,
        ins: 164,
        p1: 4,
        p2: 0,
        data: vec![160, 0, 0, 8, 32, 0, 3, 1],
    }
}

pub fn response(response: crate::apdu::ResponseApdu) -> SelectResponse {
    let known_apdu_errors = vec![KnownError {
        sw: SW_APPLET_NOT_FOUND,
        error: SelectError::PhononAppletNotFound,
    }];

    check_for_apdu_errors(&response, known_apdu_errors)?;

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
        return Err(ResponseError::KnownApduError(
            SelectError::InvalidCardPublicKey(e),
        ));
    })?;

    Ok(SelectSuccess {
        is_initialised: initialised,
        pairing_public_key: public_key,
        id,
    })
}
