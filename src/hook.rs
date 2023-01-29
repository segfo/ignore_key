use once_cell::unsync::*;
use std::os::windows::ffi::OsStringExt;
use std::{ffi::OsString, sync::Mutex};
use windows::Win32::{
    Foundation::*,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};
#[link_section = ".shared"]
pub static mut hook: HHOOK = HHOOK(0);
#[link_section = ".shared"]
pub static mut g_dll: HINSTANCE = HINSTANCE(0);
#[link_section = ".shared"]
static mut g_ignore: Lazy<Mutex<isize>> = Lazy::new(|| Mutex::new(0));

#[no_mangle]
pub extern "system" fn hook_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let vkey = wparam.0;
    if HC_ACTION as i32 == ncode {
        let ignore = unsafe { g_ignore.lock().unwrap() };
        if vkey == VK_LCONTROL.0 as usize || vkey == 'V' as usize {
            return LRESULT(*ignore);
        }
    }
    unsafe { CallNextHookEx(hook, ncode, wparam, lparam) }
}
#[no_mangle]
unsafe extern "C" fn ignore_ctrl_v() {
    let mut ignore = unsafe { g_ignore.lock().unwrap() };
    *ignore = 1;
}
#[no_mangle]
unsafe extern "C" fn notice_ctrl_v() {
    let mut ignore = unsafe { g_ignore.lock().unwrap() };
    *ignore = 0;
}

unsafe fn u16_ptr_to_string(ptr: *const u16) -> OsString {
    let len = (0..).take_while(|&i| *ptr.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(ptr, len);

    OsString::from_wide(slice)
}
