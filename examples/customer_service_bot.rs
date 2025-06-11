use twilio_rs::{
    client::TwilioClient,
    whatsapp::{
        send_whatsapp_interactive_buttons, send_whatsapp_interactive_list,
        send_whatsapp_quick_replies, InteractiveButton, InteractiveListRow, InteractiveListSection,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TwilioClient::new();
    let phone = "+917569785621"; // Replace with your phone number

    println!("🤖 Sending welcome message...");

    // Example 1: Welcome & Main Menu
    let main_menu_buttons = vec![
        InteractiveButton {
            id: "new_account".to_string(),
            title: "🆕 Open Account".to_string(),
        },
        InteractiveButton {
            id: "existing_customer".to_string(),
            title: "👤 Existing Customer".to_string(),
        },
        InteractiveButton {
            id: "general_info".to_string(),
            title: "ℹ️ General Info".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
        &client,
        phone,
        "🏦 Welcome to Aymaan Bank",
        "Hello! Welcome to our 24/7 WhatsApp Banking Service! 🌟\n\nI'm your virtual assistant, ready to help with:\n• Account services\n• Loan inquiries\n• Investment options\n• General banking info\n\nHow can I assist you today?",
        Some("Secure • Fast • Available 24/7"),
        main_menu_buttons,
    )
    .await
    {
        Ok(response) => println!("✅ Welcome message sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("💳 Sending account services menu...");

    // Example 2: Account Services Menu
    let account_services = vec![
        InteractiveListSection {
            title: Some("💰 Account Services".to_string()),
            rows: vec![
                InteractiveListRow {
                    id: "balance_inquiry".to_string(),
                    title: "Check Balance".to_string(),
                    description: Some("View your current account balance".to_string()),
                },
                InteractiveListRow {
                    id: "mini_statement".to_string(),
                    title: "Mini Statement".to_string(),
                    description: Some("Last 5 transactions".to_string()),
                },
                InteractiveListRow {
                    id: "fund_transfer".to_string(),
                    title: "Transfer Money".to_string(),
                    description: Some("Transfer to other accounts".to_string()),
                },
            ],
        },
        InteractiveListSection {
            title: Some("💳 Card Services".to_string()),
            rows: vec![
                InteractiveListRow {
                    id: "block_card".to_string(),
                    title: "Block/Unblock Card".to_string(),
                    description: Some("Temporarily block your debit card".to_string()),
                },
                InteractiveListRow {
                    id: "card_limit".to_string(),
                    title: "Change Card Limit".to_string(),
                    description: Some("Modify daily transaction limits".to_string()),
                },
            ],
        },
        InteractiveListSection {
            title: Some("🏠 Loan Services".to_string()),
            rows: vec![
                InteractiveListRow {
                    id: "home_loan".to_string(),
                    title: "Home Loan".to_string(),
                    description: Some("Apply for home loan - Interest from 8.5%".to_string()),
                },
                InteractiveListRow {
                    id: "personal_loan".to_string(),
                    title: "Personal Loan".to_string(),
                    description: Some("Quick approval - Up to ₹50 lakhs".to_string()),
                },
            ],
        },
    ];

    match send_whatsapp_interactive_list(
        &client,
        phone,
        "🏦 Banking Services",
        "Here are all the services available for you. Select any service to proceed:",
        Some("All transactions are secure and encrypted 🔒"),
        "Select Service",
        account_services,
    )
    .await
    {
        Ok(response) => println!("✅ Account services menu sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("❓ Sending FAQ quick replies...");

    // Example 3: FAQ Quick Replies
    let faq_options = vec![
        "🕐 What are your working hours?",
        "📍 Find nearest branch",
        "💰 What are the charges?",
        "📱 How to activate mobile banking?",
        "🔒 How to reset my PIN?",
        "📞 Talk to customer care",
    ];

    match send_whatsapp_quick_replies(
        &client,
        phone,
        "🤔 **Frequently Asked Questions**\n\nChoose a topic you'd like to know more about. Our smart bot will provide instant answers!\n\n✨ Popular queries this week:",
        faq_options,
    )
    .await
    {
        Ok(response) => println!("✅ FAQ menu sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("🚨 Sending security alert...");

    // Example 4: Security Alert with Actions
    let security_buttons = vec![
        InteractiveButton {
            id: "confirm_transaction".to_string(),
            title: "✅ Yes, it's me".to_string(),
        },
        InteractiveButton {
            id: "block_transaction".to_string(),
            title: "🚨 Block & Report".to_string(),
        },
        InteractiveButton {
            id: "call_support".to_string(),
            title: "📞 Call Support".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
        &client,
        phone,
        "🔐 Security Alert",
        "We detected an unusual transaction attempt on your account:\n\n💳 Card ending: ****1234\n💰 Amount: ₹15,000\n🏪 Merchant: Amazon.in\n📍 Location: Mumbai\n⏰ Time: Just now\n\nWas this transaction made by you?",
        Some("Your security is our priority. Act immediately."),
        security_buttons,
    )
    .await
    {
        Ok(response) => println!("✅ Security alert sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("💎 Sending investment opportunity...");

    // Example 5: Investment Opportunities
    let investment_buttons = vec![
        InteractiveButton {
            id: "learn_more".to_string(),
            title: "📚 Learn More".to_string(),
        },
        InteractiveButton {
            id: "start_sip".to_string(),
            title: "🚀 Start SIP".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
        &client,
        phone,
        "💎 Wealth Building Opportunity",
        "🎯 **Mutual Fund SIP Special**\n\nStart your wealth journey with just ₹500/month!\n\n📈 **Top Performing Funds:**\n• Large Cap Fund - 12.5% returns*\n• Mid Cap Fund - 15.2% returns*\n• ELSS Tax Saver - 13.8% returns*\n\n✨ **Limited Time Benefits:**\n• Zero entry load\n• Free portfolio review\n• Tax benefits under 80C\n\n*Past performance doesn't guarantee future returns",
        Some("Start investing in 2 minutes 🚀"),
        investment_buttons,
    )
    .await
    {
        Ok(response) => println!("✅ Investment opportunity sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    println!("🤖 All customer service messages sent successfully!");
    Ok(())
}
