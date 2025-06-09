use crate::client::TwilioClient;
use reqwest::{multipart, Client};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct TwilioMessageResponse {
    pub sid: String,
    pub status: String,
    pub to: String,
    pub from: String,
    pub body: Option<String>,
}

pub async fn send_whatsapp_text(
    client: &TwilioClient,
    to: &str,
    message: &str,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let to = format!("whatsapp:{}", to);
    let from = format!("whatsapp:{}", &client.from_phone);

    let params = [
        ("To", to.as_str()),
        ("From", from.as_str()),
        ("Body", message),
    ];

    let http = Client::new();
    let res = http
        .post(&client.base_url())
        .basic_auth(&client.account_sid, Some(&client.auth_token))
        .form(&params)
        .send()
        .await?
        .json::<TwilioMessageResponse>()
        .await?;

    Ok(res)
}

pub async fn send_whatsapp_media(
    client: &TwilioClient,
    to: &str,
    message: &str,
    media_url: &str,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let to = format!("whatsapp:{}", to);
    let from = format!("whatsapp:{}", &client.from_phone);

    let params = [
        ("To", to.as_str()),
        ("From", from.as_str()),
        ("Body", message),
        ("MediaUrl", media_url),
    ];

    let http = Client::new();
    let res = http
        .post(&client.base_url())
        .basic_auth(&client.account_sid, Some(&client.auth_token))
        .form(&params)
        .send()
        .await?
        .json::<TwilioMessageResponse>()
        .await?;

    Ok(res)
}

pub async fn send_whatsapp_template(
    client: &TwilioClient,
    to: &str,
    template_name: &str,
    lang: &str,
    components_json: &str,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let to = format!("whatsapp:{}", to);
    let from = format!("whatsapp:{}", &client.from_phone);

    let params = [
        ("To", to.as_str()),
        ("From", from.as_str()),
        ("MessagingServiceSid", ""),
        ("ContentSid", template_name),
        ("ContentVariables", components_json), // example: "{\"1\":\"John\",\"2\":\"Tuesday\"}"
    ];

    let http = Client::new();
    let res = http
        .post(&client.base_url())
        .basic_auth(&client.account_sid, Some(&client.auth_token))
        .form(&params)
        .send()
        .await?
        .json::<TwilioMessageResponse>()
        .await?;

    Ok(res)
}
