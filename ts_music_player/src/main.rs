mod engine;
mod player;
mod site;

fn main() {
    run_player();
}

#[allow(dead_code)]
fn run_player() {
    use crate::player::Player;

    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("arg error, first arg: n/x, two: song name");
    }
    let play_type: &str = &args[1];
    let keyword: String = args[2].to_string();

    println!("info: type: {}, keyword: {}", play_type, keyword);

    let mut player = Player::new();
    match play_type {
        "x" => {
            site::play_xinlifm(player.get_download_list_sender(), keyword);
        }
        "n" => {
            site::play_netease(player.get_download_list_sender(), keyword);
        }
        "q" => {
            site::play_tencent(player.get_download_list_sender(), keyword);
        }
        _ => {}
    }
    //调用输入输出线程
    player.term();
}
