pub mod netease;
pub mod xinlifm;

use mp3_duration;
use reqwest::Url;
use rodio::queue::SourcesQueueOutput;
use rodio::source::Source;
use rodio::Sink;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Music {
    name: String,
    url: String,
    path: Option<String>,
    total_duration: Option<u64>,
}
#[allow(dead_code)]
pub fn player_start() -> Sender<String> {
    extern crate rodio as rodio_lib;
    use std::io::BufReader;
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};

    let (sender, receiver) = channel::<String>();

    let _device = rodio_lib::default_output_device().unwrap();
    let sink = Arc::new(Mutex::new(Sink::new(&_device)));
    let sink_clone = sink.clone();

    let mut musics: Vec<Music> = Vec::new();

    //let player = Player::new();
    thread::spawn(move || {
        let d = Duration::from_millis(10);

        let mut last_song_name: String = "".to_string();
        let mut current_duration = 0;

        loop {
            let musics_len = musics.len();
            if musics_len > 0 {
                let idx = musics_len - sink_clone.lock().unwrap().len();
                let current_song = musics.get(idx).unwrap();
                let current_song_name = current_song.clone().name;
                if !current_song_name.eq(&last_song_name) {
                    last_song_name = current_song_name.clone();
                    current_duration = 0;
                    println!(
                        "正在播放歌曲：{}, 总时长：{}",
                        current_song_name.clone(),
                        current_song.clone().total_duration.unwrap()
                    );
                }

                current_duration += 1;

                println!("{}: {}", "time", current_duration);
                thread::sleep(Duration::from_millis(1_000));
            }

            let _r = receiver.recv_timeout(d);
            match _r {
                Ok(msg) => {
                    let music: Music = json::decode(&msg).unwrap();
                    musics.push(music.clone());
                    let path = music.path.unwrap();
                    let file = std::fs::File::open(path).unwrap();
                    let source = rodio_lib::Decoder::new(BufReader::new(file)).unwrap();
                    sink_clone.lock().unwrap().append(source);
                }
                Err(_e) => {}
            }
        }
    });
    sink.lock().unwrap().sleep_until_end();
    sender
}
pub fn player_ing() {
    thread::park();
}
#[allow(dead_code)]
pub fn play_netease(sender: Sender<String>, keyword: String) {
    let _songs = netease::get_songs_data(keyword);
    for song in _songs {
        let obj = song.as_object().unwrap();
        let name = match obj.get("name").unwrap() {
            Json::String(s) => s,
            _ => "error name",
        };
        let id = match obj.get("id").unwrap() {
            Json::U64(s) => s,
            _ => &0,
        };
        let song_url = netease::get_song_play_url_by_id(id.to_string());
        if !song_url.is_empty() {
            let mut music = Music {
                name: name.to_string(),
                url: song_url,
                path: None,
                total_duration: None,
            };
            let file_path = download_file_by_play_url(&music.url.as_str());
            music.path = Some(file_path.clone());
            music.total_duration = Some(
                mp3_duration::from_path(file_path.clone())
                    .unwrap()
                    .as_secs(),
            );
            let msg = json::encode(&music).unwrap();
            sender.send(msg).unwrap();
        }
    }
}

#[allow(dead_code)]
pub fn play_xinlifm(sender: Sender<String>, keyword: String) {
    let songs = xinlifm::get_songs_data(keyword);
    for song in songs {
        let obj = song.as_object().unwrap();
        let name = match obj.get("title").unwrap() {
            Json::String(s) => s,
            _ => "error name",
        };
        let id = match obj.get("id").unwrap() {
            Json::U64(s) => s,
            _ => &0,
        };
        let song_url = xinlifm::get_song_playurl_by_id(id.to_string());
        if !song_url.is_empty() {
            let mut music = Music {
                name: name.to_string(),
                url: song_url,
                path: None,
                total_duration: None,
            };
            let file_path = download_file_by_play_url(&music.url.as_str());
            music.path = Some(file_path.clone());
            music.total_duration = Some(
                mp3_duration::from_path(file_path.clone())
                    .unwrap()
                    .as_secs(),
            );
            let msg = json::encode(&music).unwrap();
            sender.send(msg).unwrap();
        }
    }
}
#[allow(dead_code)]
fn download_file_by_play_url(play_url: &str) -> String {
    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::io::copy;
    use std::path::Path;
    use std::time::SystemTime;

    static SAVE_DIR: &str = "download";
    let duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    //println!("play_url: {:?}", play_url);
    let url = Url::parse(play_url).unwrap();
    //println!("url: {:?}", url);
    let file_name = duration.as_secs();
    let mut download_file = reqwest::Client::new()
        .get(url)
        .send()
        .expect("request error");

    let file_path = format!("{}/{}.mp3", SAVE_DIR, &file_name);
    fs::create_dir_all(SAVE_DIR).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    let path = Path::new(&file_path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}, {}", display, why.description()),
        Ok(file) => file,
    };
    copy(&mut download_file, &mut file).unwrap();
    file_path
}
