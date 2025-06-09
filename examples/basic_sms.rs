use twilio_rs::client::TwilioClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Create Twilio client
    let client = TwilioClient::new();

    // Send SMS message
    let result = client
        .send_sms(
            "+1234567890", // Replace with actual phone number
            "Hello from Rust Twilio SDK!",
        )
        .await?;

    println!("Message sent successfully!");
    println!("Message SID: {}", result.sid);
    println!("Status: {}", result.status);

    Ok(())
}
