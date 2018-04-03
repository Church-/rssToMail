extern crate rss;
extern crate html2text;
extern crate lettre;

use html2text::from_read;
use rss::Channel;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::SUBMISSION_PORT;
use lettre::{SimpleSendableEmail, EmailTransport, EmailAddress, SmtpTransport};
use lettre::smtp::extension::ClientId;
use lettre::smtp::ConnectionReuseParameters;



fn genText(url: &str) -> String {
	let channel = Channel::from_url(url).unwrap();
    let items = channel.items();
	let mut message = String::new();
    for i in items {
    	let title = i.title().unwrap();
    	let body = from_read(i.content().unwrap().as_bytes(),80);
		message.push_str(title);
		message.push_str("\n");
		message.push_str(&body);
		message.push_str("\n");
    }
    message.to_string()
}

fn sendMail(message: String) {
	let email = SimpleSendableEmail::new(
	            EmailAddress::new("appleman2412@gmail.com".to_string()),
	            vec![EmailAddress::new("turing.thesis@gmail.com".to_string())],
	            "UpWork RSS Feed".to_string(),
	            message,
	            );
	            
	let mut mailer = SmtpTransport::simple_builder("smtp.gmail.com".to_string()).unwrap()
	    // Set the name sent during EHLO/HELO, default is `localhost`
	    .hello_name(ClientId::Domain("smtp.gmail.com".to_string()))
	    // Add credentials for authentication
	    .credentials(Credentials::new("appleman2412@gmail.com".to_string(), "blades40!".to_string()))
	    // Enable SMTPUTF8 if the server supports it
	    .smtp_utf8(true)
	    // Configure expected authentication mechanism
	    .authentication_mechanism(Mechanism::Plain)
	    // Enable connection reuse
	    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited).build();
	
	let res = mailer.send(&email);
	assert!(res.is_ok());
	
	mailer.close();      
	
}

fn main() {
	let s = genText("https://www.upwork.com/ab/feed/jobs/rss?q=%28devops+OR+devop+OR+python+scripting+OR+python+OR+sysadmin+OR+rust+OR+ruby%29+AND+NOT+%28marketing+automation+OR+marketing+OR+advertising%29&user_location_match=1&sort=renew_time_int+desc&paging=0%3B10&api_params=1&securityToken=287ec3ca3d67c86e1ce9e63b48765e2010fe536a1643404d4d7244231c9ecdb4840c3ed565c5170010592929d0633b526a6a26be7c7d52e4834f18ce1a0d2df2&userUid=862909525124521984&orgUid=862909525128716289");
	sendMail(s);	
}
