use std::process::Command;
use std::thread;
use std::time::Duration;
use x11::xlib;
use x11::xss::{XScreenSaverInfo, XScreenSaverQueryInfo};
use std::ptr;

fn get_idle_time(display: *mut xlib::Display) -> u64 {
    unsafe {
        let mut info: XScreenSaverInfo = std::mem::zeroed();
        XScreenSaverQueryInfo(display, xlib::XDefaultRootWindow(display), &mut info);
        info.idle
    }
}

fn logger(s:&str) {
    if let Err(e) = Command::new("/usr/bin/logger")
        .arg("-e") //  -e, --skip-empty         do not log empty lines when processing files
            .arg("-s") //  -s, --stderr             output message to standard error as well
            .arg("--")
            .arg(s)
            .status() {
                eprintln!("Failed to execute 'logger' with the msg:'{}', error: {}", e,s);
    }
}

fn main() {
    unsafe {
        // Open connection to X server
        let display = xlib::XOpenDisplay(ptr::null());

        if display.is_null() {
            eprintln!("Unable to open X display");
            return;
        }

        let lock_duration = Duration::from_secs(5 * 60); // 5 minutes
        let blank_duration = Duration::from_secs(lock_duration.as_secs() - 5); // 5 secs before 5mins are up, blank!

        loop {
            let idle_time = get_idle_time(display);

            if idle_time >= blank_duration.as_millis() as u64 {
                let s:String=format!("No activity detected for '{}' minutes '{}' seconds. Blanking the screen... press a key to unblank, it's not locked yet!",
                    blank_duration.as_secs()/60, blank_duration.as_secs() % 60);
                logger(&s);
                if let Err(e) = Command::new("blank").status() {
                    let s=format!("Failed to execute command 'blank', error: {}", e);
                    logger(&s);
                }
            }

            if idle_time >= lock_duration.as_millis() as u64 {
                let s=format!("No activity detected for '{}' minutes '{}' seconds. Locking the screen...", lock_duration.as_secs()/60, lock_duration.as_secs() % 60);
                logger(&s);
                if let Err(e) = Command::new("xdg-screensaver").arg("lock").status() {
                    let s=format!("Failed to execute 'xdg-screensaver lock', error: {}", e);
                    logger(&s);
                }
            }

            thread::sleep(Duration::from_secs(1)); // Check every N seconds
        }

        // xlib::XCloseDisplay(display); // Unreachable code warning, display never closes in loop
    }
}

