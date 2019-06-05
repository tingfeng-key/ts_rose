use crate::player::Player;

mod player;
mod site;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("arg error, first arg: n/x, two: song name");
    }
    let _type: &str = &args[1];
    let _keyword: String = args[2].to_string();
    println!("info: type: {}, keyword: {}", _type, _keyword);
    let player = Player::new();
    match _type {
        "x" => {
            site::play_xinlifm(player.get_download_list_sender(), _keyword);
        }
        "n" => {
            site::play_netease(player.get_download_list_sender(), _keyword);
        }
        _ => {}
    }
    player.term();
}
