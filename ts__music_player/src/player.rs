extern crate rodio;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;
use self::rodio::source::SineWave;

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
    play_sender: Sender<Cmd>,
    term_receiver: Receiver<String>,
    musics: Vec<MusicMeta>,
    download_list_sender: Sender<Download>,
}
enum Cmd {
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
impl Player {
    pub fn new() -> Self {
        use std::io::BufReader;

        let (player_sender, player_receiver) = channel();
        let (terminal_sender, terminal_receiver) = channel();
        let (down_list_sender, down_list_receiver) = channel();

        let mut musics: Vec<MusicMeta> = Vec::new();
        let mut musics_clone = musics.clone();

        let player_sender_clone = player_sender.clone();
        let duration = Duration::from_millis(10_000); // 10 秒
        let player_thread_join_handle: thread::JoinHandle<Result<(), Error>> =
            thread::spawn(move || -> Result<(), Error> {
                let device = rodio::default_output_device().unwrap();
                let sink = rodio::Sink::new(&device);
                loop {
                    let recv_msg = player_receiver.recv_timeout(duration).map_err(|e| match e {
                        std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                        std::sync::mpsc::RecvTimeoutError::Disconnected => Error::ChannelRecvDisconnected
                    })?;
                    match recv_msg {
                        Cmd::Play(music_meta) => {
                            if sink.empty() {
                                let file = std::fs::File::open(music_meta.local_path).unwrap();
                                let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                                sink.append(source);
                                sink.play();
                                //sink.sleep_until_end();
                                terminal_sender.send(format!("正在播放：{}", music_meta.name)).expect("error");
                            }
                        }
                        Cmd::Next(music_meta) => {
                            sink.stop();
                            sink.append(music_meta.since_wave());
                            sink.play();
                        }
                        Cmd::Stop => {
                            sink.stop();
                        }
                        Cmd::Exit => {
                            sink.stop();
                        }
                    }
                }
            });

        let download_thread_join_handle = thread::spawn( move ||  {
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
                        musics_clone.push(music.clone());
                        player_sender_clone.send(Cmd::Play(music)).expect("error");
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
    pub fn term(&self) {
        let duration = Duration::from_millis(10_000); // 10 秒
        let mut t = term::stdout().unwrap();
        loop {
            let recv_msg = self.term_receiver.recv_timeout(duration).map_err(|e| match e {
                std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                std::sync::mpsc::RecvTimeoutError::Disconnected => Error::ChannelRecvDisconnected
            });
            match recv_msg {
                Ok(msg) => {
                    println!("{}", msg);
                    t.fg(term::color::GREEN).unwrap();
                    write!(t, "{}", msg).unwrap();

                    t.reset().unwrap();
                }
                Err(_) => {}
            }
        }
    }
    #[allow(dead_code)]
    pub fn play(&self) {
        let music = self.musics.get(0).expect("music play error").clone();
        self.play_sender.send(Cmd::Play(music)).expect("error");
    }

    #[allow(dead_code)]
    pub fn pause(&self) {}

    #[allow(dead_code)]
    pub fn next(&self) {
        //self.sender.send(Cmd::Next());
    }

    #[allow(dead_code)]
    pub fn stop(&self) {
        self.play_sender.send(Cmd::Stop).expect("error");
    }

    #[allow(dead_code)]
    pub fn exit(&self) {
        self.play_sender.send(Cmd::Exit).expect("error");
    }

    #[allow(dead_code)]
    pub fn add(&self, music_meta: MusicMeta) {
        self.musics.clone().push(music_meta);
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
        copy(&mut download_file, &mut file).unwrap();
        file_path
    }
}
