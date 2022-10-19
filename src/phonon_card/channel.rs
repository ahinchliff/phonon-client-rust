pub struct Channel<T> {
    pub send: Box<super::SendCommand<T>>,
    pairing_public_key: secp256k1::PublicKey,
    pairing_private_key: secp256k1::SecretKey,
    card_pairing_public_key: Option<secp256k1::PublicKey>,
    pairing_shared_secret: Option<secp256k1::ecdh::SharedSecret>,
}

impl<T> Channel<T> {
    pub fn new(send: Box<super::SendCommand<T>>) -> Channel<T> {
        let secp = secp256k1::Secp256k1::new();
        let (pairing_private_key, pairing_public_key) =
            secp.generate_keypair(&mut secp256k1::rand::rngs::OsRng);

        Channel {
            send,
            pairing_public_key,
            pairing_private_key,
            card_pairing_public_key: None,
            pairing_shared_secret: None,
        }
    }

    pub fn set_card_pairing_public_key(&mut self, pk: secp256k1::PublicKey) {
        self.card_pairing_public_key = Some(pk);
        self.pairing_shared_secret = Some(secp256k1::ecdh::SharedSecret::new(
            &pk,
            &self.pairing_private_key,
        ));
    }
}
