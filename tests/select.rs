use phonon_client_rust::apdu;
use phonon_client_rust::usb_phonon_card;
use std::process::Command;

fn install_applet() {
    Command::new("./build-and-install-applet.sh")
        .current_dir("../phonon-card")
        .output()
        .expect("failed to execute process");
}

fn delete_applet() {
    Command::new("./delete-applet.sh")
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
fn test_select_applet_not_installed() {
    delete_applet();
    let mut card = get_first_connected_card();
    match card.select().unwrap().unwrap_err() {
        apdu::responses::ResponseError::KnownApduError(error) => {
            assert!(matches!(
                error,
                apdu::responses::select::SelectError::PhononAppletNotFound
            ))
        }
        apdu::responses::ResponseError::UnknownApduError(sw1, sw2) => {
            panic!("Unknown Error: {:?} {:?}", sw1, sw2)
        }
    };
}

#[test]
fn test_select_happy_path_not_initialised() {
    install_applet();
    let mut card = get_first_connected_card();
    let response = card.select().unwrap().unwrap();
    assert_eq!(response.is_initialised, false);
}

#[test]
fn test_select_happy_path_initialised() {
    install_applet();
    let mut card = get_first_connected_card();
    let response = card.select().unwrap().unwrap();
    assert_eq!(response.is_initialised, true);
    assert!(response.id.is_some());
}
