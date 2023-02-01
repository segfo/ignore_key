use once_cell::unsync::*;
use std::sync::RwLock;
use windows::Win32::UI::Input::Ime::*;
use windows::Win32::{
    Foundation::*,
    UI::{Input::KeyboardAndMouse::*, WindowsAndMessaging::*},
};
#[link_section = ".shared"]
pub static mut hook: HHOOK = HHOOK(0);
#[link_section = ".shared"]
pub static mut g_dll: HINSTANCE = HINSTANCE(0);
#[link_section = ".shared"]
static mut g_ignore: Lazy<RwLock<isize>> = Lazy::new(|| RwLock::new(0));

#[no_mangle]
pub extern "system" fn hook_proc(ncode: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let vkey = wparam.0;
    if HC_ACTION as i32 == ncode {
        if vkey == 'V' as usize {
            // CTRL+VならとにかくIMEを無効化する
            unsafe {
                let lctrl = GetAsyncKeyState(VK_LCONTROL.0 as i32) as u16 & 0x8000 > 0;
                if lctrl {
                    let hwnd = GetForegroundWindow();
                    let ctx = ImmGetContext(hwnd);
                    ImmSetOpenStatus(ctx, false);
                    ImmReleaseContext(hwnd, ctx);
                }
            }
            let ignore = { *unsafe { g_ignore.read().unwrap() } };
            return LRESULT(ignore);
        }
    }
    unsafe { CallNextHookEx(hook, ncode, wparam, lparam) }
}
#[no_mangle]
unsafe extern "C" fn ignore_ctrl_v() {
    let mut ignore = unsafe { g_ignore.write().unwrap() };
    *ignore = 1;
}
#[no_mangle]
unsafe extern "C" fn notice_ctrl_v() {
    let mut ignore = unsafe { g_ignore.write().unwrap() };
    *ignore = 0;
}
