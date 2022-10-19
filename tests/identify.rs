use phonon_client_rust::usb_phonon_card;
use rand::Rng;
use std::process::Command;

fn install_applet() {
    Command::new("./build-and-install-applet.sh")
        .current_dir("../phonon-card")
        .output()
        .expect("failed to execute process");
}

fn get_first_connected_card() -> usb_phonon_card::UsbPhononCard {
    let mut cards = match usb_phonon_card::connect_all() {
        Ok(cards) => cards,
        Err(err) => {
            panic!("Failed to connect to cards: {:?}", err)
        }
    };
    if cards.len() == 0 {
        panic!("No cards connected")
    }
    cards.remove(0)
}

#[test]
fn test_identify_no_certificate_installed() {
    install_applet();
    let mut card = get_first_connected_card();
    let nonce = rand::thread_rng().gen::<[u8; 32]>();
    let _ = card.select();
    let result = card.identify(nonce).unwrap().unwrap();

    let secp = secp256k1::Secp256k1::new();
    let message = secp256k1::Message::from_slice(&nonce).unwrap();

    assert!(secp
        .verify_ecdsa(&message, &result.signature, &result.public_key)
        .is_ok())
}
