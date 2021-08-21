use tokio::prelude::*;
use tokio::timer::Interval;
use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use std::time::{Duration, Instant};

extern crate lettre;
extern crate lettre_email;
extern crate mime;

fn main() {
    let task = Interval::new(Instant::now(), Duration::from_millis(86400000))//86400000
        .take(9999999)
        .for_each(|instant| {

            let email_receiver = "2066404102@qq.com";
            let mine_email = "2533958824@qq.com";
            let smtp_server = "smtp.qq.com";
            let password = "ldrmexiknhlwdigh"; //需要生成应用专用密码
        
            let email = Email::builder()
            .to(email_receiver)
            .from((mine_email,"老公"))
            .subject("记得填写今天的日报哦~")
            .html("https://docs.qq.com/sheet/DSGVSYkpudG5lUEd1")
            .build()
            .unwrap();

            let creds = Credentials::new(
                mine_email.to_string(),
                password.to_string(),
            );
        
            // Open connection to Gmail
            let mut mailer = SmtpClient::new_simple(smtp_server)
            .unwrap()
            .credentials(creds)
            .transport();
         
            // Send the email
            let result = mailer.send(email.into());
         
            if result.is_ok() {
                println!("Email sent");
            } else {
                println!("Could not send email: {:?}", result);
            }
         
            print!("{:?}", result);
            mailer.close();

            println!("fire; instant={:?}", instant);

            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
}
