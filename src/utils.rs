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
    if let Ok(term) = env::var("TERM_PROGRAM") {
        return term;
    }

    if is_termux() {
        return "Termux".to_string();
    }

    if let Ok(term) = env::var("TERM") {
        if !term.is_empty() && term != "xterm" && term != "xterm-256color" {
            return term;
        }
    }

    if let Ok(term) = env::var("TERMINAL") {
        return term;
    }

    if let Ok(term) = env::var("COLORTERM") {
        return term;
    }

    if let Ok(term) = env::var("TERM_PROGRAM_VERSION") {
        return format!("Terminal {}", term);
    }

    if let Ok(output) = Command::new("ps")
        .args(&["-o", "comm=", "-p"])
        .arg(format!("{}", std::process::id()))
        .output()
    {
        if let Ok(parent) = String::from_utf8(output.stdout) {
            let parent = parent.trim();
            if !parent.is_empty() {
                return parent.to_string();
            }
        }
    }

    "unknown".to_string()
}

pub fn is_termux() -> bool {
    env::var("PREFIX").map(|p| p.contains("com.termux")).unwrap_or(false)
}

pub fn count_packages() -> u64 {
    let mut total = 0;

    let package_managers = [
        ("pacman", &["-Q"] as &[&str]),
        ("dpkg", &["-l"]),
        ("rpm", &["-qa"]),
        ("pkg", &["list-installed"]),
        ("brew", &["list"]),
        ("flatpak", &["list"]),
        ("snap", &["list"]),
        ("apk", &["list", "-I"]),
        ("emerge", &["--list"]),
        ("zypper", &["search", "-i"]),
        ("yum", &["list", "installed"]),
        ("dnf", &["list", "installed"]),
        ("pkg_info", &[""]),
        ("pkgin", &["list"]),
        ("xbps-query", &["-l"]),
        ("nix-env", &["-q"]),
        ("guix", &["package", "--list-installed"]),
    ];

    for (manager, args) in &package_managers {
        if let Ok(output) = Command::new(manager).args(*args).output() {
            if output.status.success() {
                let count = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .count() as u64;
                
                total += count;
                
                if manager == &"pkg" && count > 0 {
                    break;
                }
            }
        }
    }

    total
}

pub fn is_container() -> bool {
    if std::path::Path::new("/.dockerenv").exists() {
        return true;
    }

    if let Ok(content) = std::fs::read_to_string("/proc/1/cgroup") {
        if content.contains("docker") || content.contains("lxc") || content.contains("containerd") {
            return true;
        }
    }

    false
}

pub fn get_desktop_environment() -> String {
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

    if let Ok(wm) = env::var("WINDOW_MANAGER") {
        return wm;
    }

    "unknown".to_string()
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
    Path::new("/.dockerenv").exists() ||
    env::var("container").is_ok() ||
    fs::read_to_string("/proc/1/cgroup")
        .map(|content| content.contains("docker") || content.contains("lxc"))
        .unwrap_or(false)
}