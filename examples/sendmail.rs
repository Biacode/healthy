extern crate lettre;

use lettre::transport::smtp::SmtpTransportBuilder;
use lettre::transport::EmailTransport;
use lettre::email::EmailBuilder;

fn main() {
    let mut sender = SmtpTransportBuilder::localhost().unwrap().build();
    let email = EmailBuilder::new()
        .to("biacoder@gmail.com")
        .from("arthur@localhost")
        .body("Hello World!")
        .subject("Hello")
        .build()
        .unwrap();
    let result = sender.send(email);
    assert!(result.is_ok());
    println!("result = {:?}", result);
}
