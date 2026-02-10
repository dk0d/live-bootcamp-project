use crate::error::AuthApiError;
use askama::Template;

use super::Email;

#[derive(Template)]
#[template(path = "two_factor.html")]
pub struct LoginTemplate<'a> {
    pub email: &'a str,
    pub code: &'a str,
    pub site_url: &'a str,
    pub redirect_url: &'a str,
}

#[async_trait::async_trait]
pub trait EmailClient {
    async fn send_email(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> Result<(), AuthApiError>;
}
