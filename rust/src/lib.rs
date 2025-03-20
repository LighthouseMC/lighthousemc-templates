use lhmc_api_rs::prelude::*;
use core::time::Duration;


#[unsafe(no_mangle)]
pub fn _vx_on_player_join(session_id : u64) {
    let player    = Player::from_session_id(session_id);
    let plot_id   = Plot::id();
    let plot_name = Plot::name_fmt();

    player.send_chat(&format!("<obf><black>###</></obf> <green>Welcome to</> <gold>plot</> <red><u>{}</></><pink><b>!</></>", plot_id));
    player.send_chat(&format!("Plot Name: {}", sanitise_text(&format!("{:?}", plot_name))));

    for i in 0..3 {
        sleep(Duration::from_millis(500));
        player.send_chat(&format!("<b><#ff0000>{}</></>", i));
    }
    sleep(Duration::from_millis(500));

    player.play_sound("entity.player.levelup", SoundCat::Player, 1.0, 2.0, 0);
}
