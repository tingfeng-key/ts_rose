extern crate rodio;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
use self::rodio::source::SineWave;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct MusicMeta {
    name: String,
    local_path: String,
    duration: u64,
}
impl MusicMeta {
    #[allow(dead_code)]
    pub fn remove() {}
    pub fn since_wave(&self) -> SineWave {
        SineWave::new(4000)
    }
}
#[derive(Debug)]
pub struct Download {
    pub url: String,
    pub name: String
}
pub struct Player {
    play_sender: Sender<PlayerCmd>,
    term_receiver: Receiver<TerminalCmd>,
    musics: Arc<Mutex<Vec<MusicMeta>>>,
    download_list_sender: Sender<Download>,
}
enum PlayerCmd {
    Next(MusicMeta),
    Stop,
    Play(MusicMeta),
    Exit,
}
#[derive(Debug)]
enum Error {
    NoOutputDevice,
    ChannelRecvTimeout,
    ChannelRecvDisconnected,
    RecvError,
}

enum TerminalCmd {
    AddToList(MusicMeta),
    NowPlay(String),
}
impl Player {
    pub fn new() -> Self {
        use std::io::BufReader;

        let (player_sender, player_receiver) = channel();
        let (terminal_sender, terminal_receiver) = channel();
        let (down_list_sender, down_list_receiver) = channel();

        let musics:Arc<Mutex<Vec<MusicMeta>>> = Arc::new(Mutex::new(Vec::new()));//Arc::new<Mutex::new<Vec<MusicMeta>>>

        let player_sender_clone = player_sender.clone();
        let terminal_sender_clone = terminal_sender.clone();
        let duration = Duration::from_millis(10_000); // 10 秒

        let device = rodio::default_output_device().unwrap();
        let sink_init = Arc::new(Mutex::new(rodio::Sink::new(&device)));
        let mut sink = sink_init.clone();
        let mut sink_sleep_thread: thread::JoinHandle<Result<(), Error>> = thread::spawn(move || -> Result<(), Error> {
            Ok(())
        });
        let _player_thread_join_handle: thread::JoinHandle<Result<(), Error>> =
            thread::spawn(move || -> Result<(), Error> {
                loop {
                    let recv_msg = player_receiver.recv_timeout(duration).map_err(|e| match e {
                        std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                        std::sync::mpsc::RecvTimeoutError::Disconnected => Error::ChannelRecvDisconnected
                    })?;
                    match recv_msg {
                        PlayerCmd::Play(music_meta) => {
                            let sink_clone = sink.lock().unwrap();
                            if sink_clone.empty() {
                                let file = std::fs::File::open(music_meta.local_path).unwrap();
                                let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                                sink_clone.append(source);
                                terminal_sender.send(TerminalCmd::NowPlay(music_meta.name)).expect("error");

                                let sink_sleep_thread_sink_clone = sink.clone();
                                //sink_sleep_thread.thread().unpark();
                                sink_sleep_thread = thread::spawn(move || -> Result<(), Error> {
                                    sink_sleep_thread_sink_clone.lock().unwrap().sleep_until_end();
                                    //thread::park();
                                    Ok(())
                                });
                            }
                        }
                        PlayerCmd::Next(music_meta) => {
                            //let sink_clone = sink.lock().unwrap();
                            println!("123");
                            sink.lock().unwrap().stop();
                            println!("123");
                            sink_sleep_thread.thread().unpark();
                            println!("123");
                            sink = Arc::new(Mutex::new(rodio::Sink::new(&device)));
                            let file = std::fs::File::open(music_meta.local_path).unwrap();
                            let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                            sink.lock().unwrap().append(source);
                            terminal_sender.send(TerminalCmd::NowPlay(music_meta.name)).expect("error");
                            let sink_sleep_thread_sink_clone = sink.clone();
                            sink_sleep_thread = thread::spawn(move || -> Result<(), Error> {
                                sink_sleep_thread_sink_clone.lock().unwrap().sleep_until_end();
                                //thread::park();
                                Ok(())
                            });
                        }
                        PlayerCmd::Stop => {
                            sink.lock().unwrap().stop();
                        }
                        PlayerCmd::Exit => {
                            sink.lock().unwrap().stop();
                        }
                    }
                }
                Ok(())
            });

        let _download_thread_join_handle = thread::spawn( move ||  -> Result<(), Error> {
            loop {
                let recv_msg = down_list_receiver.recv_timeout(duration).map_err(|e| match e {
                    std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                    std::sync::mpsc::RecvTimeoutError::Disconnected => Error::ChannelRecvDisconnected
                });
                match recv_msg {
                    Ok(download) => {
                        let d: Download = download;
                        let path = Self::download_file_by_url(d.url.as_str());
                        let total_duration = mp3_duration::from_path(path.clone())
                            .unwrap()
                            .as_secs();
                        let music: MusicMeta = MusicMeta {
                            name: d.name,
                            local_path: path,
                            duration: total_duration
                        };
                        terminal_sender_clone.send(TerminalCmd::AddToList(music.clone())).expect("error");
                        player_sender_clone.send(PlayerCmd::Play(music)).expect("error");
                    }
                    Err(_) => {}
                }
            }
            Ok(())
        });
        Self {
            play_sender: player_sender,
            musics: musics,
            term_receiver: terminal_receiver,
            download_list_sender: down_list_sender,
        }
    }
    pub fn term(&self) {
        use std::thread;
        use std::time::Duration;
        use console::Term;
        let duration = Duration::from_millis(3_000); // 10 秒

        let term = Term::stdout();
        loop {
            let recv_msg = self.term_receiver.recv_timeout(duration).map_err(|e| match e {
                std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                std::sync::mpsc::RecvTimeoutError::Disconnected => Error::ChannelRecvDisconnected
            });
            match recv_msg {
                Ok(TerminalCmd::AddToList(music_meta)) => {
                    self.musics.lock().unwrap().push(music_meta.clone());
                    term.write_line(&format!("添加歌曲《{}》至播放列表", music_meta.name).to_string()).unwrap();
                    //thread::sleep(Duration::from_millis(2000));
                    //term.clear_screen().unwrap();
                }
                Ok(TerminalCmd::NowPlay(song_name)) => {
                    term.write_line(&format!("正在播放歌曲：《{}》", song_name).to_string()).unwrap();
                    //thread::sleep(Duration::from_millis(2000));
                    //term.clear_screen().unwrap();
                }
                Err(_) => {
                    let input_str = term.read_line();
                    let input_command = match input_str {
                        Ok(s) => s,
                        Err(_) => String::new()
                    };
                    println!("input: {}", input_command.as_str());
                    match input_command.as_str() {
                        "n" => {
                            let musics = self.musics.lock().unwrap();
                            println!("play next: {}", musics.len());
                            if musics.len() > 3 {
                                let music = musics.get(3).unwrap();
                                self.play_sender.send(PlayerCmd::Next(music.clone())).expect("send error");
                            }
                        }
                        "m" => {}
                        _ => {}
                    }
                }
                _ => {}
            }
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
    pub fn add(&self, music_meta: MusicMeta) {

        //self.musics.clone().push(music_meta);
    }

    #[allow(dead_code)]
    pub fn remove_one(&self) {}

    #[allow(dead_code)]
    pub fn remove_all(&self) {}

    #[allow(dead_code)]
    pub fn get_download_list_sender(&self) -> Sender<Download> {
        let sender = self.download_list_sender.clone();
        sender
    }

    #[allow(dead_code)]
     fn download_file_by_url(play_url: &str) -> String {
        use std::error::Error;
        use std::fs;
        use std::fs::File;
        use std::io::copy;
        use std::time::SystemTime;
        use reqwest::Url;

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
