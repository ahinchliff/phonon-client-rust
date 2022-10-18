mod apdu;
mod phonon_card;
mod usb_phonon_card;

fn main() {
    let mut cards = usb_phonon_card::connect_all();
    let card = &mut cards[0];

    match card.select() {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => {
            eprintln!("Command error: {:?}", e)
        }
    };
}
