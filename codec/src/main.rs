//use cpal::Sample;

/*extern crate libc;

extern {
    //fn doubler(input: libc::c_int) -> libc::c_int;
    pub fn avpriv_io_delete(s: libc::c_char);
}*/
const LATENCY_MS: f32 = 150.0;
fn main() {
    extern crate cpal;
    use cpal::{StreamData, UnknownTypeOutputBuffer};
    use cpal::EventLoop;

    use minimp3::Decoder;
    use std::fs::File;
    use minimp3::Frame;
    use minimp3::Error;
    use std::path::Path;

    let mut decoder = Decoder::new(File::open(
        Path::new("resource/hxfs.mp3")
    ).unwrap());//1559457768

    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate_c = format.sample_rate.0 as f32;
    let mut current_frame = decoder.next_frame().unwrap();

    let mut current_frame_data_index = 0;
    let mut next_value = || {
        let mut s = 0f32;
        if current_frame_data_index+2 < current_frame.data.len() {
            s = (current_frame.data[current_frame_data_index] as f32 / current_frame.sample_rate as f32);
            current_frame_data_index += 1;
            s = (s + ((current_frame.data[current_frame_data_index] as f32 / current_frame.sample_rate as f32))) / 2.0;
            current_frame_data_index += 1;
            //println!("{}, {}", current_frame.channels, current_frame.bitrate);
        }else {
            match decoder.next_frame() {
                Ok(f) => {
                    current_frame = f;
                    current_frame_data_index = 0;
                    s = (current_frame.data[current_frame_data_index] as f32 / current_frame.sample_rate as f32);
                    current_frame_data_index += 1;
                    s = (s + ((current_frame.data[current_frame_data_index] as f32 / current_frame.sample_rate as f32))) / 2.0;
                    current_frame_data_index += 1;
                }
                Err(e) => panic!("err")
            };
        }
        s
    };
    event_loop.run(move |_, data| {
        match data {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (next_value() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = next_value();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            _ => (),
        }
    });
}

fn test_ffi() {
    use std::path::Path;
    unsafe {
        //println!("{}", doubler(1));
        //avpriv_io_delete("./test")
    }
}
/*
fn get_mp3_duration() {
    let path = "resource/1559457768.mp3"; //1559457768
    use mp3_duration;
    use std::path::Path;

    //let path = Path::new("music.mp3");
    let total_second = mp3_duration::from_path(&path).unwrap().as_secs();
    let minute = format!("{:02}", total_second / 60);
    let second = format!("{:02}", total_second % 60);
    println!("{}, {}", minute, second);
}
fn test_cpal() {
    use cpal::{StreamData, UnknownTypeOutputBuffer};
    use cpal::EventLoop;
    let event_loop = EventLoop::new();

    let device = cpal::default_output_device().expect("no output device available");
    let mut supported_formats_range = device.supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range.next()
        .expect("no supported format?!")
        .with_max_sample_rate();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);
    event_loop.run(move |_stream_id, mut stream_data| match stream_data {
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::U16(mut buffer),
        } => {
            for elem in buffer.iter_mut() {
                *elem = u16::max_value() / 2;
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::I16(mut buffer),
        } => {
            for elem in buffer.iter_mut() {
                *elem = 0;
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::F32(mut buffer),
        } => {
            for elem in buffer.iter_mut() {
                *elem = 0.0;
            }
        }
        _ => (),
    });
}

fn setup_stream( ) {
    use minimp3::Decoder;
    use std::fs::File;
    use minimp3::Frame;
    use minimp3::Error;

    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format  = device.default_output_format().expect("Failed to get default output format");

    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let mut decoder = Decoder::new(File::open(
        "resource/1559457768.mp3",
    ).unwrap());

    *//*loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate, channels, .. }) => {
                println!("Decoded {} samples", data.len() / channels)
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }*//*
    event_loop.run(move |_, data_t| {
        match data_t {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    match decoder.next_frame() {
                        Ok(Frame { data, sample_rate, channels, .. }) => {
                            for out in sample.iter_mut() {
                                *out = ((data.len() / channels)) as f32;
                            }
                            println!("Decoded {} samples, {}", data.len() / channels, sample_rate)
                        },
                        Err(Error::Eof) => break,
                        Err(e) => panic!("{:?}", e),
                    }
                }
            }
            _ => (),
        }
    });
}
*/
