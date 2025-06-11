use twilio_rs::{client::TwilioClient, whatsapp::send_whatsapp_text};

#[tokio::main]
async fn main() {
    let client = TwilioClient::new();

    match send_whatsapp_text(&client, "+917569785621", "Hello from Aymaan!").await {
        Ok(response) => println!("WhatsApp message sent! SID: {}", response.sid),
        Err(e) => eprintln!("Error sending message: {}", e),
    }
}
