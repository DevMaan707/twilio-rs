
use twilio_rs::{
    client::TwilioClient,
    whatsapp::{
        send_whatsapp_interactive_buttons, send_whatsapp_text,
        InteractiveButton,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TwilioClient::new();
    let phone = "+917569785621"; // Replace with your phone number

    println!("🌍 Sending multilingual messages...");

    // Example 1: Language Selection
    let language_buttons = vec![
        InteractiveButton {
            id: "english".to_string(),
            title: "🇺🇸 English".to_string(),
        },
        InteractiveButton {
            id: "hindi".to_string(),
            title: "🇮🇳 हिंदी".to_string(),
        },
        InteractiveButton {
            id: "spanish".to_string(),
            title: "🇪🇸 Español".to_string(),
        },
    ];

    match send_whatsapp_interactive_buttons(
        &client,
        phone,
        "🌍 Choose Your Language / भाषा चुनें / Elige tu idioma",
        "Welcome to our global service! Please select your preferred language to continue.\n\nनमस्ते! कृपया अपनी भाषा चुनें।\n\n¡Hola! Por favor selecciona tu idioma.",
        Some("Language selection / भाषा चयन / Selección de idioma"),
        language_buttons,
    )
    .await
    {
        Ok(response) => println!("✅ Language selection sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Example 2: Hindi Message
    match send_whatsapp_text(
        &client,
        phone,
        "🙏 **आपका स्वागत है!**\n\nनमस्ते! मैं आयमान हूं, एक फुल-स्टैक डेवलपर।\n\n🌐 **हमारी सेवाएं:**\n• वेबसाइट डिज़ाइन और डेवलपमेंट\n• मोबाइल ऐप्स (Android/iOS)\n• ई-कॉमर्स समाधान\n• डिजिटल मार्केटिंग\n\n💰 **विशेष छूट:** पहली प्रोजेक्ट पर 20% की छूट!\n\n📞 **संपर्क करें:** व्हाट्सऐप पर मुफ्त परामर्श के लिए 'हां' टाइप करें।\n\n✨ आपके सपनों को डिजिटल दुनिया में लाने के लिए तैयार हैं!",
    )
    .await
    {
        Ok(response) => println!("✅ Hindi message sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    tokio::time::sleep(tokio::time::Duration::from_secs(4)).await;

    // Example 3: Spanish Message
    match send_whatsapp_text(
        &client,
        phone,
        "¡Hola! **¡Bienvenido a Aymaan Tech Studio!** 🚀\n\n👨‍💻 Soy Aymaan, desarrollador full-stack especializado en:\n\n🌐 **Servicios Disponibles:**\n• Desarrollo web personalizado\n• Aplicaciones móviles nativas\n• Tiendas online (e-commerce)\n• Consultoría tecnológica\n\n💡 **¿Por qué elegirnos?**\n✅ Diseños modernos y responsivos\n✅ Código limpio y optimizado\n✅ Soporte técnico 24/7\n✅ Precios competitivos\n\n🎁 **Oferta especial:** ¡Consulta gratuita de 30 minutos!\n\n📱 Responde 'SÍ' para comenzar tu proyecto digital hoy mismo.",
    )
    .await
    {
        Ok(response) => println!("✅ Spanish message sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    println!("🌍 All multilingual messages sent successfully!");
    Ok(())
}
