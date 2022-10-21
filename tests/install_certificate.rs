mod test_utils;
use phonon_client_rust::apdu;
use rand::Rng;

#[test]
fn install_certificate() {
    test_utils::install_applet();
    let mut card = test_utils::get_first_connected_card();
    card.select().unwrap().unwrap();
    let nonce = rand::thread_rng().gen::<[u8; 32]>();
    let identity = card.identify(nonce).unwrap().unwrap();
    let certificate = test_utils::create_demo_card_certificate(identity.public_key);
    card.install_certificate(certificate).unwrap().unwrap();
}

#[test]
fn install_certificate_when_certificate_already_installed() {
    test_utils::install_applet();
    let mut card = test_utils::get_first_connected_card();
    card.select().unwrap().unwrap();
    let nonce = rand::thread_rng().gen::<[u8; 32]>();
    let identity = card.identify(nonce).unwrap().unwrap();
    let certificate = test_utils::create_demo_card_certificate(identity.public_key);
    card.install_certificate(certificate).unwrap().unwrap();

    let certificate = test_utils::create_demo_card_certificate(identity.public_key);
    let result = card.install_certificate(certificate).unwrap();

    match result.unwrap_err() {
        apdu::ResponseError::KnownApduError(error) => {
            assert!(matches!(
                error,
                apdu::install_certificate::InstallCertificateError::CardAlreadyHasCertificate
            ))
        }
        _ => {
            panic!();
        }
    };
}

#[test]
fn test_select_initialised() {
    // todo
}

#[test]
fn test_create_signature() {
    let public_key_bytes: Vec<u8> = vec![
        4, 137, 245, 63, 56, 199, 139, 221, 181, 168, 105, 148, 251, 52, 247, 172, 203, 9, 45, 170,
        223, 3, 226, 5, 228, 234, 175, 42, 61, 223, 173, 10, 103, 161, 47, 208, 67, 167, 97, 46,
        238, 224, 164, 255, 238, 8, 255, 100, 87, 62, 194, 47, 95, 119, 37, 28, 202, 58, 83, 98,
        187, 180, 214, 168, 77,
    ];

    let public_key = secp256k1::PublicKey::from_slice(&public_key_bytes).unwrap();

    let cert = test_utils::create_demo_card_certificate(public_key);

    assert_eq!(
        cert,
        [
            48, 144, 2, 2, 0, 0, 128, 65, 4, 137, 245, 63, 56, 199, 139, 221, 181, 168, 105, 148,
            251, 52, 247, 172, 203, 9, 45, 170, 223, 3, 226, 5, 228, 234, 175, 42, 61, 223, 173,
            10, 103, 161, 47, 208, 67, 167, 97, 46, 238, 224, 164, 255, 238, 8, 255, 100, 87, 62,
            194, 47, 95, 119, 37, 28, 202, 58, 83, 98, 187, 180, 214, 168, 77, 48, 69, 2, 33, 0,
            144, 119, 31, 223, 58, 96, 27, 113, 241, 194, 147, 227, 132, 105, 21, 102, 79, 159,
            219, 112, 0, 207, 119, 145, 185, 141, 162, 192, 39, 147, 107, 42, 2, 32, 122, 141, 128,
            218, 21, 176, 157, 58, 175, 186, 165, 63, 237, 49, 28, 140, 142, 240, 55, 72, 255, 2,
            118, 28, 244, 67, 164, 58, 83, 58, 44, 148
        ]
    )
}
