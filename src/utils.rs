use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

pub fn get_username() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string())
}

pub fn get_hostname() -> String {
    if let Ok(hostname) = fs::read_to_string("/proc/sys/kernel/hostname") {
        hostname.trim().to_string()
    } else if let Ok(output) = Command::new("hostname").output() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        "unknown".to_string()
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn format_uptime(uptime_seconds: u64) -> String {
    let days = uptime_seconds / 86400;
    let hours = (uptime_seconds % 86400) / 3600;
    let minutes = (uptime_seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn get_shell() -> String {
    env::var("SHELL")
        .map(|shell| {
            Path::new(&shell)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&shell)
                .to_string()
        })
        .unwrap_or_else(|_| "unknown".to_string())
}

pub fn get_terminal() -> String {
    // Check if running in Termux first
    if env::var("TERMUX_VERSION").is_ok() || 
       env::var("PREFIX").map(|p| p.contains("com.termux")).unwrap_or(false) {
        return "Termux".to_string();
    }

    // Try various environment variables that might contain terminal info
    let terminal_vars = [
        "TERM_PROGRAM",
        "TERMINAL_EMULATOR", 
        "TERM",
        "COLORTERM",
    ];

    for var in &terminal_vars {
        if let Ok(value) = env::var(var) {
            if !value.is_empty() && value != "xterm-256color" && value != "screen" {
                return value;
            }
        }
    }

    // Try to detect from parent process
    if let Ok(output) = Command::new("ps")
        .args(&["-p", &env::var("PPID").unwrap_or_default(), "-o", "comm="])
        .output()
    {
        let parent = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !parent.is_empty() {
            return parent;
        }
    }

    "unknown".to_string()
}

pub fn count_packages() -> u64 {
    let mut count = 0;

    // Try different package managers
    let package_managers = [
        ("pacman", &["-Q"] as &[&str]),
        ("dpkg", &["-l"]),
        ("rpm", &["-qa"]),
        ("emerge", &["--list-installed"]),
        ("pkg", &["list-installed"]), // Termux pkg manager
        ("pkg", &["info"]), // FreeBSD pkg manager (fallback)
        ("brew", &["list"]),
        ("port", &["installed"]),
        ("nix-env", &["-q"]),
        ("flatpak", &["list"]),
        ("snap", &["list"]),
    ];

    for (manager, args) in &package_managers {
        if let Ok(output) = Command::new(manager).args(*args).output() {
            if output.status.success() {
                let lines = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .count();
                count += lines as u64;
                
                // For Termux pkg, we found packages, so break to avoid double counting
                if *manager == "pkg" && *args == &["list-installed"] && lines > 0 {
                    break;
                }
            }
        }
    }

    count
}

#[cfg(unix)]
pub fn get_file_count(path: &str) -> u64 {
    if let Ok(entries) = fs::read_dir(path) {
        entries.filter_map(|entry| entry.ok()).count() as u64
    } else {
        0
    }
}

#[cfg(windows)]
pub fn get_file_count(path: &str) -> u64 {
    if let Ok(entries) = fs::read_dir(path) {
        entries.filter_map(|entry| entry.ok()).count() as u64
    } else {
        0
    }
}

pub fn get_cpu_cores() -> usize {
    num_cpus::get()
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}

pub fn center_text(text: &str, width: usize) -> String {
    let text_len = text.len();
    if text_len >= width {
        return text.to_string();
    }
    
    let padding = (width - text_len) / 2;
    format!("{}{}", " ".repeat(padding), text)
}

pub fn is_running_in_container() -> bool {
    // Check for common container indicators
    Path::new("/.dockerenv").exists() ||
    env::var("container").is_ok() ||
    fs::read_to_string("/proc/1/cgroup")
        .map(|content| content.contains("docker") || content.contains("lxc"))
        .unwrap_or(false)
}

pub fn get_desktop_environment() -> String {
    // Check various DE environment variables
    let de_vars = [
        "XDG_CURRENT_DESKTOP",
        "DESKTOP_SESSION", 
        "GDMSESSION",
        "KDE_SESSION_VERSION",
        "GNOME_DESKTOP_SESSION_ID",
    ];

    for var in &de_vars {
        if let Ok(value) = env::var(var) {
            if !value.is_empty() {
                return value.to_lowercase();
            }
        }
    }

    // Try to detect from running processes
    let de_processes = [
        "gnome-session",
        "kde-session", 
        "xfce4-session",
        "lxsession",
        "mate-session",
        "cinnamon-session",
    ];

    for process in &de_processes {
        if Command::new("pgrep").arg(process).output().map(|o| o.status.success()).unwrap_or(false) {
            return process.replace("-session", "").to_string();
        }
    }

    "unknown".to_string()
}

pub fn get_window_manager() -> String {
    // Common window managers to check for
    let wm_processes = [
        "i3", "sway", "bspwm", "dwm", "awesome", "xmonad", 
        "openbox", "fluxbox", "blackbox", "fvwm", "jwm",
        "herbstluftwm", "qtile", "spectrwm", "cwm", "2bwm",
    ];

    for wm in &wm_processes {
        if Command::new("pgrep").arg(wm).output().map(|o| o.status.success()).unwrap_or(false) {
            return wm.to_string();
        }
    }

    // Check environment variables
    if let Ok(wm) = env::var("WINDOW_MANAGER") {
        return wm;
    }

    "unknown".to_string()
}