use twilio_rs::{
    client::TwilioClient,
    whatsapp::{
        send_whatsapp_interactive_buttons, send_whatsapp_order_status, send_whatsapp_quick_replies,
        InteractiveButton,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TwilioClient::new();
    let phone = "+917569785621"; // Replace with your phone number

    println!("📦 Sending order confirmation...");

    // Example 1: Order Placed Status
    match send_whatsapp_order_status(
        &client,
        phone,
        "ORD-2024-001234",
        "✅ Order Confirmed & Being Prepared",
        Some("https://track.mystore.com/ORD-2024-001234"),
        Some("March 18, 2024 by 6:00 PM"),
    )
    .await
    {
        Ok(response) => println!("✅ Order confirmation sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("🚚 Sending shipping update...");

    // Example 2: Order Shipped Status
    match send_whatsapp_order_status(
        &client,
        phone,
        "ORD-2024-001234",
        "🚚 Shipped - Out for Delivery",
        Some("https://track.bluedart.com/BD789456123"),
        Some("Today by 7:00 PM"),
    )
    .await
    {
        Ok(response) => println!("✅ Shipping update sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("🛍️ Sending product recommendation...");

    // Example 3: Product Recommendation with Quick Replies
    let product_choices = vec![
        "📱 View Similar Products",
        "⭐ See Reviews",
        "💰 Check Offers",
        "🚚 Delivery Options",
        "❌ Not Interested",
    ];

    match send_whatsapp_quick_replies(
        &client,
        phone,
        "🎉 Great choice! Since you loved the iPhone case, you might also like:\n\n📱 iPhone 14 Pro MagSafe Wireless Charger - ₹2,999\n✨ Features:\n• Fast 15W charging\n• MagSafe compatible\n• Premium aluminum build\n• 1-year warranty\n\n🔥 Special offer: 20% off if ordered with your recent purchase!",
        product_choices,
    )
    .await
    {
        Ok(response) => println!("✅ Product recommendation sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("📝 Sending feedback request...");

    // Example 4: Post-Delivery Feedback
    let feedback_buttons = vec![
        InteractiveButton {
            id: "rate_5".to_string(),
            title: "⭐⭐⭐⭐⭐ Excellent".to_string(),
        },
        InteractiveButton {
            id: "rate_4".to_string(),
            title: "⭐⭐⭐⭐ Good".to_string(),
        },
        InteractiveButton {
            id: "rate_low".to_string(),
            title: "⭐⭐⭐ Needs Improvement".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
        &client,
        phone,
        "📦 Order Delivered Successfully!",
        "Hi! Your order #ORD-2024-001234 has been delivered.\n\nWe hope you love your new iPhone case! 📱✨\n\nHow was your shopping experience with us?",
        Some("Your feedback helps us serve you better 💝"),
        feedback_buttons,
    )
    .await
    {
        Ok(response) => println!("✅ Feedback request sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("🎁 Sending loyalty program message...");

    // Example 5: Loyalty Program Notification
    let loyalty_buttons = vec![
        InteractiveButton {
            id: "view_rewards".to_string(),
            title: "🎁 View My Rewards".to_string(),
        },
        InteractiveButton {
            id: "redeem_points".to_string(),
            title: "✨ Redeem Points".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
        &client,
        phone,
        "🎉 Congratulations! VIP Status Unlocked",
        "Amazing news! You've earned 2,500 reward points and unlocked VIP status! 🌟\n\n💎 Your Benefits:\n• Free delivery on all orders\n• Early access to sales\n• Exclusive member discounts\n• Priority customer support\n\n🎁 Current Points: 2,500\n💰 Points Value: ₹250",
        Some("Points expire in 365 days"),
        loyalty_buttons,
    )
    .await
    {
        Ok(response) => println!("✅ Loyalty program message sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    println!("🛍️ All e-commerce messages sent successfully!");
    Ok(())
}
