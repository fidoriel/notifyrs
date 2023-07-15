use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use lettre::{message::Mailbox, Message, SmtpTransport, Transport};
use serde::Deserialize;
use std::env;

// Define our data structures
#[derive(Deserialize)]
struct EmailInput {
    recipient_email: Option<Vec<String>>,
    topic: String,
    body: String,
}

struct Config {
    sender_email: String,
    username: String,
    password: String,
    default_recipient_emails: Vec<String>,
    smtp_host: String,
    secret_key: Option<String>,
}

#[post("/notify")]
async fn send_email(req: HttpRequest, info: web::Json<EmailInput>) -> impl Responder {
    let config = match load_config() {
        Ok(config) => config,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    if let Some(secret_key) = config.secret_key {
        match req.headers().get("X-SECRET-KEY") {
            Some(value) => {
                if value.to_str().unwrap_or("") != secret_key {
                    return HttpResponse::Unauthorized().body("Invalid secret key");
                }
            }
            None => return HttpResponse::Unauthorized().body("No secret key provided"),
        };
    }

    let recipients = match &info.recipient_email {
        Some(emails) => emails.clone(),
        None => config.default_recipient_emails.clone(),
    };

    let mut response = Vec::new();

    for recipient_email in recipients {
        let email = Message::builder()
            .from(Mailbox::new(
                None,
                config.sender_email.clone().parse().unwrap(),
            ))
            .to(Mailbox::new(None, recipient_email.clone().parse().unwrap()))
            .subject(info.topic.clone())
            .body(info.body.clone())
            .unwrap();

        let mailer = SmtpTransport::relay(&config.smtp_host)
            .unwrap()
            .credentials(lettre::transport::smtp::authentication::Credentials::new(
                config.username.clone(),
                config.password.clone(),
            ))
            .build();

        match mailer.send(&email) {
            Ok(_) => response.push(format!("Email sent to {}", recipient_email)),
            Err(e) => response.push(format!(
                "Failed to send email to {}: {}",
                recipient_email, e
            )),
        }
    }
    HttpResponse::Ok().body(format!("{:?}", response))
}

fn load_config() -> Result<Config, String> {
    let sender_email =
        env::var("SENDER_EMAIL").map_err(|_| "SENDER_EMAIL environment variable not set")?;
    let username =
        env::var("SMTP_USERNAME").map_err(|_| "SMTP_USERNAME environment variable not set")?;
    let password =
        env::var("SMTP_PASSWORD").map_err(|_| "SMTP_PASSWORD environment variable not set")?;
    let default_recipient_emails = env::var("DEFAULT_RECIPIENT_EMAILS")
        .map(|emails| emails.split(',').map(String::from).collect())
        .map_err(|_| "DEFAULT_RECIPIENT_EMAILS environment variable not set or invalid format")?;
    let smtp_host = env::var("SMTP_HOST").map_err(|_| "SMTP_HOST environment variable not set")?;
    let secret_key = env::var("SECRET_KEY").ok();

    Ok(Config {
        sender_email,
        username,
        password,
        default_recipient_emails,
        smtp_host,
        secret_key,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    HttpServer::new(move || App::new().service(send_email))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
