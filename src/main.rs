use pcsc::*;
mod apdu;
mod phonon_commands;

fn encode_command_apdu(command: apdu::CommandApdu) -> Vec<u8> {
    let data_size = command.data.len().try_into().unwrap();
    let mut result = vec![command.cla, command.ins, command.p1, command.p2, data_size];
    result.extend_from_slice(command.data);
    result.push(0); // todo - is this le?
    return result;
}

fn parse_response_apdu(response: Vec<u8>) -> apdu::ResponseApdu {
    let sw1 = response[response.len() - 2];
    let sw2 = response[response.len() - 1];
    let data = response[0..=response.len() - 3].to_vec();
    apdu::ResponseApdu { sw1, sw2, data }
}

fn main() {
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Failed to establish context: {}", err);
            std::process::exit(1);
        }
    };

    let mut readers_buf = [0; 2048];
    let mut readers = match ctx.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(err) => {
            eprintln!("Failed to list readers: {}", err);
            std::process::exit(1);
        }
    };

    let reader = match readers.next() {
        Some(reader) => reader,
        None => {
            println!("No readers are connected.");
            return;
        }
    };
    println!("Using reader: {:?}", reader);

    let card = match ctx.connect(reader, ShareMode::Shared, Protocols::ANY) {
        Ok(card) => card,
        Err(Error::NoSmartcard) => {
            println!("A smartcard is not present in the reader.");
            return;
        }
        Err(err) => {
            eprintln!("Failed to connect to card: {}", err);
            std::process::exit(1);
        }
    };

    fn create_send(card: Card) -> Box<phonon_commands::SendCommand> {
        Box::new(move |command: apdu::CommandApdu| {
            let command = encode_command_apdu(command);

            let mut rapdu_buf = [0; MAX_BUFFER_SIZE];

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

    let send = create_send(card);
    let commands = phonon_commands::PhononCardCommands::new(&send);
    let result = commands.select();
    println!("Result: {:?}", result);
}
