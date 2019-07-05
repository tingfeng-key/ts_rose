use std::fs::File;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MusicMeta {
    name: String,
    local_path: String,
}

//下载结构体
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
    current_music_index: usize,
    current_music_duration: Option<Duration>,
}

//播放器指令
#[allow(dead_code)]
enum PlayerCmd {
    Stop,
    Play(MusicMeta),
    Exit,
    Hard,
}

//控制台指令
#[allow(dead_code)]
enum TerminalCmd {
    AddToList(MusicMeta),
    NowPlay(String),
    Next,
    Hard,
}
#[allow(dead_code)]
impl Player {
    pub fn new() -> Self {
        //创建播放器线程异步通道
        let (player_sender, player_receiver) = channel();
        //创建控制台线程异步通道
        let (terminal_sender, terminal_receiver) = channel();
        //创建下载资源线程异步通道
        let (down_list_sender, down_list_receiver) = channel();

        //初始化播放列表
        let musics: Arc<Mutex<Vec<MusicMeta>>> = Arc::new(Mutex::new(Vec::new()));

        //创建播放器线程
        let _player_thread_join_handle = thread::spawn(move || {
            let mut audio = crate::engine::audio::Audio::new();
            let mut timer = Instant::now();
            loop {
                //计时器，用于播放下一个音频
                match audio.duration {
                    Some(d) => {
                        if d.as_nanos() <= timer.elapsed().as_nanos() {
                            timer = Instant::now();
                            terminal_sender.send(TerminalCmd::Next).expect("error");
                        }
                    }
                    None => {
                        timer = Instant::now();
                    }
                }
                let recv_msg = player_receiver.recv_timeout(Duration::from_millis(10_000));

                //接受消息
                match recv_msg {
                    //播放
                    Ok(PlayerCmd::Play(music_meta)) => {
                        terminal_sender
                            .send(TerminalCmd::NowPlay(music_meta.clone().name))
                            .expect("error");
                        audio.play(music_meta.clone().local_path.as_str());
                        terminal_sender.send(TerminalCmd::Hard).expect("error");
                    }
                    //停止
                    Ok(PlayerCmd::Stop) => {}
                    //退出
                    Ok(PlayerCmd::Exit) => {}
                    //心跳
                    OK(PlayerCmd::Hard) => {
                        terminal_sender.send(TerminalCmd::Hard).expect("error");
                    }
                    Err(_) => {}
                }
            }
        });

        let terminal_sender_clone = terminal_sender.clone();

        //创建下载资源线程
        let _download_thread_join_handle = thread::spawn(move || loop {
            let recv_msg = down_list_receiver.recv_timeout(duration);
            match recv_msg {
                Ok(download) => {
                    let d: Download = download;
                    let path = Self::download_file_by_url(d.url.as_str());
                    let music: MusicMeta = MusicMeta {
                        name: d.name,
                        local_path: path,
                    };
                    terminal_sender_clone
                        .send(TerminalCmd::AddToList(music.clone()))
                        .expect("error");
                }
                Err(_) => {}
            }
        });

        Self {
            play_sender: player_sender,
            musics,
            term_receiver: terminal_receiver,
            download_list_sender: down_list_sender,
            current_music_index: 0,
            current_music_duration: None,
        }
    }

    //控制台输入输出
    pub fn term(&mut self) {
        loop {
            let recv_msg = self
                .term_receiver
                .recv_timeout(Duration::from_millis(3_000));
            match recv_msg {
                //添加歌曲
                Ok(TerminalCmd::AddToList(music_meta)) => {
                    Self::write_to_console(&format!(
                        "添加歌曲《{}》至播放列表",
                        music_meta.name
                    ));

                    //如果当前播放列表为空，自动播放
                    if self.musics.lock().unwrap().len() == 0 {
                        self.play_sender
                            .send(PlayerCmd::Play(music_meta.clone()))
                            .expect("send error");
                    }
                    //添加歌曲到播放列表
                    self.musics.lock().unwrap().push(music_meta.clone());
                }

                //当前播放歌曲
                Ok(TerminalCmd::NowPlay(song_name)) => {
                    Self::write_to_console(&format!("正在播放歌曲：《{}》", song_name));

                    self.play_sender.send(PlayerCmd::Hard).expect("send error");
                }

                //下一首
                Ok(TerminalCmd::Next) => {
                    Self::write_to_console(&format!("下一首歌曲：《{}》", music.name));

                    //获取下一首歌曲
                    let musics = self.musics.lock().unwrap();
                    self.current_music_index += 1;
                    let music = musics.get(self.current_music_index).unwrap();

                    //发送数据到播放器线程
                    self.play_sender
                        .send(PlayerCmd::Play(music.clone()))
                        .expect("send error");
                }

                //心跳
                Ok(TerminalCmd::Hard) => {
                    self.play_sender.send(PlayerCmd::Hard).expect("send error");
                }

                Err(_) => {
                    println!("exit!");
                }
            }
        }
    }

    //控制台输出
    fn write_to_console(s: &str) {
        use console::Term;
        let term = Term::stdout();
        term.write_line(s).unwrap();
    }

    //返回下载线程发送器
    pub fn get_download_list_sender(&self) -> Sender<Download> {
        let sender = self.download_list_sender.clone();
        sender
    }

    //根据url下载音频
    fn download_file_by_url(play_url: &str) -> String {
        use reqwest::Url;
        use std::fs;
        use std::io::copy;
        use std::time::SystemTime;

        let save_dir: &str = "download"; //保存资源目录
        let duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let url = Url::parse(play_url).unwrap();
        let file_name = duration.as_secs();
        let mut download_file = reqwest::Client::new()
            .get(url)
            .send()
            .expect("request error");

        //文件路径
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
        //保存资源(复制）
        let _ = copy(&mut download_file, &mut file).map_err(|e| {
            println!("copy file: {}", e.description());
        });
        file_path
    }
}
