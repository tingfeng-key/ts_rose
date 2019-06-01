//mod codec;
mod rodio;
use std::env;
fn main() {
    let sender = rodio::player_start();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("arg error, first arg: n/x, two: song name");
    }
    let _type: &str = &args[1];
    let _keyword: String = args[2].to_string();
    println!("info: type: {}, keyword: {}", _type, _keyword);
    match _type {
        "x" => {
            rodio::play_xinlifm(sender, _keyword);
        }
        "n" => {
            rodio::play_netease(sender, _keyword);
        }
        _ => {}
    }

    rodio::player_ing();
}
