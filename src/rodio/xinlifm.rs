extern crate json;
extern crate nom;
extern crate rand;
extern crate reqwest;
extern crate rustc_serialize;
use json::JsonValue;
use rustc_serialize::hex::ToHex;

use std::fs;
use std::fs::File;
use std::path::Path;

use std::error::Error;
use std::io::copy;
#[allow(dead_code)]
pub fn get_songs_data(keyword: String) -> JsonValue {
    let mut get_songs_result = reqwest::Client::new()
        .get(&format!("{}{}", "http://bapi.xinli001.com/fm2/broadcast_list.json/?rows=20&is_teacher=&offset=0&speaker_id=0&q=", keyword).to_string())
        .header("content-type", "application/x-www-form-urlencoded")
        .send()
        .unwrap();
    let get_songs_body = json::parse(&get_songs_result.text().unwrap()).unwrap();
    let songs = get_songs_body["data"].clone();
    songs
}

#[allow(dead_code)]
pub fn get_song_playurl_by_id(save_file_dir: String, music_id: String) -> String {
    let source_url = format!(
        "http://yiapi.xinli001.com/fm/media-url.mp3?id={}&flag=0",
        music_id
    );

    let file_name = format!("{}.mp3", music_id);
    let mut download_file = reqwest::Client::new().get(&source_url).send().unwrap();

    let file_path = format!("{}/{}", save_file_dir, &file_name);
    fs::create_dir_all(save_file_dir).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    });

    let path = Path::new(&file_path);
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}, {}", display, why.description()),
        Ok(file) => file,
    };
    copy(&mut download_file, &mut file).unwrap();
    file_path.to_string()
}
