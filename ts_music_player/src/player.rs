use crate::engine;
use minimp3::Decoder;
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MusicMeta {
    name: String,
    local_path: String,
    duration: u64,
}
impl MusicMeta {
    #[allow(dead_code)]
    pub fn remove() {}
}
#[allow(dead_code)]
#[derive(Debug)]
pub struct Download {
    pub url: String,
    pub name: String,
}
#[allow(dead_code)]
pub struct Player {
    play_sender: Sender<PlayerCmd>,
    term_receiver: Receiver<TerminalCmd>,
    musics: Arc<Mutex<Vec<MusicMeta>>>,
    download_list_sender: Sender<Download>,
}
#[allow(dead_code)]
enum PlayerCmd {
    Next(MusicMeta),
    Stop,
    Play(MusicMeta),
    Exit,
}
#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    NoOutputDevice,
    ChannelRecvTimeout,
    ChannelRecvDisconnected,
    RecvError,
}
#[allow(dead_code)]
enum TerminalCmd {
    AddToList(MusicMeta),
    NowPlay(String),
}
impl Player {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let (player_sender, player_receiver) = channel();
        let (terminal_sender, terminal_receiver) = channel();
        let (down_list_sender, down_list_receiver) = channel();

        let musics: Arc<Mutex<Vec<MusicMeta>>> = Arc::new(Mutex::new(Vec::new())); //Arc::new<Mutex::new<Vec<MusicMeta>>>

        let player_sender_clone = player_sender.clone();
        let terminal_sender_clone = terminal_sender.clone();
        let duration = Duration::from_millis(10_000); // 10 秒
        let _player_thread_join_handle: thread::JoinHandle<Result<(), Error>> =
            thread::spawn(move || -> Result<(), Error> {
                loop {
                    let recv_msg = player_receiver
                        .recv_timeout(duration)
                        .map_err(|e| match e {
                            std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                            std::sync::mpsc::RecvTimeoutError::Disconnected => {
                                Error::ChannelRecvDisconnected
                            }
                        })?;
                    let audio_engine = engine::AudioEngine::run_output_device();
                    match recv_msg {
                        PlayerCmd::Play(music_meta) => {
                            /*terminal_sender
                            .send(TerminalCmd::NowPlay(music_meta.name))
                            .expect("error");*/

                            let mut decoder = Decoder::new(
                                File::open(Path::new(music_meta.clone().local_path.as_str()))
                                    .unwrap(),
                            );
                            println!("{}, {}", music_meta.name, music_meta.duration);
                            loop {
                                let frame = decoder.next_frame();
                                match frame {
                                    Ok(mut f) => {
                                        for sample in f.data.chunks_mut(f.channels as usize) {
                                            for a in sample.iter_mut() {
                                                audio_engine
                                                    .output_sender
                                                    .clone()
                                                    .unwrap()
                                                    .send(engine::EngineInput::Input(
                                                        a.clone() as f32 / std::i16::MAX as f32,
                                                    ))
                                                    .expect("send decode error"); //a.clone() as f32 / std::i16::MAX as f32,
                                            }
                                        }
                                    }
                                    Err(_) => {
                                        break;
                                    }
                                }
                            }
                        }
                        PlayerCmd::Next(music_meta) => {
                            terminal_sender
                                .send(TerminalCmd::NowPlay(music_meta.name))
                                .expect("error");
                        }
                        PlayerCmd::Stop => {
                            audio_engine
                                .output_sender
                                .clone()
                                .unwrap()
                                .send(engine::EngineInput::Exit)
                                .expect("send decode error");
                        }
                        PlayerCmd::Exit => {}
                    }
                }
            });

        let _download_thread_join_handle = thread::spawn(move || -> Result<(), Error> {
            loop {
                let recv_msg = down_list_receiver
                    .recv_timeout(duration)
                    .map_err(|e| match e {
                        std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                        std::sync::mpsc::RecvTimeoutError::Disconnected => {
                            Error::ChannelRecvDisconnected
                        }
                    });
                match recv_msg {
                    Ok(download) => {
                        let d: Download = download;
                        let path = Self::download_file_by_url(d.url.as_str());
                        let total_duration =
                            mp3_duration::from_path(path.clone()).unwrap().as_secs();
                        let music: MusicMeta = MusicMeta {
                            name: d.name,
                            local_path: path,
                            duration: total_duration,
                        };
                        terminal_sender_clone
                            .send(TerminalCmd::AddToList(music.clone()))
                            .expect("error");
                        player_sender_clone
                            .send(PlayerCmd::Play(music))
                            .expect("error");
                    }
                    Err(_) => {}
                }
            }
        });
        Self {
            play_sender: player_sender,
            musics: musics,
            term_receiver: terminal_receiver,
            download_list_sender: down_list_sender,
        }
    }
    #[allow(dead_code)]
    pub fn term(&self) {
        use console::Term;
        let duration = Duration::from_millis(3_000); // 10 秒

        let term = Term::stdout();
        let recv_msg = self
            .term_receiver
            .recv_timeout(duration)
            .map_err(|e| match e {
                std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                std::sync::mpsc::RecvTimeoutError::Disconnected => Error::ChannelRecvDisconnected,
            });
        match recv_msg {
            Ok(TerminalCmd::AddToList(music_meta)) => {
                self.musics.lock().unwrap().push(music_meta.clone());
                term.write_line(
                    &format!("添加歌曲《{}》至播放列表", music_meta.name).to_string(),
                )
                .unwrap();
            }
            Ok(TerminalCmd::NowPlay(song_name)) => {
                term.write_line(&format!("正在播放歌曲：《{}》", song_name).to_string())
                    .unwrap();
                //term.clear_screen().unwrap();
            }
            Err(_) => {}
        }
        thread::park();
    }
    #[allow(dead_code)]
    pub fn play(&self) {
        //let music = self.musics.get(0).expect("music play error").clone();
        //self.play_sender.send(PlayerCmd::Play(music)).expect("error");
    }

    #[allow(dead_code)]
    pub fn pause(&self) {}

    #[allow(dead_code)]
    pub fn next(&self) {
        //self.sender.send(Cmd::Next());
    }

    #[allow(dead_code)]
    pub fn stop(&self) {
        self.play_sender.send(PlayerCmd::Stop).expect("error");
    }

    #[allow(dead_code)]
    pub fn exit(&self) {
        self.play_sender.send(PlayerCmd::Exit).expect("error");
    }

    #[allow(dead_code)]
    pub fn get_download_list_sender(&self) -> Sender<Download> {
        let sender = self.download_list_sender.clone();
        sender
    }

    #[allow(dead_code)]
    fn download_file_by_url(play_url: &str) -> String {
        use reqwest::Url;
        use std::error::Error;
        use std::fs;
        use std::io::copy;
        use std::time::SystemTime;

        let save_dir: &str = "download";
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

        let file_path = format!("{}/{}.mp3", save_dir, &file_name);
        fs::create_dir_all(save_dir).unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });

        let path = Path::new(&file_path);
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}, {}", display, why.description()),
            Ok(file) => file,
        };
        let _ = copy(&mut download_file, &mut file).map_err(|e| {
            println!("copy file: {}", e.description());
        });
        file_path
    }
}
