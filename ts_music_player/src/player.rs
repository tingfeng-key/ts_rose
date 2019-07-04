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
    play_status: Arc<Mutex<PLAYSTATUS>>,
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
enum PlayStatus {
    PLAYING,
    STOP,
}
struct PLAYSTATUS {
    status: PlayStatus,
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

        let _musics: Arc<Mutex<Vec<MusicMeta>>> = Arc::new(Mutex::new(Vec::new())); //Arc::new<Mutex::new<Vec<MusicMeta>>>

        let terminal_sender_clone = terminal_sender.clone();
        let duration = Duration::from_millis(1000_000); // 10 秒

        let current_play_status = Arc::new(Mutex::new(PLAYSTATUS {
            status: PlayStatus::STOP,
        }));
        let current_play_status_clone = current_play_status.clone();
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

                    match recv_msg {
                        PlayerCmd::Play(music_meta) => {
                            terminal_sender
                                .send(TerminalCmd::NowPlay(music_meta.clone().name))
                                .expect("error");
                            current_play_status_clone.lock().unwrap().status = PlayStatus::PLAYING;
                            crate::sdl2_audio::run(music_meta.clone().local_path.as_str());
                            println!("finash...");
                        }
                        PlayerCmd::Next(music_meta) => {
                            terminal_sender
                                .send(TerminalCmd::NowPlay(music_meta.name))
                                .expect("error");
                        }
                        PlayerCmd::Stop => {}
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
                    }
                    Err(_) => {}
                }
            }
        });

        Self {
            play_sender: player_sender,
            musics: _musics,
            term_receiver: terminal_receiver,
            download_list_sender: down_list_sender,
            play_status: current_play_status,
        }
    }
    #[allow(dead_code)]
    pub fn term(&self) {
        use console::Term;
        let duration = Duration::from_millis(3_000); // 10 秒

        let term = Term::stdout();
        loop {
            let recv_msg = self
                .term_receiver
                .recv_timeout(duration)
                .map_err(|e| match e {
                    std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                    std::sync::mpsc::RecvTimeoutError::Disconnected => {
                        Error::ChannelRecvDisconnected
                    }
                });
            match recv_msg {
                Ok(TerminalCmd::AddToList(music_meta)) => {
                    if self.musics.lock().unwrap().len() == 0 {
                        self.play_sender
                            .send(PlayerCmd::Play(music_meta.clone()))
                            .expect("send error");
                    }
                    self.musics.lock().unwrap().push(music_meta.clone());
                    term.write_line(
                        &format!("添加歌曲《{}》至播放列表", music_meta.name)
                            .to_string(),
                    )
                    .unwrap();
                    //term.clear_screen().unwrap();
                }
                Ok(TerminalCmd::NowPlay(song_name)) => {
                    term.write_line(
                        &format!("正在播放歌曲：《{}》", song_name).to_string(),
                    )
                    .unwrap();
                }
                Err(_) => {
                    let input_str = term.read_line();
                    let input_command = match input_str {
                        Ok(s) => s,
                        Err(_) => String::new(),
                    };
                    println!("input: {}", input_command.as_str());
                    match input_command.as_str() {
                        "n" => {
                            let musics = self.musics.lock().unwrap();
                            println!("play next: {}", musics.len());
                            self.play_status.lock().unwrap().status = PlayStatus::STOP;
                            if musics.len() > 3 {
                                let music = musics.get(1).unwrap();
                                self.play_sender
                                    .send(PlayerCmd::Play(music.clone()))
                                    .expect("send error");
                            }
                        }
                        "m" => {}
                        _ => {}
                    }
                }
            }
        }
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
