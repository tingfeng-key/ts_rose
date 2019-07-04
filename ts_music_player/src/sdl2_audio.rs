use minimp3::Decoder;
use sdl2::audio::{AudioCallback, AudioSpecDesired};
use std::fs::File;
use std::mem::drop;
use std::path::Path;
use std::time::Duration;

#[derive(Clone, Debug)]
struct Sound {
    volume: f32,
    data: Vec<f32>,
    current_count: usize,
}

impl Sound {
    pub fn init(data: Vec<f32>) -> Self {
        Self {
            volume: 0.25,
            data,
            current_count: 0,
        }
    }
}
impl AudioCallback for Sound {
    type Channel = f32;
    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            //            println!("{}", self.data.len());
            if self.data.len() == 0 {
                *x = 0f32;
            } else {
                *x = self.data[self.current_count];

                self.current_count += 1;
                if self.current_count == self.data.len() {
                    let mut v = Vec::new();
                    v.push(0f32);
                    self.data = v;
                    self.current_count = 0;
                }
            };
        }
    }
}

pub fn run(path: &str) {
    let sdl_context = sdl2::init().unwrap();
    let audio_subsystem = sdl_context.audio().unwrap();

    let mut data = Vec::new();
    let sound_path = Path::new(path);
    let sound_file = File::open(sound_path).unwrap();
    let mut decoder = Decoder::new(sound_file);
    let mut song_sample_rate = 44_100;
    let mut song_channels = 2;
    let duration = mp3_duration::from_path(&sound_path).unwrap();
    println!("{:#?}, {}", duration, duration.as_nanos() as u64);
    loop {
        let frame = decoder.next_frame();
        match frame {
            Ok(mut f) => {
                song_channels = f.channels;
                song_sample_rate = f.sample_rate;
                for sample in f.data.chunks_mut(f.channels as usize) {
                    for a in sample.iter_mut() {
                        data.push(a.clone() as f32 / std::i16::MAX as f32);
                    }
                }
            }
            Err(_) => {
                println!(
                    "song_sample_rate: {}, song_channels: {}",
                    song_sample_rate, song_channels
                );
                break;
            }
        }
    }

    let desired_spec = AudioSpecDesired {
        freq: Some(song_sample_rate),
        channels: Some(song_channels as u8), // mono
        samples: None,                       // default sample size
    };

    let sound = Sound::init(data);
    let device = audio_subsystem
        .open_playback(None, &desired_spec, |_spec| {
            // initialize the audio callback
            sound
        })
        .unwrap();

    // Start playback
    device.resume();
    // Play for 2 seconds
    std::thread::sleep(Duration::from_nanos(duration.as_nanos() as u64));
    drop(device);
}
