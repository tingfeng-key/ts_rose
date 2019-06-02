extern crate rodio as rodio_lib;
use std::io::BufReader;
use std::sync::mpsc::channel;

#[allow(dead_code)]
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct Music {
    name: String,
    url: String,
    path: Option<String>,
    duration: Option<Duration>,
}
#[allow(dead_code)]
struct Player {
    sink: Sink,
    queue_rx: SourcesQueueOutput<f32>,
    musics: Option<Vec<Music>>,
}
impl Player {
    #[allow(dead_code)]
    pub fn new() -> Player {
        let _device = rodio::default_output_device().unwrap();
        let (s, q) = Sink::new_idle();

        let player = Player {
            sink: s,
            queue_rx: q,
            musics: None,
        };
        player
    }
    pub fn start() {
        thread::spawn(move || {
            let d = Duration::from_millis(10);

            let mut last_song_name: String = String::new();

            loop {
                let musics_len = musics.len();
                if musics_len > 0 {
                    let idx = musics_len - sink_clone.lock().unwrap().len();
                    let current_song = musics.get(idx).unwrap();
                    let current_song_name = current_song.clone().name;
                    if !current_song_name.eq(&last_song_name) {
                        last_song_name = current_song_name;
                        println!("正在播放歌曲：{}, 时长：1", last_song_name);
                    }
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
    }
}
