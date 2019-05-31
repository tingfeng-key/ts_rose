pub mod netease;
pub mod xinlifm;
#[allow(dead_code)]
pub struct Music {
    name: String,
    path: String,
}

extern crate rodio;
use self::rodio::Sink;
use std::io::BufReader;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub fn player_test() -> Sender<String> {
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;

    let (sender, receiver) = channel::<String>();

    let _device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&_device);
    thread::spawn(move || {
        let d = Duration::from_millis(10);
        loop {
            let _r = receiver.recv_timeout(d);
            match _r {
                Ok(msg) => {
                    let file = std::fs::File::open(msg).unwrap();
                    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
                }
                Err(e) => {}
            }
        }
    });
    sink.sleep_until_end();
    sender
}
#[allow(unreachable_code)]
pub fn player() -> (Sink, Vec<Music>) {
    let _device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&_device);
    let musics: Vec<Music> = Vec::new();

    (sink, musics)
}
#[allow(dead_code)]
pub fn play_netease(sender: Sender<String>, keyword: String) {
    use std::io::BufReader;
    use std::sync::{Arc, Mutex};
    let _songs = netease::get_songs_data(keyword);
    for song in _songs.members() {
        //println!("{:?}", song);
        let sound_file_name =
            netease::get_song_playurl_by_id("rodio".to_string(), song["id"].to_string());
        sender.send(sound_file_name);
    }
}

#[allow(dead_code)]
pub fn play_xinlifm(sender: Sender<String>, keyword: String) {
    use std::thread;
    use std::thread::sleep;
    use std::time::Duration;

    let _songs = xinlifm::get_songs_data(keyword);
    let sink_clone = Arc::new(Mutex::new(sink));
    let sink_clone_clone = sink_clone.clone();
    let sink_musics = Arc::new(Mutex::new(musics));
    let sink_musics_clone = sink_musics.clone();
    thread::spawn(move || {
        let mut last_played_music_name: String = "".to_string();
        let mut current_playing_music_name: String = "".to_string();
        let mut last_sink_len = 0;
        loop {
            let index = sink_musics.clone().lock().unwrap().len()
                - sink_clone.clone().lock().unwrap().len();
            current_playing_music_name = match sink_musics.clone().lock().unwrap().get(index) {
                Some(music) => music.name.clone(),
                None => "".to_string(),
            };
            if (last_played_music_name.ne(&current_playing_music_name)) {
                last_played_music_name = current_playing_music_name.clone();
                println!("当前歌曲: {}", current_playing_music_name);
            }
            sleep(Duration::from_secs(2));
        }
    });

    for song in _songs.members() {
        let sound_file_name =
            xinlifm::get_song_playurl_by_id("rodio".to_string(), song["id"].to_string());
        sink_musics_clone.clone().lock().unwrap().push(Music {
            name: song["title"].to_string(),
            path: sound_file_name.clone(),
        });
        let file = std::fs::File::open(sound_file_name.clone()).unwrap();
        //files.push(rodio::Decoder::new(BufReader::new(file)).unwrap());
        sink_clone_clone
            .lock()
            .unwrap()
            .append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    }

    sink_clone_clone.lock().unwrap().sleep_until_end();
}
