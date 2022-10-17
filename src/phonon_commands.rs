use crate::apdu;

pub type SendCommand = dyn Fn(apdu::CommandApdu) -> apdu::ResponseApdu;

pub struct PhononCardCommands<'a> {
    send: &'a SendCommand,
}

impl PhononCardCommands<'_> {
    pub fn new(send: &SendCommand) -> PhononCardCommands {
        PhononCardCommands { send }
    }

    pub fn select(&self) -> apdu::responses::SelectResponse {
        let select_apdu = apdu::commands::new_select_command_apdu();
        let result = (self.send)(select_apdu);
        return apdu::responses::parse_select_response_apdu(result);
    }
}
