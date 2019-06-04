use crate::player::Player;

mod player;
mod site;
fn main() {
    let player = Player::new();
    site::play_netease(player.get_download_list_sender(), "渡我不渡她".to_string());
    player.term();
}
