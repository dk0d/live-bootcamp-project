use crate::error::AuthApiError;
use askama::Template;

use super::Email;

#[derive(Template, Clone, Debug)]
#[template(path = "two_factor.html")]
pub struct TwoFactorEmailData {
    pub email: String,
    pub code: String,
    pub site_url: String,
    pub redirect_url: String,
}

#[derive(Clone, Debug)]
pub enum EmailTemplate {
    TwoFactor(TwoFactorEmailData),
}

impl EmailTemplate {
    pub fn render(&self) -> String {
        match self {
            EmailTemplate::TwoFactor(data) => data.render().expect("valid html"),
        }
    }
}

#[async_trait::async_trait]
pub trait EmailClient: Send + Sync + std::fmt::Debug {
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        template: &EmailTemplate,
    ) -> Result<(), AuthApiError>;
}
