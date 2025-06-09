use twilio_rs::{client::TwilioClient, whatsapp::send_whatsapp_text};

#[tokio::main]
async fn main() {
    let client = TwilioClient::new();

    match send_whatsapp_text(&client, "+1234567890", "Hello from WhatsApp!").await {
        Ok(response) => println!("WhatsApp message sent! SID: {}", response.sid),
        Err(e) => eprintln!("Error sending message: {}", e),
    }
}
