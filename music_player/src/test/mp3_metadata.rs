extern crate mp3_metadata;
extern crate rtag;
use rtag::frame::types::FrameHeaderFlag;
use rtag::frame::*;
use rtag::metadata::MetadataReader;
use rtag::metadata::Unit;
#[allow(dead_code)]
fn get_info() {
    let file = "rodio/B4cBAFvgk-WAU_J7ACsW8L7l4so395.mp3";
    let meta = mp3_metadata::read_from_file(file).expect("File error");

    println!("Number of frames: {}", meta.frames.len());
    println!("\nShowing 5 first frames information:");
    for frame in meta.frames[0..meta.frames.len()].iter() {
        println!("========== NEW FRAME ==========");
        println!("{:#?}", frame);
    }
    println!("{:#?}", meta.tag);
    println!("\n========== TAGS ==========");
    if let Some(tag) = meta.tag {
        println!("title: {}", tag.title);
        println!("artist: {}", tag.artist);
        println!("album: {}", tag.album);
        println!("year: {}", tag.year);
        println!("comment: {}", tag.comment);
        println!("genre: {:?}", tag.genre);
    } else {
        println!("No tag");
    }
}
