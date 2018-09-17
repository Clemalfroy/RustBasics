extern crate clap;
extern crate reqwest;
extern crate serde_json;

use std::error::Error;
use std::io::Read;
use serde_json::{Value};
use std::env;
use std::{thread, time::Duration};

pub struct TravelInfo {
    date: String,
    dest: String,
    from: String,
}

impl TravelInfo {
    pub fn new(args: clap::ArgMatches) -> TravelInfo {
        TravelInfo {
            date: args.value_of("date").unwrap().to_string(),
            dest: args.value_of("from").unwrap().to_string(),
            from: args.value_of("to").unwrap().to_string(),
        }
    }
}

fn get_travels(url: &str) -> Result <serde_json::Value, Box<dyn Error>> {
    let mut buffer = String::new();

    let client = reqwest::Client::new();
    let mut res = client.get(url).send()?;
    res.read_to_string(&mut buffer)?;
    let v: Value = serde_json::from_str(&buffer)?;
    return Ok(v);
}

fn send_sms(message: String) -> Result<(), Box<dyn Error>> {

    println!("Sending message:{}", message);
    let (username, pass) = match (env::var_os("SMS_USER"), env::var_os("SMS_PASS")) {
        (Some(username), Some(pass)) => (username.to_os_string(), pass.to_os_string()),
        _ => return Err(Box::from("Secret keys aren't set"))
    };

    let url = format!(
        "https://smsapi.free-mobile.fr/sendmsg?user={:?}&pass={:?}&msg={}",
        username,
        pass,
        message,
        );
    print!("{:?} ", url);
    reqwest::get(&url)?;
    Ok(())
}

pub fn run(info: TravelInfo) -> Result<(), Box<dyn Error>> {
    
    let url = format!("https://ressources.data.sncf.com/api/records/1.0/search/\
    ?dataset=tgvmax&refine.od_happy_card=OUI&refine.date=\
    {}&refine.origine={}&refine.destination={}",
    info.date, info.from, info.dest);
    println!("{}", url);

    loop {
        thread::sleep(Duration::new(30, 0));
        println!("I'm searching...");
        let travels = get_travels(&url)?;
        if travels["nhits"] == 0 {
            println!("No results, I'll try again in 30 seconds!");
            continue;
        }
        send_sms("coucou".to_string())?;
        // for travel in  travels["records"] {

        // }
    }
//https://smsapi.free-mobile.fr/sendmsg?user=37005560&pass=QWW7yiyhmzppSe&msg=Hello%20World%20! 
}