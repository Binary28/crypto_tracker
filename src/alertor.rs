use dotenv::dotenv;
use lettre::{smtp::authentication::Credentials, SmtpClient, SmtpTransport, Transport};
use lettre_email::EmailBuilder;
use std::{process::Command, time::Duration};

use crate::error::{ErrorKind, Errors};

#[allow(dead_code)]
pub struct Alerter<'a> {
    email: String,
    password: String,
    to_email: &'a str,
    mailer: SmtpTransport,
    mob_no: u64,
    wait: bool,
}

impl<'a> Alerter<'a> {
    pub fn new(to_email: &'a str) -> Result<Alerter<'a>, Errors<'a>> {
        dotenv().ok();
        let email = std::env::var("EMAIL").unwrap();
        let password = std::env::var("PASSWORD").unwrap();
        let mob_no = std::env::var("PHONE_NO").unwrap().parse::<u64>().unwrap();
        let creds = Credentials::new(email.clone(), password.clone());
        let mailer = match SmtpClient::new_simple("smtp.gmail.com") {
            Ok(val) => val.credentials(creds).transport(),
            Err(_) => {
                return Err(Errors::new(
                    ErrorKind::Alerterror,
                    "Cannot Connect to SMTP Server",
                ))
            }
        };

        Ok(Alerter {
            email,
            password,
            to_email,
            mailer,
            mob_no,
            wait: false,
        })
    }

    pub async fn alert_mail(&mut self, coin: &'a str) -> Result<(), Errors<'a>> {
        if self.wait {
            return Ok(());
        }

        let mail = EmailBuilder::new();
        let body = format!(
            r#"<!DOCTYPE html>
        <html lang='en'>
        <head>
            <meta charset='UTF-8'>
            <meta http-equiv='X-UA-Compatible' content='IE=edge'>
            <meta name='viewport' content='width=device-width, initial-scale=1.0'>
            <title>Document</title>
        </head>
        <body>
            Uptime Bot {} coin is up
        </body>
        </html>"#,
            coin
        );
        let mail = match mail
            .from(self.email.as_str())
            .to(self.to_email)
            .subject("ALERT")
            .html(body)
            // .body(body)
            .build()
        {
            Ok(val) => val,
            Err(_) => {
                return Err(Errors::new(
                    ErrorKind::Alerterror,
                    "Cannot Build Email Content",
                ))
            }
        };

        match self.mailer.send(mail.into()) {
            Ok(_) => {
                println!("Email Send Successfully, Alerted: {:20}", self.to_email);
                Ok(())
            }
            Err(e) => {
                println!("{}", e.to_string());
                Err(Errors::new(ErrorKind::Alerterror, "Cannot Send e-mail"))
            }
        }
    }

    pub async fn alert_voice(&mut self, message: &'a str) {
        if self.wait {
            return;
        }

        let cmd = match Command::new("python3")
            .current_dir(".")
            .arg("./alert_voice.py")
            .args(&["-to", &format!("+91{}", self.mob_no)])
            .args(&["-m", message])
            .output()
        {
            Ok(val) => val,
            Err(_) => return println!("ALert Error: Cannot run the call script"),
        };
        println!("{}", std::str::from_utf8(&cmd.stdout).unwrap());
        self.hold(10).await;
    }

    pub async fn hold(&mut self, secs: u64) {
        self.wait = true;
        std::thread::sleep(Duration::from_secs(secs));
        std::process::exit(0);
    }
}
