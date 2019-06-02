#[macro_use]
extern crate term;
use std::io::prelude::*;
use std::thread::sleep;
use std::time::Duration;

pub fn shave_the_yak() {
    let mut t = term::stdout().unwrap();
    t.fg(term::color::GREEN).unwrap();
    write!(t, "value,1").unwrap();
    sleep(Duration::from_millis(1_000));
    //t.carriage_return(); //cursor_up();
}

fn main() {
    let path = "resource/1559457768.mp3"; //1559457768
    use mp3_duration;
    use std::path::Path;

    //let path = Path::new("music.mp3");
    let total_second = mp3_duration::from_path(&path).unwrap().as_secs();
    let minute = format!("{:02}", total_second / 60);
    let second = format!("{:02}", total_second % 60);
    println!("{}, {}", minute, second);
    shave_the_yak();
}
