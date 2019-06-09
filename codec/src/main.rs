use cpal::Sample;

fn main() {
    let path = "resource/1559457768.mp3"; //1559457768
    use mp3_duration;
    use std::path::Path;

    //let path = Path::new("music.mp3");
    let total_second = mp3_duration::from_path(&path).unwrap().as_secs();
    let minute = format!("{:02}", total_second / 60);
    let second = format!("{:02}", total_second % 60);
    println!("{}, {}", minute, second);
    setup_stream();
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

    /*loop {
        match decoder.next_frame() {
            Ok(Frame { data, sample_rate, channels, .. }) => {
                println!("Decoded {} samples", data.len() / channels)
            },
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }*/
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
