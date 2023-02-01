use windows::{
    core::*,
    Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
    },
};
enum DLL {
    PROCESS_DETACH = 0,
    PROCESS_ATTACH = 1,
    THREAD_ATTACH = 2,
    THREAD_DETACH = 3,
}
impl From<u32> for DLL {
    fn from(n: u32) -> Self {
        match n {
            0 => DLL::PROCESS_DETACH,
            1 => DLL::PROCESS_ATTACH,
            2 => DLL::THREAD_ATTACH,
            _ => DLL::THREAD_DETACH,
        }
    }
}

// https://learn.microsoft.com/ja-jp/windows/win32/dlls/dllmain
#[no_mangle]
unsafe extern "system" fn DllMain(h_inst: HINSTANCE, reason: u32, l_param: u32) -> bool {
    match DLL::from(reason) {
        DLL::PROCESS_ATTACH => {
            g_dll = h_inst;
        }
        DLL::PROCESS_DETACH => {
        }
        DLL::THREAD_ATTACH => {}
        DLL::THREAD_DETACH => {}
    }
    true
}
mod hook;
use hook::*;

#[no_mangle]
unsafe extern "C" fn sethook() -> bool {
    unsafe {
        let dll: HINSTANCE = g_dll;
        hook = match SetWindowsHookExW(WH_KEYBOARD, Some(hook_proc), dll, 0) {
            Ok(handle) => handle,
            Err(_) => {
                return false;
            }
        };
    }
    true
}
#[no_mangle]
unsafe extern "C" fn unhook() -> bool {
    unsafe {
        if hook.0 != 0 {
            let r = UnhookWindowsHookEx(hook);
        }
    }
    true
}
fn msg(caption: &str, msg: &str) {
    unsafe {
        MessageBoxW(
            HWND::default(),
            &HSTRING::from(msg),
            &HSTRING::from(caption),
            MB_OK,
        );
    }
}
