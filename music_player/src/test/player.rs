extern crate rodio;

use std::io::BufReader;

fn main() {
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);

    let file = std::fs::File::open("rodio/99388001.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    loop {
        println!("123");
    }
    sink.sleep_until_end();
}
