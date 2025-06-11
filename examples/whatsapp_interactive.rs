use twilio_rs::{
    client::TwilioClient,
    whatsapp::{
        send_whatsapp_interactive_buttons, send_whatsapp_interactive_list, InteractiveButton,
        InteractiveListRow, InteractiveListSection,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TwilioClient::new();
    let phone = "+917569785621";

    println!("🔄 Sending interactive button message...");
    let service_buttons = vec![
        InteractiveButton {
            id: "billing".to_string(),
            title: "💳 Billing Issue".to_string(),
        },
        InteractiveButton {
            id: "technical".to_string(),
            title: "🔧 Technical Support".to_string(),
        },
        InteractiveButton {
            id: "general".to_string(),
            title: "💬 General Inquiry".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
        &client,
        phone,
        "🤖 Customer Support",
        "Hi! How can we help you today? Please select the type of support you need:",
        Some("Our team will respond within 15 minutes"),
        service_buttons,
    )
    .await
    {
        Ok(response) => println!("✅ Interactive buttons sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("🔄 Sending interactive list message...");
    let menu_sections = vec![
        InteractiveListSection {
            title: Some("🍕 Main Courses".to_string()),
            rows: vec![
                InteractiveListRow {
                    id: "pizza_margherita".to_string(),
                    title: "Margherita Pizza".to_string(),
                    description: Some("Classic tomato, mozzarella, basil - ₹299".to_string()),
                },
                InteractiveListRow {
                    id: "pasta_carbonara".to_string(),
                    title: "Pasta Carbonara".to_string(),
                    description: Some("Creamy pasta with bacon and parmesan - ₹349".to_string()),
                },
                InteractiveListRow {
                    id: "chicken_biryani".to_string(),
                    title: "Chicken Biryani".to_string(),
                    description: Some(
                        "Fragrant basmati rice with spiced chicken - ₹399".to_string(),
                    ),
                },
            ],
        },
        InteractiveListSection {
            title: Some("🥤 Beverages".to_string()),
            rows: vec![
                InteractiveListRow {
                    id: "fresh_lime".to_string(),
                    title: "Fresh Lime Soda".to_string(),
                    description: Some("Refreshing lime with soda - ₹79".to_string()),
                },
                InteractiveListRow {
                    id: "mango_lassi".to_string(),
                    title: "Mango Lassi".to_string(),
                    description: Some("Creamy yogurt drink with mango - ₹99".to_string()),
                },
            ],
        },
    ];

    match send_whatsapp_interactive_list(
        &client,
        phone,
        "🍽️ Aymaan's Kitchen",
        "Welcome to our restaurant! Browse our delicious menu and place your order:",
        Some("Free delivery on orders above ₹500"),
        "View Menu",
        menu_sections,
    )
    .await
    {
        Ok(response) => println!("✅ Interactive list sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    println!("🎉 All interactive messages sent successfully!");
    Ok(())
}
