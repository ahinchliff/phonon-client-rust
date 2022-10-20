use super::*;

#[derive(Debug)]
pub struct IdentifySuccess {
    pub public_key: secp256k1::PublicKey,
    pub signature: secp256k1::ecdsa::Signature,
}

#[derive(Debug, Clone, Copy)]
pub enum IdentifyError {
    InvalidNonce,
    InvalidCardPublicKey(secp256k1::Error),
    InvalidSignature(secp256k1::Error),
}

pub type IdentifyResponse = Result<IdentifySuccess, ResponseError<IdentifyError>>;

pub fn command(nonce: [u8; 32]) -> CommandApdu {
    CommandApdu {
        cla: 128,
        ins: 20,
        p1: 0,
        p2: 0,
        data: nonce.to_vec(),
    }
}

pub fn response(response: crate::apdu::ResponseApdu) -> IdentifyResponse {
    let known_apdu_errors = vec![
        KnownError {
            sw: SW_INVALID_DATA,
            error: IdentifyError::InvalidNonce,
        },
        KnownError {
            sw: SW_DATA_WRONG_LENGTH,
            error: IdentifyError::InvalidNonce,
        },
    ];

    check_for_apdu_errors(&response, known_apdu_errors)?;

    let public_key_bytes = response.data[2..=66].to_vec();
    let signature_bytes = &response.data[67..];

    let public_key = secp256k1::PublicKey::from_slice(&public_key_bytes).or_else(|e| {
        return Err(ResponseError::KnownApduError(
            IdentifyError::InvalidCardPublicKey(e),
        ));
    })?;

    let signature = secp256k1::ecdsa::Signature::from_der(signature_bytes).or_else(|e| {
        return Err(ResponseError::KnownApduError(
            IdentifyError::InvalidSignature(e),
        ));
    })?;

    Ok(IdentifySuccess {
        public_key,
        signature,
    })
}
