use windows::Win32::UI::WindowsAndMessaging::{CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, GetMessageW, DispatchMessageW, TranslateMessage, MSG, HHOOK, WH_KEYBOARD, WM_KEYDOWN};
use windows::Win32::Foundation::{HINSTANCE, WPARAM, LPARAM, LRESULT, HWND};
use windows::Win32::System::Threading::GetCurrentThreadId;

use std::{ptr::null_mut, convert::TryFrom, mem::transmute};

static mut FOO: u32 = 0u32;
/*
DOCS
https://learn.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew
https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew
https://stackoverflow.com/questions/75870904/how-to-correctly-set-a-wh-keyboard-hook-procedure-using-setwindowshookexw-in-rus

*/
//https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644984(v=vs.85)
unsafe extern "system" fn keyboard_callback(code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    //let p: *const KBDLLHOOKSTRUCT = unsafe { mem::transmute(l_param) };
    dbg!(wp);
    if code >= 0 && wp.0 as u32 == WM_KEYDOWN { //u32::try_from(wp).unwrap()
        //wp is the kvirtual keycode
        //let kbcp = lp as *const KBDLLHOOKSTRUCT;
        //let kb_code: KBDLLHOOKSTRUCT = transmute(lp); //might be just a i32 or u3
        dbg!(lp);
    }
    CallNextHookEx(HHOOK::default(), code, wp, lp)
}

fn message_loop() {
    let mut message = MSG::default();

    unsafe {
        //stalling on this statement
        dbg!("ok1");
        dbg!(GetMessageW(&mut message, HWND::default(), 0, 0));
        dbg!("ok2");
        while GetMessageW(&mut message, HWND::default(), 0, 0).as_bool() {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

fn main() {
    let hook = unsafe {SetWindowsHookExW(
        WH_KEYBOARD,
        Some(keyboard_callback),
        HINSTANCE::default(),
        GetCurrentThreadId()
    ).unwrap()};
    message_loop();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    unsafe{UnhookWindowsHookEx(hook)};
}

/*
pub unsafe fn SetWindowsHookExW<P0>(
    idhook: WINDOWS_HOOK_ID,
    lpfn: HOOKPROC,
    hmod: P0,
    dwthreadid: u32
) -> Result<HHOOK>where
    P0: IntoParam<HINSTANCE>,






*/