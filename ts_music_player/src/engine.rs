use cpal;
use cpal::{EventLoop, StreamId};
use std::sync::mpsc::{channel, sync_channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub struct AudioEngine {
    pub output_sender: Option<Sender<EngineInput>>,
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
    Exit,
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

        let (output_sender, output_receiver) = channel();
        let duration = Duration::from_millis(1_000); // 秒

        let _out_thread_handle = thread::spawn(move || {
            event_loop.run(move |_, data| match data {
                cpal::StreamData::Output {
                    buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
                } => {
                    for sample in buffer.chunks_mut(format.channels as usize) {
                        for out in sample.iter_mut() {
                            let recv_msg = output_receiver
                                .recv_timeout(duration)
                                .map_err(|e| match e {
                                    std::sync::mpsc::RecvTimeoutError::Timeout => {
                                        Error::ChannelRecvTimeout
                                    }
                                    std::sync::mpsc::RecvTimeoutError::Disconnected => {
                                        Error::ChannelRecvDisconnected
                                    }
                                })
                                .unwrap();
                            match recv_msg {
                                EngineInput::Input(v) => {
                                    *out = v;
                                }
                                EngineInput::Exit => {
                                    panic!("exit");
                                }
                            };
                        }
                    }
                }
                _ => (),
            });
        });
        Self {
            output_sender: Some(output_sender),
            input_sender: None,
        }
    }
}
impl Drop for AudioEngine {
    fn drop(&mut self) {
        self.output_sender.clone().unwrap().send(EngineInput::Exit);
        //是在这里暂停然后删除流
        //self.event_loop_handle.destroy_stream(self.stream_id)
        println!("delete");
    }
}
