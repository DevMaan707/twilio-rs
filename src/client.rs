use dotenv::dotenv;
use std::env;

#[derive(Clone)]
pub struct TwilioClient {
    pub account_sid: String,
    pub auth_token: String,
    pub from_phone: String,
}

impl TwilioClient {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            account_sid: env::var("TWILIO_ACCOUNT_SID").expect("Missing TWILIO_ACCOUNT_SID"),
            auth_token: env::var("TWILIO_AUTH_TOKEN").expect("Missing TWILIO_AUTH_TOKEN"),
            from_phone: env::var("TWILIO_PHONE_NUMBER").expect("Missing TWILIO_PHONE_NUMBER"),
        }
    }

    pub fn base_url(&self) -> String {
        format!(
            "https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json",
            self.account_sid
        )
    }
}
