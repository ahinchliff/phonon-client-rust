pub struct CommandApdu<'a> {
    pub cla: u8,
    pub ins: u8,
    pub p1: u8,
    pub p2: u8,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub struct ResponseApdu {
    pub sw1: u8,
    pub sw2: u8,
    pub data: Vec<u8>,
}

pub mod commands {
    pub fn new_select_command_apdu() -> crate::apdu::CommandApdu<'static> {
        crate::apdu::CommandApdu {
            cla: 0,
            ins: 164,
            p1: 4,
            p2: 0,
            data: &[160, 0, 0, 8, 32, 0, 3, 1],
        }
    }
}

pub mod responses {

    #[derive(Debug)]
    pub struct SelectResponseOk {
        initialised: bool,
        public_key: secp256k1::PublicKey,
        id: Option<Vec<u8>>,
    }

    pub type SelectResponse = Result<SelectResponseOk, secp256k1::Error>;

    pub fn parse_select_response_apdu(response: crate::apdu::ResponseApdu) -> SelectResponse {
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

        let public_key = secp256k1::PublicKey::from_slice(&public_key_bytes);

        match public_key {
            Ok(public_key) => {
                return Ok(SelectResponseOk {
                    initialised,
                    public_key,
                    id,
                })
            }
            Err(e) => return Err(e),
        }
    }
}
