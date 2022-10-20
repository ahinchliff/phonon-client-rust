use phonon_client_rust::usb_phonon_card;
use rand::Rng;

use secp256k1::Message;
use secp256k1::Secp256k1;
use secp256k1::SecretKey;
use std::process::Command;

pub fn install_applet() {
    Command::new("./build-and-install-applet.sh")
        .current_dir("../phonon-card")
        .output()
        .expect("failed to execute process");
}

pub fn delete_applet() {
    Command::new("./delete-applet.sh")
        .current_dir("../phonon-card")
        .output()
        .expect("failed to execute process");
}

pub fn create_dev_certificate(card: &mut usb_phonon_card::UsbPhononCard) -> Vec<u8> {
    let mut perms: Vec<u8> = vec![0x30, 0x00, 0x02, 0x02, 0x00, 0x00, 0x80, 0x41];

    let demo_ca_private_key_bytes: [u8; 32] = [
        0x03, 0x8D, 0x01, 0x08, 0x90, 0x00, 0x00, 0x00, 0x10, 0xAA, 0x82, 0x07, 0x09, 0x80, 0x00,
        0x00, 0x01, 0xBB, 0x03, 0x06, 0x90, 0x08, 0x35, 0xF9, 0x10, 0xCC, 0x04, 0x85, 0x09, 0x00,
        0x00, 0x91,
    ];

    let demo_ca_private_key = SecretKey::from_slice(&demo_ca_private_key_bytes).unwrap();

    let nonce = rand::thread_rng().gen::<[u8; 32]>();
    let result = card.identify(nonce).unwrap().unwrap();
    let mut card_public_key_bytes = result.public_key.serialize_uncompressed().to_vec();

    let mut certificate: Vec<u8> = vec![];

    certificate.append(&mut perms);
    certificate.append(&mut card_public_key_bytes);

    let pre_image = certificate[2..].to_vec();
    let message = Message::from_slice(&pre_image).unwrap();

    let secp = Secp256k1::new();

    let signature = secp.sign_ecdsa(&message, &demo_ca_private_key);

    certificate.append(&mut signature.serialize_compact().to_vec());

    certificate[1] = certificate.len().try_into().unwrap();

    certificate
}

pub fn get_first_connected_card() -> usb_phonon_card::UsbPhononCard {
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
