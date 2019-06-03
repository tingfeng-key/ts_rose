extern crate rodio;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
use std::time::Duration;
use self::rodio::source::SineWave;

struct MusicMeta {
    id: String,
    name: String,
    url: String,
    source_type: Option<String>,
    source_id: Option<String>,
    local_path: Option<String>,
    duration: Option<u64>,
}
impl MusicMeta {
    pub fn remove() {}
    pub fn since_wave(&self) -> SineWave {
        SineWave::new(4000)
    }
}
struct Player {
    sender: SyncSender<Cmd>,
    term_sender: SyncSender<String>,
    term_receiver: Receiver<String>,
    musics: Vec<MusicMeta>,
}
pub enum Cmd {
    Next(MusicMeta),
    Stop,
    Play(MusicMeta),
    Exit,
}
#[derive(Debug)]
pub enum Error {
    NoOutputDevice,
    ChannelRecvTimeout,
    ChannelRecvDisconnected,
    RecvError,
}
impl Player {
    pub fn new() -> Self {
        let (player_sender, player_receiver) = sync_channel(2048);
        let (input_sender, output_receiver) = sync_channel(2048);

        let player_thread_join_handle: thread::JoinHandle<Result<(), Error>> =
            thread::spawn(move || -> Result<(), Error> {
                let duration = Duration::from_millis(10_000); // 10 秒
                let device = rodio::default_output_device().unwrap();
                let sink = rodio::Sink::new(&device);
                loop {
                    let recv_msg = player_receiver.recv_timeout(duration).expect("player receiver error");
                    match recv_msg {
                        Cmd::Play(music_meta) => {
                            if sink.empty() {
                                sink.append(music_meta.since_wave());
                                sink.play();
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
        Self {
            sender: player_sender,
            musics: Vec::new(),
            term_sender: input_sender,
            term_receiver: output_receiver,
        }
    }
    pub fn term(&self) {
        loop {
            let duration = Duration::from_millis(10_000); // 10 秒
            let _recv_msg = self.term_receiver.recv_timeout(duration).expect("term receiver error");
            /*match recv_msg {
                Ok(msg) => {
                    println!(msg);
                }
                Err(_) => {}
            }*/
        }
    }
    #[allow(dead_code)]
    pub fn play(&self) {}

    #[allow(dead_code)]
    pub fn pause(&self) {}

    #[allow(dead_code)]
    pub fn next(&self) {}

    #[allow(dead_code)]
    pub fn stop(&self) {}

    #[allow(dead_code)]
    pub fn exit(&self) {}

    #[allow(dead_code)]
    pub fn add(&self, music_meta: MusicMeta) {}

    #[allow(dead_code)]
    pub fn remove_one(&self) {}

    #[allow(dead_code)]
    pub fn remove_all(&self) {}
}
