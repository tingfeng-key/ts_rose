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

    let mut current_frame = decoder.next_frame().unwrap();

    let mut current_frame_data_index = 0;
    let mut next_value = || {
        let mut s = 0f32;
        if current_frame_data_index+2 < current_frame.data.len() {
            s = (
                (current_frame.data[current_frame_data_index] as f32 +
                    current_frame.data[current_frame_data_index + 1] as f32
                ) / current_frame.sample_rate as f32
            ) / (current_frame.channels as f32);
            current_frame_data_index += 2;
            println!("{}, {}", current_frame.channels, current_frame.bitrate);
        }else {
            match decoder.next_frame() {
                Ok(f) => {
                    current_frame = f;
                    current_frame_data_index = 0;
                    s = (
                        (current_frame.data[current_frame_data_index] as f32 +
                            current_frame.data[current_frame_data_index + 1] as f32
                        ) / current_frame.sample_rate as f32
                    ) / (current_frame.channels as f32);
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

/*fn test_ffi() {
    use std::path::Path;
    unsafe {
        //println!("{}", doubler(1));
        //avpriv_io_delete("./test")
    }
}*/
