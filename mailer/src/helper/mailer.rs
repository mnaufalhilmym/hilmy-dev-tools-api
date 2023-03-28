use std::error::Error;

use lettre::{message::MessageBuilder, SmtpTransport, Transport};
use tools_mailer::contract::MailReq;

pub fn send_mail(
    message_builder: &MessageBuilder,
    smtp_transport: &SmtpTransport,
    payload: MailReq,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let message = message_builder
        .to_owned()
        .to(payload.to.parse()?)
        .subject(payload.subject)
        .body(payload.body)?;
    smtp_transport.send(&message)?;

    Ok(())
}
