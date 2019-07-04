use minimp3::Decoder;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::fs::File;
use std::path::Path;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::time::Duration;

struct Sound {
    volume: f32,
    sender: SyncSender<f32>,
    receiver: Receiver<f32>,
}

impl Sound {
    pub fn init() -> Self {
        let (sender, receiver) = sync_channel(2048);

        Self {
            volume: 0.25,
            sender,
            receiver,
        }
    }
}
impl AudioCallback for Sound {
    type Channel = f32;
    fn callback(&mut self, out: &mut [f32]) {
        let duration = Duration::from_millis(1_000); // 秒
        for x in out.iter_mut() {
            let recv_msg = self.receiver.recv_timeout(duration);
            match recv_msg {
                Ok(v) => {
                    *x = v;
                }
                Err(_) => {}
            };
        }
    }
}

pub fn run() {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(2), // mono
        samples: None,     // default sample size
    };

    let mut decoder = Decoder::new(File::open(Path::new("./resource/1559457768.mp3")).unwrap());

    let sound = Sound::init();
    let sound_send_clone = sound.sender.clone();
    let device = audio_subsystem
        .open_playback(None, &desired_spec, |spec| {
            // initialize the audio callback
            sound
        })
        .unwrap();

    // Start playback
    device.resume();

    let mut data = Vec::new();
    loop {
        let frame = decoder.next_frame();
        match frame {
            Ok(mut f) => {
                for sample in f.data.chunks_mut(f.channels as usize) {
                    for a in sample.iter_mut() {
                        data.append(a.clone() as f32 / std::i16::MAX as f32);
                        /*sound_send_clone
                        .send(a.clone() as f32 / std::i16::MAX as f32)
                        .expect("send decode error");*/
                    }
                }
            }
            Err(_) => {
                println!("123");
                break;
            }
        }
    }

    println!("1256");
    device.pause();
    // Play for 2 seconds
    std::thread::sleep(Duration::from_millis(10000));
    println!("789");
}
