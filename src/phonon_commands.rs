use crate::apdu;

pub type SendCommand = dyn Fn(apdu::CommandApdu) -> apdu::ResponseApdu;

pub struct PhononCardCommands<'a> {
    send: &'a SendCommand,
}

impl PhononCardCommands<'_> {
    pub fn new(send: &SendCommand) -> PhononCardCommands {
        PhononCardCommands { send }
    }

    pub fn select(&self) -> apdu::responses::select::SelectResponse {
        let select_apdu = apdu::commands::select();
        let result = (self.send)(select_apdu);
        return apdu::responses::select::parse(result);
    }
}
