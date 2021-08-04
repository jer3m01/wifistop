//! Utils
//!
//! Holds modules per utility type

/// OS utils
pub mod os {
    /// Returns true if the current OS is Windows
    pub fn is_windows() -> bool {
        os_info::get().os_type() == os_info::Type::Windows
    }

    /// Returns true if the current OS is Android
    pub fn is_android() -> bool {
        os_info::get().os_type() == os_info::Type::Android
    }
}

/// Windows utils
pub mod windows {
    use std::process::Command;

    /// Runs a powershell command and returns the output
    pub fn run_powershell(command: &str) -> String {
        let output = Command::new("powershell.exe")
            .arg(command)
            .output()
            .expect("powershell command failed");

        String::from_utf8_lossy(&output.stdout).to_string()
    }

    /// Runs a cmd command and returns the output
    pub fn run_cmd(command: &str) -> String {
        let output = Command::new("cmd.exe")
            .arg(command)
            .output()
            .expect("cmd command failed");

        String::from_utf8_lossy(&output.stdout).to_string()
    }
}

/// *nix utils
pub mod nix {
    use std::process::Command;

    // Runs a shell command and returns the output
    pub fn run_shell(command: &str) -> String {
        let output = Command::new("sh")
            .args(&["-c", command])
            .output()
            .expect("shell command failed");

        String::from_utf8_lossy(&output.stdout).to_string()
    }

    pub fn is_elevated() -> bool {
        run_shell("echo $EUID") == "0"
    }
}

/// Android utiles
pub mod android {
    use std::process::Command;

    // Return true if busybox is detected
    pub fn has_busybox() -> bool {
        Command::new("sh").args(&["-c", "busybox"]).status().is_ok()
    }
}

/// Console logger
pub mod log {
    pub fn head() {
        println!(
            "\
██╗    ██╗██╗███████╗██╗███████╗████████╗ ██████╗ ██████╗
██║    ██║██║██╔════╝██║██╔════╝╚══██╔══╝██╔═══██╗██╔══██╗
██║ █╗ ██║██║█████╗  ██║███████╗   ██║   ██║   ██║██████╔╝
██║███╗██║██║██╔══╝  ██║╚════██║   ██║   ██║   ██║██╔═══╝
╚███╔███╔╝██║██║     ██║███████║   ██║   ╚██████╔╝██║
 ╚══╝╚══╝ ╚═╝╚═╝     ╚═╝╚══════╝   ╚═╝    ╚═════╝ ╚═╝"
        );

        for _ in 0..25 {
            print!(" ");
        }
        println!("{}", env!("CARGO_PKG_VERSION"))
    }

    #[macro_export]
    macro_rules! info {
        ($($arg:tt)*) => {
            {
                print!("ℹ️ ");
                println!($($arg)*);
            }
        }
    }

    #[macro_export]
    macro_rules! success {
        ($($arg:tt)*) => {
            {
                print!("✔️️ ");
                println!($($arg)*);
            }
        }
    }

    #[macro_export]
    macro_rules! warn {
        ($($arg:tt)*) => {
            {
                print!("⚠️️ ");
                println!($($arg)*);
            }
        }
    }

    #[macro_export]
    macro_rules! wait {
        ($($arg:tt)*) => {
            {
                println!();
                print!("⌛ ");
                println!($($arg)*);
                println!();
            }
        }
    }

    #[macro_export]
    macro_rules! done {
        ($($arg:tt)*) => {
            {
                print!("✨ ");
                println!($($arg)*);
            }
        }
    }
}
