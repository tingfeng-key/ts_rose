use cpal;
use std::sync::mpsc::{sync_channel, Sender, SyncSender};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub struct AudioEngine {
    pub output_sender: Option<SyncSender<EngineInput>>,
    input_sender: Option<Sender<f32>>,
}

#[allow(dead_code)]
#[derive(Debug)]
enum Error {
    NoOutputDevice,
    ChannelRecvTimeout,
    ChannelRecvDisconnected,
    RecvError,
}
pub enum EngineInput {
    Input(f32),
}
impl AudioEngine {
    #[allow(dead_code)]
    pub fn run_input_device() {
        let device = cpal::default_input_device().expect("Failed to get default output device");

        let format = device
            .default_input_format()
            .expect("Failed to get default output format");

        let event_loop = cpal::EventLoop::new();

        let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

        event_loop.play_stream(stream_id.clone());

        event_loop.run(move |_, data| match data {
            cpal::StreamData::Input {
                buffer: cpal::UnknownTypeInputBuffer::F32(_buffer),
            } => {}
            _ => (),
        });
    }

    #[allow(dead_code)]
    pub fn run_output_device() -> Self {
        let device = cpal::default_output_device().expect("Failed to get default output device");

        let format = device
            .default_output_format()
            .expect("Failed to get default output format");

        let event_loop = cpal::EventLoop::new();

        let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

        event_loop.play_stream(stream_id.clone());

        let (output_sender, output_receiver) = sync_channel::<EngineInput>(2048);
        let duration = Duration::from_millis(1_000); // 秒

        let _out_thread_handle = thread::spawn(move || -> Result<(), Error> {
            event_loop.run(move |_, data| match data {
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        for out in sample.iter_mut() {
                            let recv_msg =
                                output_receiver.recv_timeout(duration).map_err(|e| match e {
                                    std::sync::mpsc::RecvTimeoutError::Timeout => {
                                        Error::ChannelRecvTimeout
                                    }
                                    std::sync::mpsc::RecvTimeoutError::Disconnected => {
                                        Error::ChannelRecvDisconnected
                                    }
                                });
                            match recv_msg {
                                Ok(EngineInput::Input(v)) => {
                                    *out = v;
                                }
                                Err(_) => {}
                            };
                        }
                    }
                }
                _ => (),
            });
            Ok(())
        });
        Self {
            output_sender: Some(output_sender),
            input_sender: None,
        }
    }
}
