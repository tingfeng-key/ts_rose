mod rodio;
fn main() {
    let sender = rodio::player_test();
    rodio::play_xinlifm(sender, "汉尼拔".to_string());
    rodio::player_ing();
}
