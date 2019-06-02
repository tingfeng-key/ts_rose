extern crate rodio;
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
use std::time::Duration;
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
}
struct Player {
    sender: SyncSender,
    receiver: Receiver,
    term_sender: SyncSender,
    term_receiver: Receiver,
    musics: Vec<MusicMeta>,
}
enum Cmd {
    Next,
    Stop,
    Play,
    Exit,
}
impl Player {
    pub fn new() -> Self {
        let (player_sender, player_receiver) = sync_channel(1);
        let (input_sender, output_receiver) = sync_channel(2);

        let player_thread_join_handle: thread::JoinHandle<Result<(), Error>> =
            thread::spawn(move || -> Result<(), Error> {
                let rx = player_receiver;
                let duration = Duration::from_millis(10_000); // 10 秒
                let device = rodio::default_output_device().unwrap();
                let mut sink = rodio::Sink::new(&device);
                loop {
                    let recv_msg = rx.recv_timeout(duration);
                    match recv_msg {
                        Cmd::Play => {}
                        Cmd::Next => {}
                        Cmd::Stop => {}
                        Cmd::Exit => {}
                    }
                }
            });
        Ok(Self {
            sender: player_receiver,
            receiver: player_sender,
            musics: Vec::new(),
            term_sender: input_sender,
            term_receiver: output_receiver,
        })
    }
    pub fn term(&self) {
        loop {
            let duration = Duration::from_millis(10_000); // 10 秒
            let recv_msg = rx.recv_timeout(duration);
            match recv_msg {
                Ok(msg) => {
                    println!(msg);
                }
                Err(_e) => {}
            }
        }
    }
    pub fn play(&self) {}
    pub fn pause(&self) {}
    pub fn next(&self) {}
    pub fn stop(&self) {}
    pub fn exit(&self) {}
    pub fn add(&self, mut music_meta: MusicMeta) {}
    pub fn remove_one(&self) {}
    pub fn remove_all(&self) {}
}
