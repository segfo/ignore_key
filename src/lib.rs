use windows::{
    Win32::{
        Foundation::*,
        UI::WindowsAndMessaging::*,
    },
};
enum DLL {
    ProcessDetach = 0,
    ProcessAttach = 1,
    ThreadAttach = 2,
    ThreadDetach = 3,
}
impl From<u32> for DLL {
    fn from(n: u32) -> Self {
        match n {
            0 => DLL::ProcessDetach,
            1 => DLL::ProcessAttach,
            2 => DLL::ThreadAttach,
            _ => DLL::ThreadDetach,
        }
    }
}

// https://learn.microsoft.com/ja-jp/windows/win32/dlls/dllmain
#[no_mangle]
unsafe extern "system" fn DllMain(h_inst: HINSTANCE, reason: u32, _l_param: u32) -> bool {
    match DLL::from(reason) {
        DLL::ProcessAttach => {
            DLL = h_inst;
        }
        DLL::ProcessDetach => {
        }
        DLL::ThreadAttach => {}
        DLL::ThreadDetach => {}
    }
    true
}
mod hook;
use hook::*;

#[no_mangle]
unsafe extern "C" fn sethook() -> bool {
    unsafe {
        let dll: HINSTANCE = DLL;
        HOOK = match SetWindowsHookExW(WH_KEYBOARD, Some(hook_proc), dll, 0) {
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
        if HOOK.0 != 0 {
            let _ = UnhookWindowsHookEx(HOOK);
        }
    }
    true
}
