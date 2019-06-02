extern crate rodio;

use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread;
use std::time::Duration;

use rodio::source::SineWave;

#[derive(Debug, Clone)]
pub struct MusicMetadata {
    path: PathBuf,
    title: String,
    duration: Duration,
}

impl MusicMetadata {
    pub fn since_wave(&self) -> SineWave {
        SineWave::new(4000)
    }
}

#[derive(Debug, Clone)]
pub enum Request {
    Exit,
    Play(MusicMetadata),
    Next(MusicMetadata),
    Pause,
    Stop,
}

#[derive(Debug)]
pub enum Error {
    NoOutputDevice,
    IoError(io::Error),
    ChannelRecvTimeout,
    ChannelRecvDisconnected,
    RecvError,
}

pub struct Player {
    sender: SyncSender<Request>,
    receiver: Receiver<Result<(), Error>>,
    thread_join_handle: thread::JoinHandle<Result<(), Error>>,
}

impl Player {
    pub fn new() -> Result<Self, Error> {
        let (sender_a, receiver_a) = sync_channel::<Request>(2048);
        let (sender_b, receiver_b) = sync_channel::<Result<(), Error>>(2048);

        let thread_join_handle: thread::JoinHandle<Result<(), Error>> =
            thread::spawn(move || -> Result<(), Error> {
                let rx = receiver_a;
                let duration = Duration::from_millis(10_000); // 10 秒
                let device = rodio::default_output_device().ok_or(Error::NoOutputDevice)?;
                let mut sink = rodio::Sink::new(&device);

                loop {
                    let req = rx.recv_timeout(duration).map_err(|e| match e {
                        std::sync::mpsc::RecvTimeoutError::Timeout => Error::ChannelRecvTimeout,
                        std::sync::mpsc::RecvTimeoutError::Disconnected => {
                            Error::ChannelRecvDisconnected
                        }
                    })?;

                    println!("Request: {:?}", req);
                    match req {
                        Request::Exit => {
                            sink.stop();

                            let _ = sender_b.send(Ok(()));

                            break;
                        }
                        Request::Play(metadata) => {
                            // NOTE: 模拟声音样本，这里可以根据 Metadata 的信息从硬盘读取已编码的音频数据（如: mp3/ogg/opus）
                            //       然后解码成 rodio 所要求的格式。
                            if sink.empty() {
                                let source = metadata.since_wave();
                                sink.append(source);
                            }

                            sink.play();

                            let _ = sender_b.send(Ok(()));
                        }
                        Request::Next(metadata) => {
                            // NOTE: 模拟声音样本，这里可以根据 Metadata 的信息从硬盘读取已编码的音频数据（如: mp3/ogg/opus）
                            //       然后解码成 rodio 所要求的格式。
                            let source = metadata.since_wave();

                            sink.stop();

                            sink = rodio::Sink::new(&device);

                            sink.append(source);
                            sink.play();

                            let _ = sender_b.send(Ok(()));
                        }
                        Request::Pause => {
                            sink.pause();

                            let _ = sender_b.send(Ok(()));
                        }
                        Request::Stop => {
                            sink.stop();

                            let _ = sender_b.send(Ok(()));
                        }
                    }
                }

                Ok(())
            });

        Ok(Self {
            sender: sender_a,
            receiver: receiver_b,
            thread_join_handle: thread_join_handle,
        })
    }

    pub fn play(&mut self, metadata: MusicMetadata) -> Result<(), Error> {
        let _ = self.sender.send(Request::Play(metadata));

        self.receiver.recv().map_err(|_| Error::RecvError)?
    }

    pub fn next(&mut self, metadata: MusicMetadata) -> Result<(), Error> {
        let _ = self.sender.send(Request::Next(metadata));

        self.receiver.recv().map_err(|_| Error::RecvError)?
    }

    pub fn pause(&mut self) -> Result<(), Error> {
        let _ = self.sender.send(Request::Pause);

        self.receiver.recv().map_err(|_| Error::RecvError)?
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        let _ = self.sender.send(Request::Stop);

        self.receiver.recv().map_err(|_| Error::RecvError)?
    }
}

fn main() {
    let mut player = Player::new().unwrap();
    let fake_metadata = MusicMetadata {
        path: PathBuf::from("/home/Users/Musics/foo.opus"),
        title: "乐曲".to_string(),
        duration: Duration::from_millis(60_000),
    };

    player.play(fake_metadata).unwrap();

    // NOTE: 确保主线程不要退出了。
    thread::sleep(Duration::from_millis(1000));
}
