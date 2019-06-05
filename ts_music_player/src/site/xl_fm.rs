extern crate reqwest;
extern crate rustc_serialize;

use self::rustc_serialize::json::Json;

#[allow(dead_code)]
pub fn search(keyword: String, _page: i32) -> Vec<Json> {
    let mut get_songs_result = reqwest::Client::new()
        .get(&format!("{}{}", "http://bapi.xinli001.com/fm2/broadcast_list.json/?rows=20&is_teacher=&offset=0&speaker_id=0&q=", keyword).to_string())
        .header("content-type", "application/x-www-form-urlencoded")
        .send()
        .unwrap();
    let get_songs_body = Json::from_str(&get_songs_result.text().unwrap()).unwrap();
    let songs = get_songs_body["data"].as_array().unwrap();
    songs.to_vec()
}

#[allow(dead_code)]
pub fn get_song_playurl_by_id(music_id: String) -> String {
    let source_url = format!(
        "http://yiapi.xinli001.com/fm/media-url.mp3?id={}&flag=0",
        music_id
    );
    source_url
}
