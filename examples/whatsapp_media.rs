use twilio_rs::{client::TwilioClient, whatsapp::send_whatsapp_media};

#[tokio::main]
async fn main() {
    let client = TwilioClient::new();
    let media_url = "https://example.com/image.jpg";

    match send_whatsapp_media(&client, "+1234567890", "Check out this image!", media_url).await {
        Ok(response) => println!("Media message sent! SID: {}", response.sid),
        Err(e) => eprintln!("Error sending message: {}", e),
    }
}
