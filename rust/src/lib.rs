unsafe extern "C" {
    pub fn _vx_sleep(secs : u64, subset_nanos : u32) -> ();
    pub fn _vx_plot_id() -> u64;
    pub fn _vx_player_send_message(session_id : u64, msg_ptr : u32, msg_size : u32) -> ();
    pub fn _vx_player_send_sound(session_id : u64, sound_id_ptr : u32, sound_id_size : u32, cat : u32, volume : f32, pitch : f32, seed : u64) -> ();
}

#[unsafe(no_mangle)]
pub fn _vx_on_player_join(session_id : u64) {
    let plot_id = unsafe { _vx_plot_id() };
    let msg = format!("<obf><black>###</></obf> <green>Welcome to</> <gold>plot</> <red><u>{}</></><pink><b>!</></>", plot_id);
    unsafe { _vx_player_send_message(session_id, msg.as_ptr() as u32, msg.as_bytes().len() as u32); }
    for i in 0..3 {
        unsafe { _vx_sleep(0, 250000000); }
        let msg = format!("<b><#ff0000>{}</></>", i);
        unsafe { _vx_player_send_message(session_id, msg.as_ptr() as u32, msg.as_bytes().len() as u32); }
    }
    unsafe { _vx_sleep(0, 250000000); }
    let id = "minecraft:entity.player.levelup";
    unsafe { _vx_player_send_sound(session_id, id.as_ptr() as u32, id.as_bytes().len() as u32, 0, 1.0, 2.0, 0); }
}


#[unsafe(no_mangle)]
pub fn _vx_alloc(size : u32, align : u32) -> u32 {
    use std::alloc::*;
    let layout = unsafe { Layout::from_size_align_unchecked(size as usize, align as usize) };
    let ptr = unsafe { alloc(layout) };
    if (ptr.is_null()) { handle_alloc_error(layout); }
    ptr as u32
}

#[unsafe(no_mangle)]
pub fn _vx_dealloc(ptr : u32, size : u32, align : u32) -> () {
    use std::alloc::*;
    let layout = unsafe { Layout::from_size_align_unchecked(size as usize, align as usize) };
    unsafe { dealloc(ptr as *mut u8, layout) };
}
