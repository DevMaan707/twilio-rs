use crate::client::TwilioClient;
use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TwilioMessageResponse {
    pub sid: String,
    pub status: String,
    pub to: String,
    pub from: String,
    pub body: String,
}

impl TwilioClient {
    pub async fn send_sms(
        &self,
        to: &str,
        body: &str,
    ) -> Result<TwilioMessageResponse, Box<dyn std::error::Error>> {
        let client = Client::new();
        let params = [("To", to), ("From", &self.from_phone), ("Body", body)];

        let res = client
            .post(&self.base_url())
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .form(&params)
            .send()
            .await?
            .json::<TwilioMessageResponse>()
            .await?;

        Ok(res)
    }
}
