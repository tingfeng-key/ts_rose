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
        panic!("the args error, first arg: n/x/q, two arg: song name");
    }
    let play_type: &str = &args[1];
    let keyword: String = args[2].to_string();

    println!("info: type: {}, keyword: {}", play_type, keyword);

    let mut player = Player::new();
    match play_type {
        //心理FM资源
        "x" => {
            site::play_xinlifm(player.get_download_list_sender(), keyword);
        }
        //网易云资源
        "n" => {
            site::play_netease(player.get_download_list_sender(), keyword);
        }
        //QQ音乐资源
        "q" => {
            site::play_tencent(player.get_download_list_sender(), keyword);
        }
        _ => {}
    }
    //调用输入输出线程
    player.term();
}
