extern crate rodio;
extern crate time;

use std::io::BufReader;
use rodio::Sink;
use time::SteadyTime;
use time::Duration;

fn play_sound(device: &rodio::Device, file: std::fs::File) {

    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sink = Sink::new(&device);
    sink.append(source);
    println!("Started Music");
    sink.sleep_until_end();
    println!("Stopped Music");
}

fn main() {

    let device = rodio::default_output_device().unwrap();
    let file = std::fs::File::open("src/music.wav").unwrap();

    
    let start = SteadyTime::now();
    let time = Duration::seconds(25);

    while SteadyTime::now() - start < time {
        continue;
    }
    play_sound(&device, file);
}