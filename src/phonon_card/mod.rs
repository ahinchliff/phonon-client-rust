use crate::apdu;
mod channel;

pub type SendCommand<T> = dyn Fn(apdu::CommandApdu) -> Result<apdu::ResponseApdu, T>;

pub struct PhononCard<T> {
    channel: channel::Channel<T>,
    pub is_initialised: bool,
}

impl<T> PhononCard<T> {
    pub fn new(send: Box<SendCommand<T>>) -> PhononCard<T> {
        let channel = channel::Channel::new(send);
        PhononCard {
            channel,
            is_initialised: false,
        }
    }

    pub fn select(&mut self) -> Result<apdu::select::SelectResponse, T> {
        let apdu = apdu::select::command();
        let raw_response = (self.channel.send)(apdu)?;
        let response = apdu::select::response(raw_response);
        match &response {
            Ok(select_success) => {
                self.channel
                    .set_card_pairing_public_key(select_success.pairing_public_key);
                self.is_initialised = select_success.is_initialised;
            }
            Err(_) => {}
        };
        Ok(response)
    }

    pub fn identify(&self, nonce: [u8; 32]) -> Result<apdu::identify::IdentifyResponse, T> {
        let apdu = apdu::identify::command(nonce);
        let raw_response = (self.channel.send)(apdu)?;
        Ok(apdu::identify::response(raw_response))
    }

    pub fn install_certificate(
        &mut self,
        certificate: Vec<u8>,
    ) -> Result<apdu::install_certificate::InstallCertificateResponse, T> {
        let apdu = apdu::install_certificate::command(certificate);
        let raw_response = (self.channel.send)(apdu)?;
        Ok(apdu::install_certificate::response(raw_response))
    }
}
