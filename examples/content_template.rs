use twilio_rs::{client::TwilioClient, whatsapp::send_whatsapp_content_template};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TwilioClient::new();
    let phone = "+917569785621";
    match send_whatsapp_content_template(
        &client,
        phone,
        "HXb5b62575e6e4ff6129ad7c8efe1f983e",
        Some(r#"{"1":"12/1","2":"3pm"}"#),
    )
    .await
    {
        Ok(response) => println!("✅ Template sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    Ok(())
}
