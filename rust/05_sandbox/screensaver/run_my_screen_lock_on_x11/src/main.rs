use std::process::Command;
use std::ptr;
use x11::xlib;
use x11::xss;

fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("Cannot open display");
        }

        let mut event_base = 0;
        let mut error_base = 0;
        if xss::XScreenSaverQueryExtension(display, &mut event_base, &mut error_base) == 0 {
            panic!("XScreenSaver extension not available");
        }

        let mut event = std::mem::zeroed::<xlib::XEvent>();

        loop {
            xlib::XNextEvent(display, &mut event);

            if event.type_ == event_base + xss::ScreenSaverNotify {
                let xss_event = &*(event.as_mut() as *mut _ as *mut xss::XScreenSaverNotifyEvent);
                println!("Processing event: {:?}", xss_event);
                if xss_event.state == xss::ScreenSaverOn {
                    // Run your lock command here
                    Command::new("xflock4").spawn().expect("Failed to lock screen");
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(500));
        }

        xlib::XCloseDisplay(display);
    }
}

