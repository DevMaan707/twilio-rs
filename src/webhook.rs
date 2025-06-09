// webhook.rs

use crate::client::TwilioClient;
use crate::signature::validate_twilio_signature;
use crate::whatsapp::send_whatsapp_text;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder, Scope};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct IncomingWhatsAppMessage {
    #[serde(rename = "From")]
    pub from: String,
    #[serde(rename = "To")]
    pub to: String,
    #[serde(rename = "Body")]
    pub body: String,
    #[serde(rename = "MessageSid")]
    pub message_sid: String,
    #[serde(rename = "ProfileName")]
    pub profile_name: Option<String>,
    #[serde(rename = "WaId")]
    pub wa_id: Option<String>,
}

/// Internal handler that allows an optional auto-reply handler
#[post("")]
async fn internal_webhook_handler(
    req: HttpRequest,
    form: web::Form<HashMap<String, String>>,
    base_url: web::Data<String>,
    auto_reply_handler: web::Data<Option<Arc<dyn Fn(String, String) -> String + Send + Sync>>>,
) -> impl Responder {
    let data = form.into_inner();
    let signature = req
        .headers()
        .get("X-Twilio-Signature")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let auth_token = std::env::var("TWILIO_AUTH_TOKEN").unwrap_or_default();

    if !validate_twilio_signature(base_url.get_ref(), &data, signature, &auth_token) {
        return HttpResponse::Unauthorized().body("Invalid Twilio signature.");
    }

    let msg = IncomingWhatsAppMessage {
        from: data.get("From").cloned().unwrap_or_default(),
        to: data.get("To").cloned().unwrap_or_default(),
        body: data.get("Body").cloned().unwrap_or_default(),
        message_sid: data.get("MessageSid").cloned().unwrap_or_default(),
        profile_name: data.get("ProfileName").cloned(),
        wa_id: data.get("WaId").cloned(),
    };

    println!("✅ Verified Incoming Message: {:?}", msg);

    if let Some(handler) = auto_reply_handler.get_ref().as_ref() {
        let reply = (handler)(msg.from.clone(), msg.body.clone());
        let client = TwilioClient::new();
        let _ = send_whatsapp_text(&client, &msg.from.replace("whatsapp:", ""), &reply).await;
    }

    HttpResponse::Ok().finish()
}

/// Public function to mount the WhatsApp webhook scope
pub fn build_whatsapp_webhook_scope(
    base_url: &str,
    auto_reply_handler: Option<Arc<dyn Fn(String, String) -> String + Send + Sync>>,
) -> Scope {
    web::scope("/twilio/whatsapp")
        .app_data(web::Data::new(base_url.to_string()))
        .app_data(web::Data::new(auto_reply_handler))
        .service(internal_webhook_handler)
}
