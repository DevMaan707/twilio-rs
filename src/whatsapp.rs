use crate::client::TwilioClient;
use reqwest::Client;
use serde::{Deserialize, Deserializer, Serialize};
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct TwilioMessageResponse {
    pub sid: String,
    pub status: String,
    pub to: String,
    pub from: String,
    pub body: Option<String>,
    pub account_sid: String,
    pub api_version: String,
    pub date_created: Option<String>,
    pub date_sent: Option<String>,
    pub date_updated: Option<String>,
    pub direction: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub messaging_service_sid: Option<String>,
    #[serde(deserialize_with = "string_or_int")]
    pub num_media: String,
    #[serde(deserialize_with = "string_or_int")]
    pub num_segments: String,
    pub price: Option<String>,
    pub price_unit: Option<String>,
    pub uri: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct InteractiveButton {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Debug)]
pub struct InteractiveListRow {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct InteractiveListSection {
    pub title: Option<String>,
    pub rows: Vec<InteractiveListRow>,
}

#[derive(Serialize, Debug)]
pub struct PaymentRequest {
    pub amount: f64,
    pub currency: String,
    pub description: String,
    pub reference_id: String,
}

#[derive(Serialize, Debug)]
pub struct ReminderMessage {
    pub title: String,
    pub body: String,
    pub reminder_time: Option<String>,
    pub action_buttons: Option<Vec<InteractiveButton>>,
}

fn string_or_int<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::{self, Visitor};
    use std::fmt;

    struct StringOrIntVisitor;

    impl<'de> Visitor<'de> for StringOrIntVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or integer")
        }

        fn visit_str<E>(self, value: &str) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_u64<E>(self, value: u64) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }

        fn visit_i64<E>(self, value: i64) -> Result<String, E>
        where
            E: de::Error,
        {
            Ok(value.to_string())
        }
    }

    deserializer.deserialize_any(StringOrIntVisitor)
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
    let response = http
        .post(&client.base_url())
        .basic_auth(&client.account_sid, Some(&client.auth_token))
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(format!("Twilio API error {}: {}", status, text).into());
    }

    let res = response.json::<TwilioMessageResponse>().await?;
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
    let response = http
        .post(&client.base_url())
        .basic_auth(&client.account_sid, Some(&client.auth_token))
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(format!("Twilio API error {}: {}", status, text).into());
    }

    let res = response.json::<TwilioMessageResponse>().await?;
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
        ("ContentVariables", components_json),
    ];

    let http = Client::new();
    let response = http
        .post(&client.base_url())
        .basic_auth(&client.account_sid, Some(&client.auth_token))
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(format!("Twilio API error {}: {}", status, text).into());
    }

    let res = response.json::<TwilioMessageResponse>().await?;
    Ok(res)
}

pub async fn send_whatsapp_interactive_buttons(
    client: &TwilioClient,
    to: &str,
    header_text: &str,
    body_text: &str,
    footer_text: Option<&str>,
    buttons: Vec<InteractiveButton>,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    if buttons.len() > 3 {
        return Err("WhatsApp interactive messages support maximum 3 buttons".into());
    }

    let to = format!("whatsapp:{}", to);
    let from = format!("whatsapp:{}", &client.from_phone);

    let mut interactive_json = serde_json::json!({
        "type": "button",
        "header": {
            "type": "text",
            "text": header_text
        },
        "body": {
            "text": body_text
        },
        "action": {
            "buttons": buttons.iter().map(|btn| {
                serde_json::json!({
                    "type": "reply",
                    "reply": {
                        "id": btn.id,
                        "title": btn.title
                    }
                })
            }).collect::<Vec<_>>()
        }
    });

    if let Some(footer) = footer_text {
        interactive_json["footer"] = serde_json::json!({
            "text": footer
        });
    }

    // Use form data instead of JSON payload
    let params = [
        ("To", to.as_str()),
        ("From", from.as_str()),
        ("Body", body_text), // Use body_text instead of empty string
        ("Interactive", &interactive_json.to_string()),
    ];

    let http = Client::new();
    let response = http
        .post(&client.base_url())
        .basic_auth(&client.account_sid, Some(&client.auth_token))
        .form(&params) // Use form instead of json
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(format!("Twilio API error {}: {}", status, text).into());
    }

    let res = response.json::<TwilioMessageResponse>().await?;
    Ok(res)
}
pub async fn send_whatsapp_interactive_list(
    client: &TwilioClient,
    to: &str,
    header_text: &str,
    body_text: &str,
    footer_text: Option<&str>,
    button_text: &str,
    sections: Vec<InteractiveListSection>,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let to = format!("whatsapp:{}", to);
    let from = format!("whatsapp:{}", &client.from_phone);

    let mut interactive_json = serde_json::json!({
        "type": "list",
        "header": {
            "type": "text",
            "text": header_text
        },
        "body": {
            "text": body_text
        },
        "action": {
            "button": button_text,
            "sections": sections.iter().map(|section| {
                let mut section_json = serde_json::json!({
                    "rows": section.rows.iter().map(|row| {
                        let mut row_json = serde_json::json!({
                            "id": row.id,
                            "title": row.title
                        });
                        if let Some(desc) = &row.description {
                            row_json["description"] = serde_json::Value::String(desc.clone());
                        }
                        row_json
                    }).collect::<Vec<_>>()
                });

                if let Some(title) = &section.title {
                    section_json["title"] = serde_json::Value::String(title.clone());
                }
                section_json
            }).collect::<Vec<_>>()
        }
    });

    if let Some(footer) = footer_text {
        interactive_json["footer"] = serde_json::json!({
            "text": footer
        });
    }

    let params = [
        ("To", to.as_str()),
        ("From", from.as_str()),
        ("Body", body_text), // Use body_text instead of empty string
        ("Interactive", &interactive_json.to_string()),
    ];

    let http = Client::new();
    let response = http
        .post(&client.base_url())
        .basic_auth(&client.account_sid, Some(&client.auth_token))
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(format!("Twilio API error {}: {}", status, text).into());
    }

    let res = response.json::<TwilioMessageResponse>().await?;
    Ok(res)
}
pub async fn send_whatsapp_payment_request(
    client: &TwilioClient,
    to: &str,
    payment_request: PaymentRequest,
    custom_message: Option<&str>,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let template = &format!("💳 Payment Request\n\nAmount: {} {}\nDescription: {}\nReference: {}\n\nPlease complete your payment to proceed.",payment_request.currency,
    payment_request.amount,
    payment_request.description,
    payment_request.reference_id);
    let message = custom_message.unwrap_or(template);
    send_whatsapp_text(client, to, message).await
}
pub async fn send_whatsapp_upi_payment(
    client: &TwilioClient,
    to: &str,
    upi_id: &str,
    amount: f64,
    description: &str,
    merchant_name: &str,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let upi_link = format!(
        "upi://pay?pa={}&pn={}&am={:.2}&cu=INR&tn={}",
        urlencoding::encode(upi_id),
        urlencoding::encode(merchant_name),
        amount,
        urlencoding::encode(description)
    );

    let message = format!(
        "💰 Payment Request\n\n• Amount: ₹{:.2}\n• To: {}\n• Description: {}\n\n🔗 Pay using UPI:\n{}\n\nOr scan the QR code if available.",
        amount,
        merchant_name,
        description,
        upi_link
    );

    send_whatsapp_text(client, to, &message).await
}
pub async fn send_whatsapp_content_template(
    client: &TwilioClient,
    to: &str,
    content_sid: &str,
    content_variables: Option<&str>,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let to = format!("whatsapp:{}", to);
    let from = format!("whatsapp:{}", &client.from_phone);

    let mut params = vec![
        ("To", to.as_str()),
        ("From", from.as_str()),
        ("ContentSid", content_sid),
    ];

    if let Some(variables) = content_variables {
        params.push(("ContentVariables", variables));
    }

    let http = Client::new();
    let response = http
        .post(&client.base_url())
        .basic_auth(&client.account_sid, Some(&client.auth_token))
        .form(&params)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await?;
        return Err(format!("Twilio API error {}: {}", status, text).into());
    }

    let res = response.json::<TwilioMessageResponse>().await?;
    Ok(res)
}
pub async fn send_whatsapp_reminder(
    client: &TwilioClient,
    to: &str,
    reminder: ReminderMessage,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let mut message = format!("🔔 Reminder: {}\n\n{}", reminder.title, reminder.body);

    if let Some(time) = &reminder.reminder_time {
        message.push_str(&format!("\n⏰ Scheduled for: {}", time));
    }

    // If there are action buttons, send as interactive message
    if let Some(buttons) = reminder.action_buttons {
        return send_whatsapp_interactive_buttons(
            client,
            to,
            &format!("🔔 {}", reminder.title),
            &reminder.body,
            reminder.reminder_time.as_deref(),
            buttons,
        )
        .await;
    }
    send_whatsapp_text(client, to, &message).await
}

pub async fn send_whatsapp_quick_replies(
    client: &TwilioClient,
    to: &str,
    message: &str,
    choices: Vec<&str>,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    if choices.len() > 3 {
        // If more than 3 choices, use list format
        let sections = vec![InteractiveListSection {
            title: Some("Choose an option".to_string()),
            rows: choices
                .into_iter()
                .enumerate()
                .map(|(i, choice)| InteractiveListRow {
                    id: format!("choice_{}", i),
                    title: choice.to_string(),
                    description: None,
                })
                .collect(),
        }];

        return send_whatsapp_interactive_list(
            client,
            to,
            "Please select an option",
            message,
            None,
            "Choose",
            sections,
        )
        .await;
    } else {
        // Use buttons for 3 or fewer choices
        let buttons = choices
            .into_iter()
            .enumerate()
            .map(|(i, choice)| InteractiveButton {
                id: format!("choice_{}", i),
                title: choice.to_string(),
            })
            .collect();

        return send_whatsapp_interactive_buttons(
            client,
            to,
            "Please select an option",
            message,
            None,
            buttons,
        )
        .await;
    }
}

// NEW: Appointment Reminder
pub async fn send_whatsapp_appointment_reminder(
    client: &TwilioClient,
    to: &str,
    appointment_date: &str,
    appointment_time: &str,
    location: &str,
    doctor_name: Option<&str>,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let doctor_info = doctor_name
        .map(|name| format!("👨‍⚕️ With: Dr. {}\n", name))
        .unwrap_or_default();

    let buttons = vec![
        InteractiveButton {
            id: "confirm".to_string(),
            title: "✅ Confirm".to_string(),
        },
        InteractiveButton {
            id: "reschedule".to_string(),
            title: "📅 Reschedule".to_string(),
        },
        InteractiveButton {
            id: "cancel".to_string(),
            title: "❌ Cancel".to_string(),
        },
    ];

    let body = format!(
        "📅 Date: {}\n⏰ Time: {}\n📍 Location: {}\n{}",
        appointment_date, appointment_time, location, doctor_info
    );

    send_whatsapp_interactive_buttons(
        client,
        to,
        "🩺 Appointment Reminder",
        &body,
        Some("Please confirm your appointment"),
        buttons,
    )
    .await
}

// NEW: Order Status Messages
pub async fn send_whatsapp_order_status(
    client: &TwilioClient,
    to: &str,
    order_id: &str,
    status: &str,
    tracking_url: Option<&str>,
    estimated_delivery: Option<&str>,
) -> Result<TwilioMessageResponse, Box<dyn Error>> {
    let mut message = format!(
        "📦 Order Update\n\nOrder ID: {}\nStatus: {}",
        order_id, status
    );

    if let Some(delivery) = estimated_delivery {
        message.push_str(&format!("\n🚚 Estimated Delivery: {}", delivery));
    }

    if let Some(url) = tracking_url {
        message.push_str(&format!("\n\n🔗 Track your order: {}", url));
    }

    let buttons = vec![
        InteractiveButton {
            id: "track_order".to_string(),
            title: "📍 Track Order".to_string(),
        },
        InteractiveButton {
            id: "contact_support".to_string(),
            title: "💬 Support".to_string(),
        },
    ];

    send_whatsapp_interactive_buttons(
        client,
        to,
        "📦 Order Status Update",
        &message,
        Some("Need help with your order?"),
        buttons,
    )
    .await
}
