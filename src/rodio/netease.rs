extern crate crypto;
extern crate json;
extern crate nom;
extern crate rand;
extern crate reqwest;
extern crate rustc_serialize;

use crypto::buffer::{ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer};
use json::JsonValue;
use rustc_serialize::hex::ToHex;

use std::fs;
use std::fs::File;
use std::path::Path;

use std::error::Error;
use std::io::copy;

#[allow(dead_code)]
pub fn get_songs_data(keyword: String) -> JsonValue {
    let get_songs_str =
        format!("{}{}{}",
        r#"{"method":"POST","url":"http:\/\/music.163.com\/api\/cloudsearch\/pc","params":{"s":""#,
        keyword, r#"","type":1,"offset":0,"limit":10}}""#
    );
    let mut get_songs_result = reqwest::Client::new()
        .post("http://music.163.com/api/linux/forward")
        .header("referer", "http://music.163.com/")
        .header("content-type", "application/x-www-form-urlencoded")
        .body(format!("eparams={}", encode_netease_data(&get_songs_str)))
        .send()
        .unwrap();
    let get_songs_body = json::parse(&get_songs_result.text().unwrap()).unwrap();
    let songs = get_songs_body["result"]["songs"].clone();
    songs
}

#[allow(dead_code)]
pub fn get_song_playurl_by_id(save_file_dir: String, music_id: String) -> String {
    let get_song_by_id_str = &format!("{}{}{}", r#"{"method":"POST","url":"http:\/\/music.163.com\/api\/song\/enhance\/player\/url","params":{"ids":["#, music_id, r#"],"br":320000}}"#);
    let mut get_song_by_id_result = reqwest::Client::new()
        .post("http://music.163.com/api/linux/forward")
        .header("referer", "http://music.163.com/")
        .header("content-type", "application/x-www-form-urlencoded")
        .body(format!(
            "eparams={}",
            encode_netease_data(get_song_by_id_str)
        ))
        .send()
        .unwrap();
    println!("{}", get_song_by_id_result.text().unwrap());
    let get_song_by_id_body =
        json::parse(&get_song_by_id_result.text().unwrap().clone()).expect("json parse error");
    let source_url = get_song_by_id_body["data"][0]["url"].to_string();

    let source_url_collect: Vec<&str> = source_url.split('/').collect();
    let file_name = source_url_collect[source_url_collect.len() - 1];
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

fn encode_netease_data(encode_before_data: &str) -> String {
    let key = "rFgB&h#%2?^eDg:Q";
    let mut encryptor = aes::ecb_encryptor(
        aes::KeySize::KeySize128,
        key.as_bytes(),
        blockmodes::PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = buffer::RefReadBuffer::new(encode_before_data.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = buffer::RefWriteBuffer::new(&mut buffer);
    encryptor
        .encrypt(&mut read_buffer, &mut write_buffer, true)
        .unwrap();
    final_result.extend(
        write_buffer
            .take_read_buffer()
            .take_remaining()
            .iter()
            .map(|&i| i),
    );
    final_result.to_hex().to_uppercase()
}
