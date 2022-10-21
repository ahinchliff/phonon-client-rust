mod test_utils;
use rand::Rng;

#[test]
fn test_identify_no_certificate_installed() {
    loop {
        // signature validation sometimes fails. Loop until it does so I dont forget
        test_utils::install_applet();
        let mut card = test_utils::get_first_connected_card();
        card.select().unwrap().unwrap();
        let nonce = rand::thread_rng().gen::<[u8; 32]>();
        let result = card.identify(nonce).unwrap().unwrap();

        let secp = secp256k1::Secp256k1::new();
        let message = secp256k1::Message::from_slice(&nonce).unwrap();

        let verification_result =
            secp.verify_ecdsa(&message, &result.signature, &result.public_key);

        match verification_result {
            Ok(_) => (),
            Err(e) => panic!("{:?}", e),
        }
    }
}

#[test]
fn identify_with_certificate_installed() {
    test_utils::install_applet();
    let mut card = test_utils::get_first_connected_card();
    card.select().unwrap().unwrap();

    test_utils::create_and_install_demo_certificate(&mut card);

    let nonce = rand::thread_rng().gen::<[u8; 32]>();
    let result = card.identify(nonce).unwrap().unwrap();

    let secp = secp256k1::Secp256k1::new();
    let message = secp256k1::Message::from_slice(&nonce).unwrap();

    let verification_result = secp.verify_ecdsa(&message, &result.signature, &result.public_key);

    match verification_result {
        Ok(_) => return,
        Err(e) => panic!("{:?}", e),
    }
}
