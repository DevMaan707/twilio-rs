use twilio_rs::{
    client::TwilioClient,
    whatsapp::{
        send_whatsapp_appointment_reminder, send_whatsapp_reminder, InteractiveButton,
        ReminderMessage,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = TwilioClient::new();
    let phone = "+917569785621"; // Replace with your phone number

    println!("⏰ Sending task reminder...");

    // Example 1: Task Reminder with Action Buttons
    let task_buttons = vec![
        InteractiveButton {
            id: "completed".to_string(),
            title: "✅ Mark Complete".to_string(),
        },
        InteractiveButton {
            id: "snooze".to_string(),
            title: "⏰ Snooze 1hr".to_string(),
        },
        InteractiveButton {
            id: "reschedule".to_string(),
            title: "📅 Reschedule".to_string(),
        },
    ];

    let task_reminder = ReminderMessage {
        title: "Project Deadline".to_string(),
        body: "Don't forget to submit the client presentation for the mobile app project. The deadline is in 2 hours!\n\n📋 Checklist:\n• Final design mockups ✅\n• User flow diagrams ✅\n• Technical documentation ⏳\n• Budget breakdown ⏳".to_string(),
        reminder_time: Some("2024-03-15T16:00:00Z".to_string()),
        action_buttons: Some(task_buttons),
    };

    match send_whatsapp_reminder(&client, phone, task_reminder).await {
        Ok(response) => println!("✅ Task reminder sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("🩺 Sending appointment reminder...");

    // Example 2: Medical Appointment
    match send_whatsapp_appointment_reminder(
        &client,
        phone,
        "March 20, 2024",
        "2:30 PM",
        "Apollo Hospital, Consultation Room 3A, 2nd Floor",
        Some("Rajesh Kumar"),
    )
    .await
    {
        Ok(response) => println!(
            "✅ Medical appointment reminder sent! SID: {}",
            response.sid
        ),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("💇 Sending salon appointment reminder...");

    // Example 3: Salon/Beauty Appointment
    match send_whatsapp_appointment_reminder(
        &client,
        phone,
        "Tomorrow (March 16)",
        "11:00 AM",
        "Glamour Studio, MG Road, Near City Mall",
        None, // No doctor name for salon
    )
    .await
    {
        Ok(response) => println!("✅ Salon appointment reminder sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    // Wait a bit before sending next message
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    println!("🔔 Sending meeting reminder...");

    // Example 4: Simple Meeting Reminder (no buttons)
    let meeting_reminder = ReminderMessage {
        title: "Team Standup Meeting".to_string(),
        body: "📞 Daily standup meeting starts in 15 minutes!\n\n🎯 Today's Agenda:\n• Sprint progress updates\n• Blocker discussions\n• Tomorrow's goals\n\n💻 Join Link: https://meet.google.com/abc-defg-hij\n📞 Dial-in: +91-xxx-xxx-xxxx".to_string(),
        reminder_time: Some("2024-03-15T09:45:00Z".to_string()),
        action_buttons: None,
    };

    match send_whatsapp_reminder(&client, phone, meeting_reminder).await {
        Ok(response) => println!("✅ Meeting reminder sent! SID: {}", response.sid),
        Err(e) => eprintln!("❌ Error: {}", e),
    }

    println!("⏰ All reminder messages sent successfully!");
    Ok(())
}
