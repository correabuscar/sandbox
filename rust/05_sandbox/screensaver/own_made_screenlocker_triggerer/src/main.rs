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
        //let lock_duration = Duration::from_secs(1 * 60); // FIXME: 1 minutes, for tests!
        //let lock_duration = Duration::from_secs(23); // FIXME: temp tests!
        //const BLANK_THIS_MANY_SECONDS_BEFORE_LOCKING:u64=10;
        const RETRY_TIME_ON_FAIL : Duration=Duration::from_secs(5); // retry to blank or lock if either or both failed to run, after this much delay.
        const BLANK_THIS_MUCH_TIME_BEFORE_LOCKING : Duration=Duration::from_secs(10);
        //assert!(lock_duration > Duration::from_secs(BLANK_THIS_MUCH_TIME_BEFORE_LOCKING));
        assert!(lock_duration > BLANK_THIS_MUCH_TIME_BEFORE_LOCKING);
        let blank_duration: Duration = lock_duration - BLANK_THIS_MUCH_TIME_BEFORE_LOCKING;
        assert!(blank_duration <= lock_duration);
        //TODO: handle when they're the same value, better.

        if let Some(env_value) = std::env::var_os("PATH") {
            let paths = std::env::split_paths(&env_value);
            let mut how_many:usize=0;
            for path in paths {
                // PATH=" " is ok because there can be a dir with name " " aka 1 space.
                println!("Path: {:?}", path);
                if ! path.as_os_str().is_empty() {
                    how_many+=1;
                }
            }
            if how_many == 0 {
                panic!("No paths in PATH={:?}", env_value);// output includes ""
            }
        } else {
            panic!("PATH variable not found in environment. Very likely the blank/lock commands will fail then!");
        }

        //needed to prevent re-blanking and re-locking while already blanked/locked
        let mut last_idle_duration:Duration=Duration::from_millis(0);
        let mut is_blanked=false;
        let mut is_locked=false;
        loop {
            let idle_time_ms:u64 = get_idle_time(display);
            let idle_duration = Duration::from_millis(idle_time_ms);
            if idle_duration < last_idle_duration {
                // was idle and now isn't anymore.
                eprintln!("no longer idle (but this could've been a while ago, while we were sleeping here)");
                is_locked=false;
                is_blanked=false;
            }
            //FIXME: the next idle_duration gotten above via get_idle_time() might(?) be higher than the saved one here due to the below sleep(), see if this is true or not, else it might think it's still locked! Maybe save this only after the sleeps? or i dno, add the sleep() to it? unclear, must think.
            last_idle_duration=idle_duration;

            //if idle_time_ms >= blank_duration.as_millis() as u64 {
            if !is_blanked && idle_duration >= blank_duration {
                let s:String=format!("No activity detected for '{}' minutes '{}' seconds. Blanking the screen... press a key to unblank, it's not locked yet!",
                    blank_duration.as_secs()/60, blank_duration.as_secs() % 60);
                logger(&s);
                //TODO: dedup the cmd execution and return the bool for is_blanked/is_locked
                match Command::new("blank").status() { // this blanks and exits, doesn't hang around until non-idle!
                    Ok(exit_status) => {
                        is_blanked=exit_status.success();
                        if !is_blanked {
                            if let Some(code)=exit_status.code() {
                                let s=format!("Failed to blank, process exited with exit code: '{}'", code);
                                logger(&s);
                            }
                        }
                    },
                    Err(e) => {
                        let s=format!("Failed to execute command 'blank', error: {}", e);
                        logger(&s);
                    },
                }//match
            }

            //if idle_time_ms >= lock_duration.as_millis() as u64 {
            if !is_locked && idle_duration >= lock_duration {
                let s=format!("No activity detected for '{}' minutes '{}' seconds. Locking the screen...", lock_duration.as_secs()/60, lock_duration.as_secs() % 60);
                logger(&s);
                // NOPE:this currently locks and hangs until unlocked, then exits, because it's using 'alock'; ie. this call to status() is blocking while the screen is locked. Wait a minute, this doesn't block! why did I think it did?! yep i made it not block a while back! but what if it did block?! does alock blank it if i unidle and not unlock it? it does NOT!
                // FIXME: if i move mouse or otherwise not unlock, the screen remains on!, maybe run this in a separate thread? and keep track of whether or not it exited, if not don't run it again but assume it's still locked!
                // FIXME: can't assume this hangs/blocks or that it doesn't! and since it uses `alock` cmd in my case, internally (eg. script), then maybe also have to `pgrep alock` to check if it's still alive, to detect if it's still locked.
                match Command::new("xdg-screensaver").arg("lock").status() {
                    Ok(exit_status) => {
                        is_locked=exit_status.success();
                        if !is_locked {
                            if let Some(code)=exit_status.code() {
                                let s=format!("Failed to lock via 'xdg-screensaver lock', it exited with exit code: '{}'", code);
                                logger(&s);
                            }
                        }
                    },
                    Err(e) => {
                        let s=format!("Failed to execute command 'xdg-screensaver lock', error: {}", e);
                        logger(&s);
                    },
                }//match
            }

            if idle_duration < blank_duration {
                // if there's more time until the time to blank, wait that amount of time, this would be slightly over by some ms due to the code between get_idle_time() call and the thread::sleep below.
                let duration_difference = blank_duration - idle_duration;

                let total_seconds = duration_difference.as_secs();
                let hours = total_seconds / 3600;
                let minutes = (total_seconds % 3600) / 60;
                let seconds = total_seconds % 60;
                eprintln!("Sleeping for {:02}:{:02}:{:02} h:m:s while waiting for minimum time to blank", hours, minutes, seconds);
                //eprintln!("Sleeping for {:?}.", duration_difference); // this is seconds
                //thread::sleep(Duration::from_secs(1)); // Check every N seconds
                thread::sleep(duration_difference);
            } else {
                //if blanked or locked, we would need to wait at least this amount until next blank
                //if idle_duration > lock_duration {
                if is_locked {
                    //FIXME: well, but it can be locked but awoken so no longer blanked, yet still locked! maybe monitor blanking in a different thread? based on idle time! but we still need to know if it's locked and after how many seconds to blank WHILE locked, if it's unblanked!
                    eprintln!("Sleeping for {:?} while locked, the minimum time to blank if were just un-idled!", blank_duration);
                    //already locked, so wait at least this long until a potential re-blank would be needed
                    thread::sleep(blank_duration);
                    // bad workaround attempt:
                    //eprintln!("Actually no, sleeping only {:?} because we might be locked but unidle thus unblanked and thus in need of re-blanking!", BLANK_THIS_MUCH_TIME_BEFORE_LOCKING);
                    //thread::sleep(BLANK_THIS_MUCH_TIME_BEFORE_LOCKING);
                //} else {
                } else if is_blanked {
                    //FIXME: if locking failed but blanking succeeded(before), we get here and wait this long until lock retry!
                    eprintln!("Sleeping for {:?} while blanked, the minimum time to wait until it's time to lock!", BLANK_THIS_MUCH_TIME_BEFORE_LOCKING);
                    //this is just a blank, thus only few seconds left until a lock is needed:
                    thread::sleep(BLANK_THIS_MUCH_TIME_BEFORE_LOCKING);
                } else {
                    // we're here if we tried to blank but failed (eg. to execute the 'blank' cmd)
                    // but also if we tried to lock after that, and that failed too!
                    eprintln!("Sleeping for {:?} before continuing...", RETRY_TIME_ON_FAIL);
                    thread::sleep(RETRY_TIME_ON_FAIL);
                }
            }
        }

        // xlib::XCloseDisplay(display); // Unreachable code warning, display never closes in loop
    }
}

