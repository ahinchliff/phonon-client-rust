use super::*;

#[derive(Debug)]
pub struct InstallCertificateSuccess {}

#[derive(Debug, Clone, Copy)]
pub enum InstallCertificateError {
    CardAlreadyHasCertificate,
    InvalidCASignature,
}

pub type InstallCertificateResponse =
    Result<InstallCertificateSuccess, ResponseError<InstallCertificateError>>;

pub fn command(certificate: Vec<u8>) -> CommandApdu {
    CommandApdu {
        cla: 128,
        ins: 21,
        p1: 0,
        p2: 0,
        data: certificate,
    }
}

pub fn response(response: crate::apdu::ResponseApdu) -> InstallCertificateResponse {
    let known_apdu_errors = vec![
        KnownError {
            sw: SW_COMMAND_NOT_ALLOWED,
            error: InstallCertificateError::CardAlreadyHasCertificate,
        },
        KnownError {
            sw: SW_SECURITY_CONDITIONS_NOT_SATISFIED,
            error: InstallCertificateError::InvalidCASignature,
        },
    ];

    check_for_apdu_errors(&response, known_apdu_errors)?;

    Ok(InstallCertificateSuccess {})
}
