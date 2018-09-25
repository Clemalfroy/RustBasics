extern crate rodio;
extern crate time;

use std::io::BufReader;
use std::thread;
use rodio::Sink;
use time::{Duration, PreciseTime};

fn play_sound(device: &rodio::Device, file: std::fs::File) {

    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sink = Sink::new(&device);
    sink.append(source);
    println!("Started Music");
    sink.sleep_until_end();
    println!("Stopped Music");
}

fn main() {

    let start = PreciseTime::now();
    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("src/music.wav").unwrap();

    println!("{:?}", start);
    play_sound(&device, file);
}