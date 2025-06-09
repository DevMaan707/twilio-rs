use actix_web::{App, HttpServer};
use std::sync::Arc;
use twilio_rs::{build_twilio_webhook, AutoReplyHandler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let base_url = "https://your-domain.com/twilio/whatsapp";
    let handler: AutoReplyHandler = Arc::new(|_from, body| format!("You said: {}", body));

    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new().service(build_twilio_webhook(base_url, Some(handler.clone())))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
