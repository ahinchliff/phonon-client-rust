use super::CommandApdu;

pub fn select() -> CommandApdu {
    CommandApdu {
        cla: 0,
        ins: 164,
        p1: 4,
        p2: 0,
        data: vec![160, 0, 0, 8, 32, 0, 3, 1],
    }
}

pub fn identify(nonce: [u8; 32]) -> CommandApdu {
    CommandApdu {
        cla: 128,
        ins: 20,
        p1: 0,
        p2: 0,
        data: nonce.to_vec(),
    }
}

pub fn pair_step_one(pairing_public_key: secp256k1::PublicKey, salt: [u8; 32]) -> CommandApdu {
    let pk_bytes = pairing_public_key.serialize().to_vec();
    let pk_length: u8 = pk_bytes.len().try_into().unwrap();

    let mut data: Vec<u8> = vec![];

    data.extend_from_slice(&salt);
    data.push(28); // work out and create constant so no magic numbers
    data.push(pk_length);
    data.extend(pk_bytes);

    CommandApdu {
        cla: 128,
        ins: 18,
        p1: 0,
        p2: 0,
        data,
    }
}
