// lib.rs

pub mod client;
pub mod payments;
pub mod signature;
pub mod sms;
pub mod webhook;
pub mod whatsapp;
use actix_web::Scope;
use std::sync::Arc;

/// Re-export essential types and functions
pub use webhook::{build_whatsapp_webhook_scope, IncomingWhatsAppMessage};

/// Type alias for auto reply handler for convenience
pub type AutoReplyHandler = Arc<dyn Fn(String, String) -> String + Send + Sync>;

/// Function to create Twilio WhatsApp webhook scope with optional auto-reply
///
/// # Example
/// ```
/// use my_twilio_lib::{build_twilio_webhook, AutoReplyHandler};
/// use actix_web::{App, HttpServer};
/// use std::sync::Arc;
///
/// let handler: AutoReplyHandler = Arc::new(|from, body| format!("Reply to {}: {}", from, body));
/// let scope = build_twilio_webhook("https://myapp.com/twilio/whatsapp", Some(handler));
/// ```
pub fn build_twilio_webhook(base_url: &str, auto_reply_handler: Option<AutoReplyHandler>) -> Scope {
    build_whatsapp_webhook_scope(base_url, auto_reply_handler)
}
