use crate::client::TwilioClient;
use crate::whatsapp::send_whatsapp_text;
use qrcode::render::unicode;
use qrcode::QrCode;
use std::error::Error;

#[derive(Debug)]
pub enum UpiApp {
    GooglePay,
    PhonePe,
    Paytm,
    Any,
}

impl UpiApp {
    pub fn to_scheme(&self) -> &'static str {
        match self {
            UpiApp::GooglePay => "tez://upi/pay",
            UpiApp::PhonePe => "phonepe://upi/pay",
            UpiApp::Paytm => "paytmmp://pay",
            UpiApp::Any => "upi://pay",
        }
    }
}

pub fn generate_upi_link(vpa: &str, name: &str, amount: f64, app: UpiApp) -> String {
    let base = app.to_scheme();
    format!(
        "{}?pa={}&pn={}&am={:.2}&cu=INR",
        base,
        urlencoding::encode(vpa),
        urlencoding::encode(name),
        amount
    )
}

pub fn generate_upi_qr(link: &str) -> Result<String, Box<dyn Error>> {
    let code = QrCode::new(link.as_bytes())?;
    let image = code.render::<unicode::Dense1x2>().build();
    Ok(image)
}

/// Sends a WhatsApp message with a UPI payment link and optional QR code
///
/// # Arguments
/// * `client` - Twilio client
/// * `to` - WhatsApp recipient (phone number without whatsapp: prefix)
/// * `vpa` - UPI ID (e.g., someone@upi)
/// * `name` - Payee name
/// * `amount` - Amount in INR
/// * `app` - Preferred UPI app
/// * `custom_message` - Optional custom message formatter
/// * `include_qr` - Whether to include a QR code
///
/// # Returns
/// * Result<(), Box<dyn Error>>
///
pub async fn send_upi_payment_request(
    client: &TwilioClient,
    to: &str,
    vpa: &str,
    name: &str,
    amount: f64,
    app: UpiApp,
    custom_message: Option<&dyn Fn(&str, f64, &str) -> String>,
    include_qr: bool,
) -> Result<(), Box<dyn Error>> {
    let link = generate_upi_link(vpa, name, amount, app);

    let msg = if let Some(format_fn) = custom_message {
        format_fn(name, amount, &link)
    } else {
        format!(
            "Hi {},\nPlease complete the payment of ₹{:.2} using the UPI link below:\n{}",
            name, amount, link
        )
    };

    let final_msg = if include_qr {
        match generate_upi_qr(&link) {
            Ok(qr) => format!("{}\n\nScan this QR:\n{}", msg, qr),
            Err(_) => msg,
        }
    } else {
        msg
    };

    send_whatsapp_text(client, to, &final_msg)
        .await
        .map(|_| ())
        .map_err(|e| e.into())
}
