use crate::apdu;
mod channel;

pub type SendCommand = dyn Fn(apdu::CommandApdu) -> apdu::ResponseApdu;

pub struct PhononCard<'a> {
    channel: channel::Channel<'a>,
    pub is_initialised: bool,
}

impl PhononCard<'_> {
    pub fn new(send: &SendCommand) -> PhononCard {
        let channel = channel::Channel::new(send);
        PhononCard {
            channel,
            is_initialised: false,
        }
    }

    pub fn select(&mut self) -> apdu::responses::select::SelectResponse {
        let select_apdu = apdu::commands::select();
        let raw_response = (self.channel.send)(select_apdu);
        let response = apdu::responses::select::parse(raw_response);
        match &response {
            Ok(select_success) => {
                self.channel
                    .set_card_pairing_public_key(select_success.pairing_public_key);
                self.is_initialised = select_success.is_initialised;
            }
            Err(_) => {}
        };
        response
    }
}
