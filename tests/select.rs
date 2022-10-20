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
fn test_select_happy_path_not_initialised() {
    test_utils::install_applet();
    let mut card = test_utils::get_first_connected_card();
    let response = card.select().unwrap().unwrap();
    assert_eq!(response.is_initialised, false);
}

#[test]
fn test_select_happy_path_initialised() {
    test_utils::install_applet();
    let mut card = test_utils::get_first_connected_card();
    let response = card.select().unwrap().unwrap();
    assert_eq!(response.is_initialised, true);
    assert!(response.id.is_some());
}
