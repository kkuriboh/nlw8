use std::env;

use lettre::{
	message::header::ContentType,
	transport::smtp::{authentication::Credentials, Error},
	Message, SmtpTransport, Transport,
};

use crate::Feedback;

pub fn send_mail(feedback: &Feedback) -> Result<(), Error> {
	let email: String = env::var("EMAIL").unwrap();
	let message = Message::builder()
		.from(format!("Augusto <{}>", email).parse().unwrap())
		.reply_to(format!("Dio Brando <{}>", email).parse().unwrap())
		.to(format!("Hola <{}>", email).parse().unwrap())
		.subject("FEEDBACK REPORT")
		.header(ContentType::TEXT_HTML)
		.body(
			[
				"<div style=\"font-family: sans-serif; font-size: 16px; color: #111;\">",
				format!("<p>Tipo do feedback{}</p>", feedback.r#type).as_str(),
				format!("<p>Comentário: {}</p>", feedback.comment).as_str(),
				format!(
					"<img src=\"{}\" />",
					feedback.screenshot.as_ref().unwrap_or(&"".to_string())
				)
				.as_str(),
				"</div>",
			]
			.join(" ")
			.to_string(),
		)
		.unwrap();

	let creds = Credentials::new(email, env::var("PSSWD").unwrap());

	let mailer = SmtpTransport::relay("smtp.gmail.com")
		.unwrap()
		.credentials(creds)
		.build();

	match mailer.send(&message) {
		Ok(_) => Ok(()),
		Err(e) => Err(e),
	}
}
