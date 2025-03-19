use core::time::Duration;
use std::borrow::Cow;


unsafe extern "C" {
    pub fn _vx_sleep(secs : u64, subsec_nanos : u32) -> ();
}

pub fn sleep(dur : Duration) -> () {
    unsafe { _vx_sleep(dur.as_secs(), dur.subsec_nanos()); }
}


unsafe extern "C" {
    pub fn _vx_plot_id() -> u64;
    pub fn _vx_plot_name_fmt(out_ptr : u32) -> ();
}

pub struct Plot(());
impl Plot {

    pub fn id() -> u64 {
        unsafe { _vx_plot_id() }
    }

    pub fn name_fmt() -> String {
        let mut ptr_size = (0u32, 0u32,);
        unsafe { _vx_plot_name_fmt(&mut ptr_size as *mut _ as u32) };
        let (ptr, size,) = ptr_size;
        unsafe { String::from_raw_parts(ptr as *mut u8, size as usize, size as usize) }
    }

}


unsafe extern "C" {
    pub fn _vx_player_send_chat(session_id : u64, msg_ptr : u32, msg_size : u32) -> ();
    pub fn _vx_player_send_sound(session_id : u64, sound_id_ptr : u32, sound_id_size : u32, cat : u32, volume : f32, pitch : f32, seed : u64) -> ();
}

pub struct Player { session_id : u64 }
impl Player {

    pub fn from_session_id(session_id : u64) -> Self {
        Self { session_id }
    }

    pub fn send_chat(&self, msg : &str) -> () {
        unsafe { _vx_player_send_chat(self.session_id, msg.as_ptr() as u32, msg.as_bytes().len() as u32); }
    }

    pub fn play_sound(&self, sound_id : &str, cat : SoundCat, volume : f32, pitch : f32, seed : u64) -> () {
        unsafe { _vx_player_send_sound(self.session_id, sound_id.as_ptr() as u32, sound_id.as_bytes().len() as u32, cat as u32, volume, pitch, seed); }
    }

}


#[repr(u32)]
pub enum SoundCat {
    Master  = 0,
    Music   = 1,
    Records = 2,
    Weather = 3,
    Blocks  = 4,
    Hostile = 5,
    Neutral = 6,
    Player  = 7,
    Ambient = 8,
    Voice   = 9
}


pub fn sanitise_text(text : &str) -> String {
    text.chars().map(|ch| match (ch) {
        '\\' => Cow::Borrowed("\\\\"),
        '<'  => Cow::Borrowed("\\<"),
        _    => Cow::Owned(ch.to_string())
    }).collect()
}



#[unsafe(no_mangle)]
pub unsafe fn _vx_alloc(size : u32, align : u32) -> u32 {
    use std::alloc::*;
    let layout = unsafe { Layout::from_size_align_unchecked(size as usize, align as usize) };
    let ptr = unsafe { alloc(layout) };
    if (ptr.is_null()) { handle_alloc_error(layout); }
    ptr as u32
}

#[unsafe(no_mangle)]
pub unsafe fn _vx_dealloc(ptr : u32, size : u32, align : u32) -> () {
    use std::alloc::*;
    let layout = unsafe { Layout::from_size_align_unchecked(size as usize, align as usize) };
    unsafe { dealloc(ptr as *mut u8, layout) };
}
