use clap::Parser;
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
            let len = unsafe { GetWindowTextW(hwnd, &mut buf) };
            if len > 0 {
                let window_title = OsString::from_wide(&buf[..len as usize])
                    .to_string_lossy()
                    .to_string();
                eprintln!("{}", window_title);
                let search = unsafe { &*(lparam.0 as *const String) };
                let c1 = window_title.contains(search);
                let c2 = window_title.contains("key-sim");
                if c1 && !c2 && unsafe { IsWindowVisible(hwnd) }.as_bool() {
                    let _ = unsafe { SetForegroundWindow(hwnd) };
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
    files: Vec<String>,
    /// Delay between keystrokes in milliseconds
    #[arg(short, long, default_value_t = 20)]
    delay: u64,

    /// Simulate typing in the window with this title
    #[arg(short = 's')]
    simulate: bool,

    /// Window title to focus before typing
    #[arg(short = 'w', long)]
    window_title: Option<String>,
}

#[cfg(target_os = "windows")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use enigo::*;
    let args = Args::parse();
    if let Some(ref title) = args.window_title {
        win_focus::focus_window_by_title(title);
    }
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    for file in args.files.iter() {
        use std::io::BufRead;

        let text = fs::read(file)?.lines().flatten().collect::<Vec<_>>();
        for line in text {
            let key = match line.as_str() {
                "{ENTER}" => Some(Key::Return),
                "{TAB}" => Some(Key::Tab),
                "{ESC}" => Some(Key::Escape),
                "{BACKSPACE}" => Some(Key::Backspace),
                "{SPACE}" => Some(Key::Space),
                _ => None,
            };
            if let Some(key) = key {
                if args.simulate {
                    println!("{:?}", key);
                } else {
                    enigo.key(key, Direction::Press)?;
                    enigo.key(key, Direction::Release)?;
                    sleep(Duration::from_millis(args.delay));
                }
                continue;
            }
            //     for c in line.chars() {
            //         if args.simulate {
            //             println!("{:?}", Key::Unicode(c));
            //         } else {
            //             // enigo.key(Key::Unicode(c), Direction::Press)?;
            //             // enigo.key(Key::Unicode(c), Direction::Release)?;
            //             // let _ = enigo.text(&c.to_string());
            //             sleep(Duration::from_millis(args.delay));
            //         }
            //     }
            if args.simulate {
                println!("{}", line);
            } else {
                enigo.text(&line)?;
                sleep(Duration::from_millis(args.delay));
            }
        }
    }
    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("Error: key-sim is only supported on Windows.");
    std::process::exit(1);
}
