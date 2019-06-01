//mod codec;
mod rodio;
fn main() {
    let sender = rodio::player_start();
    rodio::play_netease(sender, "vk".to_string());
    rodio::player_ing();
}
