use crate::domain::{Email, EmailClient, Password};
use crate::error::AuthApiError;

use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

#[derive(Debug)]
pub struct Emailer {
    pub config: EmailConfig,
}

#[derive(serde::Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub struct EmailConfig {
    pub smtp_host: String,
    pub smtp_port: u16,
    pub sender: Email,
    password: Password,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            smtp_host: "127.0.0.1".to_string(),
            smtp_port: 1025,
            sender: Email::parse("dev@local.test").expect("valid email"),
            password: Password::parse("password123-ignore-me").expect("valid password"),
        }
    }
}

impl Emailer {
    pub fn new(config: &EmailConfig) -> Self {
        Self {
            config: config.clone(),
        }
    }
}

#[async_trait::async_trait]
impl EmailClient for Emailer {
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> Result<(), AuthApiError> {
        let email = Message::builder()
            .from(Mailbox::new(
                Some("LGRAuth".to_owned()),
                self.config.sender.as_ref().parse().unwrap(),
            ))
            .to(Mailbox::new(
                Some("User".to_owned()),
                recipient.as_ref().parse().unwrap(),
            ))
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(content.to_string())
            .unwrap();

        let creds = Credentials::new(
            self.config.sender.as_ref().to_owned(),
            self.config.password.as_ref().to_owned(),
        );

        // FIXME: allow insecure connections for local SMTP server
        let mailer = if self.config.smtp_host == "127.0.0.1" {
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&self.config.smtp_host)
                // .credentials(creds)
                .port(self.config.smtp_port)
                .build()
        } else {
            AsyncSmtpTransport::<Tokio1Executor>::relay(&self.config.smtp_host)
                .unwrap()
                .credentials(creds)
                .port(self.config.smtp_port)
                .build()
        };

        mailer
            .send(email)
            .await
            .map_err(|e| AuthApiError::EmailSendError(e.to_string()))?;

        Ok(())
    }
}
