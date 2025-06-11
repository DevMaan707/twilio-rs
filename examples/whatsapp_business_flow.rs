use twilio_rs::{
    client::TwilioClient,
    whatsapp::{
        send_whatsapp_appointment_reminder, send_whatsapp_interactive_buttons,
        send_whatsapp_order_status, send_whatsapp_text, send_whatsapp_upi_payment,
        InteractiveButton,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TwilioClient::new();
    let phone = "+917569785621"; // Replace with your phone number

    println!("🚀 Starting complete business flow demo...");

    // Step 1: Business Introduction
    match send_whatsapp_text(
        &client,
        phone,
        "👋 **Welcome to Aymaan's Tech Studio!**\n\nHi there! I'm Aymaan, a full-stack developer specializing in:\n\n🌐 **Web Development**\n• React, Node.js, Python\n• E-commerce solutions\n• Custom web applications\n\n📱 **Mobile Apps**\n• React Native\n• iOS & Android\n• Cross-platform solutions\n\n☁️ **Cloud Services**\n• AWS, Google Cloud\n• DevOps & CI/CD\n• Database optimization\n\n💡 Ready to turn your ideas into reality? Let's chat about your project!",
    )
    .await
    {
        Ok(response) => println!("✅ Business intro sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Step 2: Service Selection
    let service_buttons = vec![
        InteractiveButton {
            id: "web_project".to_string(),
            title: "🌐 Web Development".to_string(),
        },
        InteractiveButton {
            id: "mobile_project".to_string(),
            title: "📱 Mobile App".to_string(),
        },
        InteractiveButton {
            id: "consultation".to_string(),
            title: "💬 Free Consultation".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
        &client,
        phone,
        "🎯 Let's Get Started",
        "What type of project are you looking to build?\n\n💰 **Starting from:**\nWhat type of project are you looking to build?\n\n💰 **Starting from:**\n🌐 Websites: ₹15,000\n📱 Mobile Apps: ₹25,000\n💬 Consultation: FREE\n\n🚀 All projects include:\n• Modern design\n• Mobile responsive\n• SEO optimized\n• 3 months support",
                Some("Click below to discuss your project"),
                service_buttons,
            )
            .await
            {
                Ok(response) => println!("✅ Service selection sent! SID: {}", response.sid),
                Err(e) => eprintln!("❌ Error: {}", e),
            }

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Step 3: Project Proposal & Quote
    match send_whatsapp_text(
                &client,
                phone,
                "📋 **Project Proposal - E-commerce Website**\n\n🎯 **Scope of Work:**\n• Modern responsive design\n• Product catalog (up to 100 products)\n• Shopping cart & checkout\n• Payment gateway integration\n• Admin dashboard\n• SEO optimization\n• Mobile app integration ready\n\n💰 **Investment:** ₹45,000\n⏱️ **Timeline:** 3-4 weeks\n🎁 **Bonus:** Free logo design + 6 months hosting\n\n📅 Ready to start? We can begin this week!",
            )
            .await
            {
                Ok(response) => println!("✅ Project proposal sent! SID: {}", response.sid),
                Err(e) => eprintln!("❌ Error: {}", e),
            }

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Step 4: Payment Request
    match send_whatsapp_upi_payment(
        &client,
        phone,
        "aymaan.dev@paytm", // Replace with actual UPI ID
        22500.0,            // 50% advance
        "E-commerce Website Development - Advance Payment (50%)",
        "Aymaan's Tech Studio",
    )
    .await
    {
        Ok(response) => println!("✅ Payment request sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Step 5: Project Kickoff Meeting
    match send_whatsapp_appointment_reminder(
        &client,
        phone,
        "Tomorrow (March 16, 2024)",
        "11:00 AM - 12:00 PM",
        "Google Meet (Link will be shared 30 mins before)",
        None,
    )
    .await
    {
        Ok(response) => println!("✅ Meeting reminder sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Step 6: Project Progress Update
    let progress_buttons = vec![
        InteractiveButton {
            id: "view_demo".to_string(),
            title: "👀 View Demo".to_string(),
        },
        InteractiveButton {
            id: "request_changes".to_string(),
            title: "✏️ Request Changes".to_string(),
        },
        InteractiveButton {
            id: "approve_design".to_string(),
            title: "✅ Approve Design".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
                &client,
                phone,
                "🎨 Design Phase Complete!",
                "Great news! The homepage and product catalog designs are ready! 🎉\n\n✅ **Completed:**\n• Homepage design\n• Product listing page\n• Product detail page\n• Shopping cart design\n• Responsive mobile version\n\n🔗 **Demo Link:** https://demo.aymaan.dev/yourstore\n\n📱 **Test on mobile too!** The design adapts beautifully to all screen sizes.\n\nWhat would you like to do next?",
                Some("Your feedback helps us perfect the design"),
                progress_buttons,
            )
            .await
            {
                Ok(response) => println!("✅ Progress update sent! SID: {}", response.sid),
                Err(e) => eprintln!("❌ Error: {}", e),
            }

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    // Step 7: Project Delivery
    match send_whatsapp_text(
                &client,
                phone,
                "🎉 **PROJECT DELIVERED!** 🚀\n\nYour e-commerce website is now LIVE! 🌟\n\n🌐 **Your Website:** https://yourstore.com\n🔐 **Admin Panel:** https://yourstore.com/admin\n📱 **Mobile App Ready:** API endpoints configured\n\n📦 **What You Get:**\n✅ Complete source code\n✅ Admin credentials\n✅ Documentation\n✅ 3 months free support\n✅ Free SSL certificate\n✅ SEO optimization\n\n📈 **Next Steps:**\n• Add your products\n• Configure payment methods\n• Set up shipping options\n• Launch marketing campaigns\n\n🎁 **BONUS:** I'll help you add the first 10 products for FREE!\n\nCongratulations on your new online store! 🛍️",
            )
            .await
            {
                Ok(response) => println!("✅ Project delivery sent! SID: {}", response.sid),
                Err(e) => eprintln!("❌ Error: {}", e),
            }

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Step 8: Final Payment Request
    match send_whatsapp_upi_payment(
        &client,
        phone,
        "aymaan.dev@paytm",
        22500.0, // Remaining 50%
        "E-commerce Website Development - Final Payment (50%)",
        "Aymaan's Tech Studio",
    )
    .await
    {
        Ok(response) => println!("✅ Final payment request sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Step 9: Follow-up & Future Services
    let followup_buttons = vec![
        InteractiveButton {
            id: "mobile_app".to_string(),
            title: "📱 Build Mobile App".to_string(),
        },
        InteractiveButton {
            id: "marketing".to_string(),
            title: "📈 Digital Marketing".to_string(),
        },
        InteractiveButton {
            id: "maintenance".to_string(),
            title: "🔧 Maintenance Plan".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
                &client,
                phone,
                "🚀 Ready to Scale Your Business?",
                "Your website is performing great! 📈\n\n📊 **30-Day Stats:**\n• 1,247 unique visitors\n• 89 orders placed\n• ₹67,500 revenue\n• 4.8/5 customer rating\n\n🎯 **Ready for the next level?**\n\n📱 **Mobile App** - Increase sales by 40%\n📈 **Digital Marketing** - 3x your traffic\n🔧 **Maintenance Plan** - Keep everything running smooth\n\nWhich service interests you most?",
                Some("Let's grow your business together! 💪"),
                followup_buttons,
            )
            .await
            {
                Ok(response) => println!("✅ Follow-up sent! SID: {}", response.sid),
                Err(e) => eprintln!("❌ Error: {}", e),
            }

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Step 10: Customer Success & Testimonial Request
    let testimonial_buttons = vec![
        InteractiveButton {
            id: "write_review".to_string(),
            title: "⭐ Write Review".to_string(),
        },
        InteractiveButton {
            id: "refer_friend".to_string(),
            title: "👥 Refer a Friend".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
                &client,
                phone,
                "🙏 Thank You for Trusting Us!",
                "It's been amazing working with you! 🌟\n\nSeeing your business grow and succeed makes all our hard work worthwhile. Your e-commerce store is now generating consistent revenue and happy customers!\n\n💝 **Your Success = Our Success**\n\nWould you mind sharing your experience? Your testimonial helps other entrepreneurs discover our services and achieve similar success.\n\n🎁 **Referral Bonus:** Get 20% off your next project for each successful referral!",
                Some("Your success story inspires others! ✨"),
                testimonial_buttons,
            )
            .await
            {
                Ok(response) => println!("✅ Testimonial request sent! SID: {}", response.sid),
                Err(e) => eprintln!("❌ Error: {}", e),
            }

    println!("\n🎉 **COMPLETE BUSINESS FLOW DEMO FINISHED!** 🎉");
    println!("📈 This demo showcased:");
    println!("   • Business introduction & service offering");
    println!("   • Interactive service selection");
    println!("   • Project proposal & quotation");
    println!("   • Payment requests (advance & final)");
    println!("   • Appointment scheduling");
    println!("   • Progress updates with client interaction");
    println!("   • Project delivery notification");
    println!("   • Upselling & cross-selling");
    println!("   • Customer success & testimonial requests");
    println!("\n✨ All messages sent successfully! Check your WhatsApp!");

    Ok(())
}
