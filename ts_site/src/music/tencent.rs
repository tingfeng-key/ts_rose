extern crate reqwest;
extern crate rustc_serialize;

use self::rustc_serialize::json::Json;
pub fn search(keyword: String, page: i32) -> Vec<Json> {
    let mut get_songs_result = reqwest::Client::new()
        .get(&format!("http://c.y.qq.com/soso/fcgi-bin/search_for_qq_cp?w={}&p={}&n=10&format=json", keyword, page).to_string())
        .header("referer", "http://m.y.qq.com")
        .header("user-agent", "Mozilla/5.0 (iPhone; CPU iPhone OS 9_1 like Mac OS X) AppleWebKit/601.1.46 (KHTML, like Gecko) Version/9.0 Mobile/13B143 Safari/601.1")
        .send()
        .unwrap();
    let get_songs_body = Json::from_str(&get_songs_result.text().unwrap()).unwrap();
    let songs = get_songs_body["data"]["song"]["list"].as_array().unwrap();
    songs.to_vec()
}

pub fn get_song_playurl_by_id(music_id: String, vkey: &str) -> String {
    let source_url = format!(
        "http://dl.stream.qqmusic.qq.com/{}{}.mp3?vkey={}&guid=5150825362&fromtag=1",
        "M500", music_id, vkey
    );
    source_url
}

pub fn get_vkey() -> String {
    let mut get_songs_result = reqwest::Client::new()
        .get(&format!("{}", "http://base.music.qq.com/fcgi-bin/fcg_musicexpress.fcg?json=3&guid=5150825362&format=json").to_string())
        .header("referer", "http://y.qq.com")
        .send()
        .unwrap();
    let get_songs_body = Json::from_str(&get_songs_result.text().unwrap()).unwrap();
    let vkey = get_songs_body["key"].as_string().unwrap();
    vkey.to_string()
}