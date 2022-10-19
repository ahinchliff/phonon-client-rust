#[derive(Debug)]
pub struct IdentifySuccess {
    pub public_key: secp256k1::PublicKey,
    pub signature: secp256k1::ecdsa::Signature,
}
#[derive(Debug, Clone, Copy)]
pub enum IdentifyError {
    PhononAppletNotFound, // todo - remove
    InvalidCardPublicKey(secp256k1::Error),
    InvalidSignature(secp256k1::Error),
}

pub type IdentifyResponse = Result<IdentifySuccess, super::ResponseError<IdentifyError>>;

pub fn parse(response: crate::apdu::ResponseApdu) -> IdentifyResponse {
    let known_apdu_errors = vec![super::KnownError {
        sw1: 0x6a,
        sw2: 0x82,
        error: IdentifyError::PhononAppletNotFound,
    }];

    super::check_for_apdu_errors(&response, known_apdu_errors)?;

    let public_key_bytes = response.data[2..=66].to_vec();
    let signature_bytes = &response.data[67..];

    let public_key = secp256k1::PublicKey::from_slice(&public_key_bytes).or_else(|e| {
        return Err(super::ResponseError::KnownApduError(
            IdentifyError::InvalidCardPublicKey(e),
        ));
    })?;

    let signature = secp256k1::ecdsa::Signature::from_der(signature_bytes).or_else(|e| {
        return Err(super::ResponseError::KnownApduError(
            IdentifyError::InvalidSignature(e),
        ));
    })?;

    Ok(IdentifySuccess {
        public_key,
        signature,
    })
}
