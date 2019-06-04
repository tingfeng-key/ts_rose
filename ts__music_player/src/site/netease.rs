extern crate crypto;
extern crate reqwest;
extern crate rustc_serialize;

use self::rustc_serialize::json::Json;
use crypto::buffer::{ReadBuffer, WriteBuffer};
use crypto::{aes, blockmodes, buffer};
use rustc_serialize::hex::ToHex;

fn request(encode_body: &str) -> Json {
    static REQUEST_URL: &str = "http://music.163.com/api/linux/forward";
    static REQUEST_REFERER: &str = "http://music.163.com/";
    static REQUEST_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

    let mut response = reqwest::Client::new()
        .post(REQUEST_URL)
        .header("referer", REQUEST_REFERER)
        .header("content-type", REQUEST_CONTENT_TYPE)
        .body(format!("eparams={}", encode_netease_data(encode_body)))
        .send()
        .unwrap();
    let json = Json::from_str(&response.text().unwrap()).unwrap();
    json
}
#[allow(dead_code)]
pub fn get_songs_data(keyword: String) -> Vec<Json> {
    let get_songs_str =
        format!("{}{}{}",
                r#"{"method":"POST","url":"http:\/\/music.163.com\/api\/cloudsearch\/pc","params":{"s":""#,
                keyword, r#"","type":1,"offset":0,"limit":10}}""#
        );

    let get_songs_body = request(&get_songs_str);

    let songs = get_songs_body["result"]["songs"].as_array().unwrap();
    songs.to_vec()
}

#[allow(dead_code)]
pub fn get_song_play_url_by_id(music_id: String) -> String {
    let body_str = format!("{}{}{}", r#"{"method":"POST","url":"http:\/\/music.163.com\/api\/song\/enhance\/player\/url","params":{"ids":["#, music_id, r#"],"br":320000}}"#);

    let body = request(&body_str);
    let data = body.as_object().unwrap().get("data").unwrap();

    let data_arr = data.as_array().unwrap();
    let mut url: String = String::new();
    if data_arr.len() > 0 {
        let source_url = data_arr
            .get(0)
            .unwrap()
            .as_object()
            .unwrap()
            .get("url")
            .unwrap();
        url = match source_url {
            Json::String(s) => s.to_string(),
            Json::Null => url,
            _ => url,
        };
    }
    url
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
