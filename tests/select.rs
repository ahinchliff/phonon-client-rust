use phonon_client_rust::apdu;
mod test_utils;

#[test]
fn test_select_applet_not_installed() {
    test_utils::delete_applet();
    let mut card = test_utils::get_first_connected_card();
    match card.select().unwrap().unwrap_err() {
        apdu::ResponseError::KnownApduError(error) => {
            assert!(matches!(
                error,
                apdu::select::SelectError::PhononAppletNotFound
            ))
        }
        apdu::ResponseError::UnknownApduError(sw1, sw2) => {
            panic!("Unknown Error: {:?} {:?}", sw1, sw2)
        }
    };
}

#[test]
fn test_not_initialised_and_no_certificate() {
    test_utils::install_applet();
    let mut card = test_utils::get_first_connected_card();
    let response = card.select().unwrap().unwrap();
    assert_eq!(response.is_initialised, false);
}

#[test]
fn test_not_initialised_and_with_certificate() {
    test_utils::install_applet();
    let mut card = test_utils::get_first_connected_card();
    card.select().unwrap().unwrap();
    test_utils::create_and_install_demo_certificate(&mut card);

    let response = card.select().unwrap().unwrap();

    assert_eq!(response.is_initialised, false);
}

#[test]
fn test_select_initialised() {
    // todo
}
