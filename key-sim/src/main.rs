use clap::Parser;
use enigo::{Enigo, KeyboardControllable};
use std::fs;
use std::thread::sleep;
use std::time::Duration;

#[cfg(target_os = "windows")]
mod win_focus {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows::Win32::Foundation::*;
    use windows::Win32::UI::WindowsAndMessaging::*;

    pub fn focus_window_by_title(title: &str) -> bool {
        unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
            let mut buf = [0u16; 512];
            let len = GetWindowTextW(hwnd, &mut buf);
            if len > 0 {
                let window_title = OsString::from_wide(&buf[..len as usize]).to_string_lossy().to_string();
                let search = unsafe { &*(lparam.0 as *const String) };
                if window_title.contains(search) && IsWindowVisible(hwnd).as_bool() {
                    let _ = SetForegroundWindow(hwnd);
                    // Store result in lparam
                    return false.into();
                }
            }
            true.into()
        }
        let search = title.to_string();
        let lparam = LPARAM(&search as *const _ as isize);
        unsafe {
            let _ = EnumWindows(Some(enum_windows_proc), lparam);
        }
        // No robust way to check success, so just sleep a bit
        std::thread::sleep(std::time::Duration::from_millis(300));
        true
    }
}

/// Simple tool to simulate typing a string as keystrokes.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file containing the text to type
    #[arg(value_name = "FILE")]
    file: String,
    /// Delay between keystrokes in milliseconds
    #[arg(short, long, default_value_t = 20)]
    delay: u64,
    /// Window title to focus before typing
    #[arg(short = 'w', long)]
    window_title: Option<String>,
}

fn main() {
    let args = Args::parse();
    let text = fs::read_to_string(&args.file)
        .expect("Failed to read input file");
    #[cfg(target_os = "windows")]
    if let Some(ref title) = args.window_title {
        win_focus::focus_window_by_title(title);
    }
    let mut enigo = Enigo::new();
    for c in text.chars() {
        enigo.key_sequence(&c.to_string());
        sleep(Duration::from_millis(args.delay));
    }
}
