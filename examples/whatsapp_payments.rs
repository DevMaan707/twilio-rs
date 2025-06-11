use twilio_rs::{
    client::TwilioClient,
    whatsapp::{send_whatsapp_payment_request, send_whatsapp_upi_payment, PaymentRequest},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TwilioClient::new();
    let phone = "+917569785621"; // Replace with your phone number

    println!("💳 Sending payment request message...");

    // Example 1: Generic Payment Request
    let payment_request = PaymentRequest {
        amount: 1299.0,
        currency: "INR".to_string(),
        description: "Web Development Service - Homepage Design".to_string(),
        reference_id: "INV-2024-001".to_string(),
    };

    match send_whatsapp_payment_request(
        &client,
        phone,
        payment_request,
        Some("🚀 Invoice Ready!\n\nHi there! Your website homepage design is complete. Please make the payment to proceed with hosting setup.\n\nPayment Details:"),
    )
    .await
    {
        Ok(response) => println!("✅ Payment request sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("💰 Sending UPI payment message...");

    // Example 2: UPI Payment (India-specific)
    match send_whatsapp_upi_payment(
        &client,
        phone,
        "aymaan@paytm", // Replace with actual UPI ID
        599.0,
        "Monthly Subscription - Premium Plan",
        "Aymaan's Tech Services",
    )
    .await
    {
        Ok(response) => println!("✅ UPI payment message sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("🛒 Sending e-commerce payment message...");

    // Example 3: E-commerce Order Payment
    let ecommerce_payment = PaymentRequest {
        amount: 2499.0,
        currency: "INR".to_string(),
        description: "iPhone Case + Screen Protector + Wireless Charger".to_string(),
        reference_id: "ORD-EC-789".to_string(),
    };

    let custom_ecommerce_message = format!(
        "🛍️ **Order Confirmation**\n\n📱 Your cart is ready for checkout!\n\n**Items:**\n• iPhone 14 Pro Case - ₹999\n• Tempered Glass Protector - ₹299\n• Wireless Charger - ₹1,201\n\n**Order Summary:**\nSubtotal: ₹2,499\nShipping: FREE 🚚\nTotal: **₹2,499**\n\n💳 **Payment Details:**\nAmount: {} {}\nOrder ID: {}\n\n✨ Complete payment to get **FREE express delivery**!"
    , ecommerce_payment.currency, ecommerce_payment.amount, ecommerce_payment.reference_id);

    match send_whatsapp_payment_request(
        &client,
        phone,
        ecommerce_payment,
        Some(&custom_ecommerce_message),
    )
    .await
    {
        Ok(response) => println!("✅ E-commerce payment sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    println!("💳 All payment messages sent successfully!");
    Ok(())
}
