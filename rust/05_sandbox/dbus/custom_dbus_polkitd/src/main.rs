use std::{error::Error, future::pending};
use zbus::{connection, interface};

struct PowerManager;
/// $ busctl call SERVICE OBJECT INTERFACE METHOD
/// ^ also seen in /var/tmp/portage/lxqt-base/liblxqt-2.0.0-r1/work/liblxqt-2.0.0/lxqtpower/lxqtpowerproviders.cpp:138
/// $ busctl --system --verbose call org.freedesktop.ConsoleKit /org/freedesktop/ConsoleKit/Manager org.freedesktop.ConsoleKit.Manager CanPowerOff
/// dbusCall: QDBusInterface is invalid "org.freedesktop.ConsoleKit" "/org/freedesktop/ConsoleKit/Manager" "org.freedesktop.ConsoleKit.Manager" "CanPowerOff"

#[interface(name = "org.freedesktop.ConsoleKit.Manager")]
impl PowerManager {
    //XXX: because returning nothing means "yes", in lxqt-leave or so, well it's actually in /var/tmp/portage/lxqt-base/liblxqt-2.0.0-r1/work/liblxqt-2.0.0/lxqtpower/lxqtpowerproviders.cpp:158
    //otherwise it's next to impossible to cleanly return a QLatin1String("yes") from here. (aka QL1S("yes") in liblxqt-2.0.0/lxqtpower/lxqtpowerproviders.cpp )
    fn can_power_off(&self) {}
    fn can_reboot(&self) {}
    // systemd: "Reboot" = "Rejected send message, 3 matched rules; type=\"method_call\", sender=\":1.122\" (uid=1000 pid=90753 comm=\"/var/tmp/portage/lxqt-base/lxqt-session-2.0.0/work\") interface=\"org.freedesktop.ConsoleKit.Manager\" member=\"Reboot\" error name=\"(unset)\" requested_reply=\"0\" destination=\"org.freedesktop.ConsoleKit\" (uid=0 pid=88903 comm=\"./target/debug/custom_dbus_polkitd\")"
    // dbusCall: QDBusInterface is invalid "org.freedesktop.ConsoleKit" "/org/freedesktop/ConsoleKit/Manager" "org.freedesktop.ConsoleKit.Manager" "Reboot"
    fn reboot(&self) {
        exec(
            "Rebooting",
            &[
            "-r",
            "now",
            "reboot issued by lxqt-leave",
            //TODO: make sure only lxqt-leave can call us?
            ],
        );
    }
    // systemd: "PowerOff" = "Rejected send message, 3 matched rules; type=\"method_call\", sender=\":1.129\" (uid=1000 pid=91946 comm=\"/var/tmp/portage/lxqt-base/lxqt-session-2.0.0/work\") interface=\"org.freedesktop.ConsoleKit.Manager\" member=\"PowerOff\" error name=\"(unset)\" requested_reply=\"0\" destination=\"org.freedesktop.ConsoleKit\" (uid=0 pid=91930 comm=\"./target/debug/custom_dbus_polkitd\")"
    // dbusCall: QDBusInterface is invalid "org.freedesktop.ConsoleKit" "/org/freedesktop/ConsoleKit/Manager" "org.freedesktop.ConsoleKit.Manager" "PowerOff"
    fn power_off(&self) {
        exec(
            "Shutting down",
            &[
            "-h",
            "-P",
            "-t",
            "3",
            "now",
            "shutdown issued by lxqt-leave",
            //TODO: make sure only lxqt-leave can call us? and above for reboot too!
            ],
        );
    }
} //impl

fn exec(action: &str, args: &[&str]) {
    //XXX: we use 'sudo' with properly set sudoers, to not require running this dbus service as root!
    println!("{}", action);
    let status = std::process::Command::new("/usr/bin/sudo")
        .arg("--")
        .arg("/sbin/shutdown")
        //.arg("/bin/echo")
        .args(args)
        .status();
    match status {
        Ok(status) if status.success() => {
            println!("Command executed ok!");
        },
        Ok(status) => {
            println!("{} failed with status: '{}'", action, status);
        },
        Err(err) => {
            println!("Failed to execute the {} command, error: '{}'", action, err);
        },
    };
}

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _conn = connection::Builder::system()?
        .name("org.freedesktop.ConsoleKit")?
        .serve_at("/org/freedesktop/ConsoleKit/Manager", PowerManager)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}


