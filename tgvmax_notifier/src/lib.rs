extern crate clap;
extern crate reqwest;
extern crate serde_json;
extern crate url;

use std::error::Error;
use std::io::Read;
use serde_json::{Value};
use std::env;
use std::{thread, time::Duration};
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

pub struct TravelInfo {
    date: String,
    dest: String,
    from: String,
}

impl TravelInfo {
    pub fn new(args: clap::ArgMatches) -> TravelInfo {
        TravelInfo {
            date: args.value_of("date").unwrap().to_string(),
            dest: args.value_of("to").unwrap().to_string(),
            from: args.value_of("from").unwrap().to_string(),
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

fn send_sms(message: &str) -> Result<(), Box<dyn Error>> {

    let (username, pass) = match (env::var_os("SMS_USER"), env::var_os("SMS_PASS")) {
        (Some(username), Some(pass)) => (username.into_string().unwrap(), pass.into_string().unwrap()),
        _ => return Err(Box::from("Secret keys aren't set."))
    };

    let url = format!(
        "https://smsapi.free-mobile.fr/sendmsg?user={}&pass={}&msg={}",
        username,
        pass,
        message,
        );

    reqwest::get(&url)?;

    println!("Sending message: {}", message);

    Ok(())
}

pub fn run(info: TravelInfo) -> Result<(), Box<dyn Error>> {
    
    let url = format!("https://ressources.data.sncf.com/api/records/1.0/search/\
    ?dataset=tgvmax&refine.od_happy_card=OUI&refine.date=\
    {}&refine.origine={}&refine.destination={}",
    info.date, info.from, info.dest);
    println!("{}", url);

    let mut last_message = String::new();

    loop {
        thread::sleep(Duration::new(30, 0));
        println!("I'm searching...");
        let travels = get_travels(&url)?;
        if travels["nhits"] == 0 {
            println!("No results, I'll try again in 30 seconds!");
            continue;
        }
        let mut message = String::new();
        let travels = travels["records"].as_array().unwrap();
        for travel in travels {
            message.push_str(travel["fields"]["heure_depart"].as_str().unwrap());
            message.push('\n');
        }
        let message: String = utf8_percent_encode(&message, DEFAULT_ENCODE_SET).collect();
        if message == last_message { continue }
        send_sms(&message)?;
        last_message = message;
    }
}