#[macro_use]
extern crate lazy_static;
mod rodio;
fn main() {
    let sender = rodio::player_test();
    rodio::play_netease(sink, musics, "心若止水".to_string());
    //rodio::play_xinlifm(sink, musics, "汉尼拔".to_string());
}
