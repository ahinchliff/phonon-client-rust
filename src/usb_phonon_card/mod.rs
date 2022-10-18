use crate::apdu;
use crate::phonon_card;
use pcsc;

fn encode_command_apdu(command: apdu::CommandApdu) -> Vec<u8> {
    let data_size = command.data.len().try_into().unwrap();
    let mut result = vec![command.cla, command.ins, command.p1, command.p2, data_size];
    result.extend(command.data);
    result.push(0); // todo - is this le?
    result
}

fn parse_response_apdu(response: Vec<u8>) -> apdu::ResponseApdu {
    let sw1 = response[response.len() - 2];
    let sw2 = response[response.len() - 1];

    let data = if response.len() == 2 {
        vec![]
    } else {
        response[0..=response.len() - 3].to_vec()
    };

    apdu::ResponseApdu { sw1, sw2, data }
}

fn create_send(card: pcsc::Card) -> Box<phonon_card::SendCommand> {
    Box::new(move |command: apdu::CommandApdu| {
        let command = encode_command_apdu(command);

        let mut rapdu_buf = [0; pcsc::MAX_BUFFER_SIZE];

        let rapdu = match card.transmit(&command, &mut rapdu_buf) {
            Ok(rapdu) => rapdu,
            Err(err) => {
                eprintln!("Failed to transmit APDU command to card: {}", err);
                std::process::exit(1);
            }
        };

        parse_response_apdu(rapdu.to_vec())
    })
}

pub fn connect_all() -> Vec<phonon_card::PhononCard<'static>> {
    let ctx = match pcsc::Context::establish(pcsc::Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Failed to establish context: {}", err);
            std::process::exit(1);
        }
    };

    let mut readers_buf = [0; 2048];
    let readers = match ctx.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(err) => {
            eprintln!("Failed to list readers: {}", err);
            std::process::exit(1);
        }
    };

    readers
        .into_iter()
        .map(
            |reader| match ctx.connect(reader, pcsc::ShareMode::Shared, pcsc::Protocols::ANY) {
                Ok(card) => Some(card),
                Err(pcsc::Error::NoSmartcard) => {
                    println!("A smartcard is not present in the reader {:?}", reader);
                    None
                }
                Err(err) => {
                    eprintln!("Failed to connect to card in reader {:?}: {}", reader, err);
                    None
                }
            },
        )
        .collect::<Vec<Option<pcsc::Card>>>()
        .into_iter()
        .filter_map(|card| match card {
            Some(card) => {
                let send = create_send(card);
                Some(phonon_card::PhononCard::new(&send))
            }
            None => None,
        })
        .collect::<Vec<phonon_card::PhononCard>>()
}
