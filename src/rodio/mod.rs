pub mod netease;
pub mod xinlifm;
#[allow(dead_code)]
pub struct Music {
    name: String,
    path: String,
}

extern crate rodio;
use std::sync::mpsc::Sender;

pub fn player_test() -> Sender<String> {
    use rodio::Sink;
    use std::io::BufReader;
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    let (sender, receiver) = channel::<String>();

    let _device = rodio::default_output_device().unwrap();
    let sink = Arc::new(Mutex::new(Sink::new(&_device)));
    let sink_clone = sink.clone();
    thread::spawn(move || {
        let d = Duration::from_millis(10);
        loop {
            let _r = receiver.recv_timeout(d);
            match _r {
                Ok(msg) => {
                    println!("song_file_path:{:}", msg);
                    let file = std::fs::File::open(msg).unwrap();
                    sink_clone
                        .lock()
                        .unwrap()
                        .append(rodio::Decoder::new(BufReader::new(file)).unwrap());
                    println!("is_sleep");
                }
                Err(_e) => {}
            }
        }
    });
    sink.lock().unwrap().sleep_until_end();
    sender
}
pub fn player_ing() {
    loop {}
}
#[allow(dead_code)]
pub fn play_netease(sender: Sender<String>, keyword: String) {
    let _songs = netease::get_songs_data(keyword);
    for song in _songs.members() {
        let sound_file_name =
            netease::get_song_playurl_by_id("rodio".to_string(), song["id"].to_string());
        sender.send(sound_file_name).unwrap();
    }
}

#[allow(dead_code)]
pub fn play_xinlifm(sender: Sender<String>, keyword: String) {
    let _songs = xinlifm::get_songs_data(keyword);
    for song in _songs.members() {
        let sound_file_name =
            xinlifm::get_song_playurl_by_id("rodio".to_string(), song["id"].to_string());
        println!("song_name: {:}", sound_file_name);
        sender.send(sound_file_name).unwrap();
    }
}
