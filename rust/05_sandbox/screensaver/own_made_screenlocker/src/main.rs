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

fn main() {
    unsafe {
        // Open connection to X server
        let display = xlib::XOpenDisplay(ptr::null());

        if display.is_null() {
            eprintln!("Unable to open X display");
            return;
        }

        let lock_duration = Duration::from_secs(5 * 60); // 5 minutes

        loop {
            let idle_time = get_idle_time(display);

            if idle_time >= lock_duration.as_millis() as u64 {
                println!("No activity detected for 5 minutes. Locking the screen...");
                if let Err(e) = Command::new("xdg-screensaver").arg("lock").status() {
                    eprintln!("Failed to execute xdg-screensaver lock: {}", e);
                }
            }

            thread::sleep(Duration::from_secs(5)); // Check every 10 seconds
        }

        // xlib::XCloseDisplay(display); // Unreachable code warning, display never closes in loop
    }
}

