extern crate tgvmax_notifier;
extern crate clap;

use clap::{Arg, App};
use std::process;

fn main() {
    
    let args = App::new("tgvmax_notifier")
                            .version("1.0").author("Clement M. <clement.malfroy@gmail.com>")
                            .about("Notify you when a train is available")
                            .arg(Arg::with_name("date")
                               .short("d").long("date").value_name("DATE")
                               .help("Sets the date when you want to travel, format:\
                               <YEAR-MONTH-DAY> exemple: 2018-10-01")
                               .takes_value(true).required(true))
                            .arg(Arg::with_name("from")
                               .short("f").long("from").value_name("FROM")
                               .help("Sets the city you want to depart from")
                               .takes_value(true).required(true))
                            .arg(Arg::with_name("to")
                               .short("t").long("to").value_name("TO")
                               .help("Sets the city you want go to")
                               .takes_value(true).required(true))
                          .get_matches();

    if let Err(e) = tgvmax_notifier::run(tgvmax_notifier::TravelInfo::new(args)) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
