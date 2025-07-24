use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;

use crate::config::Config;
use crate::error::RFetchError;
use crate::utils::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub user: String,
    pub hostname: String,
    pub os: String,
    pub kernel: String,
    pub uptime: String,
    pub packages: u64,
    pub shell: String,
    pub resolution: String,
    pub desktop_environment: String,
    pub window_manager: String,
    pub theme: String,
    pub icons: String,
    pub font: String,
    pub cursor: String,
    pub terminal: String,
    pub cpu: String,
    pub gpu: String,
    pub memory: MemoryInfo,
    pub disk: Vec<DiskInfo>,
    pub battery: Option<BatteryInfo>,
    pub locale: String,
    pub local_ip: String,
    pub public_ip: String,
    pub users: Vec<String>,
    pub date: String,
    pub colors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub device: String,
    pub mount_point: String,
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub percentage: f64,
    pub filesystem: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub status: String,
    pub time_remaining: Option<String>,
}

impl SystemInfo {
    pub fn gather(config: &Config) -> Result<Self, RFetchError> {
        let mut info = SystemInfo {
            user: get_username(),
            hostname: get_hostname(),
            os: String::new(),
            kernel: String::new(),
            uptime: String::new(),
            packages: 0,
            shell: String::new(),
            resolution: String::new(),
            desktop_environment: String::new(),
            window_manager: String::new(),
            theme: String::new(),
            icons: String::new(),
            font: String::new(),
            cursor: String::new(),
            terminal: String::new(),
            cpu: String::new(),
            gpu: String::new(),
            memory: MemoryInfo {
                total: 0,
                used: 0,
                available: 0,
                percentage: 0.0,
            },
            disk: Vec::new(),
            battery: None,
            locale: String::new(),
            local_ip: String::new(),
            public_ip: String::new(),
            users: Vec::new(),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            colors: vec!["■".repeat(8)],
        };

        // Gather information based on config
        if config.info.show_os {
            info.os = Self::get_os_info()?;
        }

        if config.info.show_kernel {
            info.kernel = Self::get_kernel_info()?;
        }

        if config.info.show_uptime {
            info.uptime = Self::get_uptime()?;
        }

        if config.info.show_packages {
            info.packages = count_packages();
        }

        if config.info.show_shell {
            info.shell = get_shell();
        }

        if config.info.show_resolution {
            info.resolution = Self::get_resolution()?;
        }

        if config.info.show_de {
            info.desktop_environment = get_desktop_environment();
        }

        if config.info.show_wm {
            info.window_manager = get_window_manager();
        }

        if config.info.show_terminal {
            info.terminal = get_terminal();
        }

        if config.info.show_cpu {
            info.cpu = Self::get_cpu_info()?;
        }

        if config.info.show_gpu {
            info.gpu = Self::get_gpu_info()?;
        }

        if config.info.show_memory {
            info.memory = Self::get_memory_info()?;
        }

        if config.info.show_disk {
            info.disk = Self::get_disk_info()?;
        }

        if config.info.show_battery {
            info.battery = Self::get_battery_info().ok();
        }

        if config.info.show_locale {
            info.locale = Self::get_locale();
        }

        if config.info.show_local_ip {
            info.local_ip = Self::get_local_ip().unwrap_or_else(|_| "unknown".to_string());
        }

        if config.info.show_users {
            info.users = Self::get_logged_users();
        }

        // Generate color bar
        info.colors = Self::generate_color_bar();

        Ok(info)
    }

    #[cfg(target_os = "linux")]
    fn get_os_info() -> Result<String, RFetchError> {
        // Check if running in Termux (Android)
        if Self::is_termux() {
            if let Ok(version) = env::var("TERMUX_VERSION") {
                return Ok(format!("Termux {}", version));
            }
            return Ok("Termux".to_string());
        }

        // Check for Arch Linux specifically
        if let Ok(_) = fs::read_to_string("/etc/arch-release") {
            // Try to get version from /etc/os-release
            if let Ok(content) = fs::read_to_string("/etc/os-release") {
                for line in content.lines() {
                    if line.starts_with("PRETTY_NAME=") {
                        let name = line.split('=').nth(1).unwrap_or("")
                            .trim_matches('"');
                        return Ok(name.to_string());
                    }
                }
            }
            return Ok("Arch Linux".to_string());
        }

        // Check for Ubuntu specifically
        if let Ok(content) = fs::read_to_string("/etc/lsb-release") {
            let mut distrib_id = String::new();
            let mut distrib_release = String::new();
            
            for line in content.lines() {
                if line.starts_with("DISTRIB_ID=") {
                    distrib_id = line.split('=').nth(1).unwrap_or("").to_string();
                } else if line.starts_with("DISTRIB_RELEASE=") {
                    distrib_release = line.split('=').nth(1).unwrap_or("").to_string();
                }
            }
            
            if !distrib_id.is_empty() && !distrib_release.is_empty() {
                return Ok(format!("{} {}", distrib_id, distrib_release));
            }
        }

        // Try /etc/os-release first
        if let Ok(content) = fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    let name = line.split('=').nth(1).unwrap_or("")
                        .trim_matches('"');
                    return Ok(name.to_string());
                }
            }
        }

        // Fallback to /etc/issue
        if let Ok(content) = fs::read_to_string("/etc/issue") {
            let first_line = content.lines().next().unwrap_or("Linux");
            return Ok(first_line.replace("\\n", "").replace("\\l", "").trim().to_string());
        }

        Ok("Linux".to_string())
    }

    #[cfg(target_os = "macos")]
    fn get_os_info() -> Result<String, RFetchError> {
        if let Ok(output) = Command::new("sw_vers").arg("-productName").output() {
            let product = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(version_output) = Command::new("sw_vers").arg("-productVersion").output() {
                let version = String::from_utf8_lossy(&version_output.stdout).trim().to_string();
                return Ok(format!("{} {}", product, version));
            }
            return Ok(product);
        }
        Ok("macOS".to_string())
    }

    #[cfg(target_os = "ios")]
    fn get_os_info() -> Result<String, RFetchError> {
        // iOS detection - check for iOS-specific environment
        if let Ok(version) = env::var("IPHONEOS_DEPLOYMENT_TARGET") {
            return Ok(format!("iOS {}", version));
        }
        
        // Try to get iOS version from system
        if let Ok(output) = Command::new("sw_vers").arg("-productVersion").output() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return Ok(format!("iOS {}", version));
        }
        
        // Check if running in iSH (iOS shell)
        if let Ok(_) = fs::read_to_string("/proc/ish") {
            return Ok("iOS (iSH)".to_string());
        }
        
        Ok("iOS".to_string())
    }

    #[cfg(target_os = "windows")]
    fn get_os_info() -> Result<String, RFetchError> {
        if let Ok(output) = Command::new("wmic")
            .args(&["os", "get", "Caption", "/value"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("Caption=") {
                    return Ok(line.replace("Caption=", "").trim().to_string());
                }
            }
        }
        Ok("Windows".to_string())
    }

    #[cfg(unix)]
    fn get_kernel_info() -> Result<String, RFetchError> {
        #[cfg(target_os = "ios")]
        {
            // iOS kernel detection
            if Self::is_ios() {
                if let Ok(output) = Command::new("uname").arg("-r").output() {
                    let kernel = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    return Ok(format!("Darwin {}", kernel));
                } else {
                    return Ok("Darwin (iOS)".to_string());
                }
            }
        }

        if let Ok(output) = Command::new("uname").arg("-r").output() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    #[cfg(windows)]
    fn get_kernel_info() -> Result<String, RFetchError> {
        if let Ok(output) = Command::new("wmic")
            .args(&["os", "get", "Version", "/value"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            for line in output_str.lines() {
                if line.starts_with("Version=") {
                    return Ok(line.replace("Version=", "").trim().to_string());
                }
            }
        }
        Ok("unknown".to_string())
    }

    #[cfg(target_os = "linux")]
    fn get_uptime() -> Result<String, RFetchError> {
        if let Ok(content) = fs::read_to_string("/proc/uptime") {
            let uptime_seconds = content
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<f64>().ok())
                .unwrap_or(0.0) as u64;
            Ok(format_uptime(uptime_seconds))
        } else {
            Ok("unknown".to_string())
        }
    }

    #[cfg(target_os = "macos")]
    fn get_uptime() -> Result<String, RFetchError> {
        if let Ok(output) = Command::new("uptime").output() {
            let uptime_str = String::from_utf8_lossy(&output.stdout);
            // Parse uptime output to extract just the uptime part
            // Example: "19:08  up 7 days,  7:12, 2 users, load averages: 11.23 31.89 30.19"
            if let Some(up_index) = uptime_str.find("up ") {
                let after_up = &uptime_str[up_index + 3..];
                if let Some(comma_index) = after_up.find(',') {
                    let mut uptime_part = after_up[..comma_index].trim();
                    // Check if there's another comma (for users part)
                    if let Some(remaining) = after_up.get(comma_index + 1..) {
                        if let Some(second_comma) = remaining.find(',') {
                            uptime_part = &after_up[..comma_index + 1 + second_comma];
                        }
                    }
                    return Ok(uptime_part.trim_end_matches(',').trim().to_string());
                } else {
                    // No comma found, take everything after "up "
                    let end_index = after_up.find(" user").unwrap_or(after_up.len());
                    return Ok(after_up[..end_index].trim().to_string());
                }
            }
            Ok("unknown".to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    #[cfg(target_os = "ios")]
    fn get_uptime() -> Result<String, RFetchError> {
        // iOS uptime detection
        if Self::is_ios() {
            // Try uptime command first (available in iSH)
            if let Ok(output) = Command::new("uptime").output() {
                let uptime_str = String::from_utf8_lossy(&output.stdout);
                if let Some(up_index) = uptime_str.find("up ") {
                    let after_up = &uptime_str[up_index + 3..];
                    if let Some(comma_index) = after_up.find(',') {
                        return Ok(after_up[..comma_index].trim().to_string());
                    }
                }
            }
            
            // Fallback: try to read from /proc/uptime if available (iSH)
            if let Ok(content) = fs::read_to_string("/proc/uptime") {
                let uptime_seconds = content
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap_or(0.0) as u64;
                return Ok(format_uptime(uptime_seconds));
            }
        }
        
        Ok("unknown".to_string())
    }

    #[cfg(target_os = "windows")]
    fn get_uptime() -> Result<String, RFetchError> {
        if let Ok(output) = Command::new("wmic")
            .args(&["os", "get", "LastBootUpTime", "/value"])
            .output()
        {
            // Parse Windows uptime - simplified
            Ok("unknown".to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    fn get_resolution() -> Result<String, RFetchError> {
        #[cfg(unix)]
        {
            if let Ok(output) = Command::new("xrandr").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("*") && line.contains("x") {
                        if let Some(resolution) = line.split_whitespace().next() {
                            return Ok(resolution.to_string());
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = Command::new("system_profiler")
                .arg("SPDisplaysDataType")
                .output()
            {
                let _output_str = String::from_utf8_lossy(&output.stdout);
                // Parse macOS display info - simplified
                return Ok("unknown".to_string());
            }
        }

        Ok("unknown".to_string())
    }

    fn get_cpu_info() -> Result<String, RFetchError> {
        #[cfg(target_os = "linux")]
        {
            // Special handling for Termux
            if Self::is_termux() {
                // Try getprop for Android device info
                if let Ok(output) = Command::new("getprop").arg("ro.product.cpu.abi").output() {
                    let cpu_abi = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !cpu_abi.is_empty() {
                        // Try to get more detailed CPU info
                        if let Ok(model_output) = Command::new("getprop").arg("ro.product.model").output() {
                            let model = String::from_utf8_lossy(&model_output.stdout).trim().to_string();
                            if !model.is_empty() {
                                return Ok(format!("{} ({})", model, cpu_abi));
                            }
                        }
                        return Ok(cpu_abi);
                    }
                }
                
                // Fallback to lscpu if available
                if let Ok(output) = Command::new("lscpu").output() {
                    let output_str = String::from_utf8_lossy(&output.stdout);
                    for line in output_str.lines() {
                        if line.starts_with("Model name:") {
                            if let Some(name) = line.split(':').nth(1) {
                                return Ok(name.trim().to_string());
                            }
                        }
                    }
                }
                
                return Ok("ARM".to_string());
            }

            if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
                for line in content.lines() {
                    if line.starts_with("model name") {
                        if let Some(name) = line.split(':').nth(1) {
                            return Ok(name.trim().to_string());
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = Command::new("sysctl")
                .args(&["-n", "machdep.cpu.brand_string"])
                .output()
            {
                return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = Command::new("wmic")
                .args(&["cpu", "get", "name", "/value"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.starts_with("Name=") {
                        return Ok(line.replace("Name=", "").trim().to_string());
                    }
                }
            }
        }

        Ok("unknown".to_string())
    }

    // Helper function to detect if running in Termux
    fn is_termux() -> bool {
        // Check for Termux-specific environment variables
        env::var("TERMUX_VERSION").is_ok() ||
        env::var("PREFIX").map(|p| p.contains("com.termux")).unwrap_or(false) ||
        std::path::Path::new("/data/data/com.termux").exists()
    }

    // Helper function to detect if running on iOS
    fn is_ios() -> bool {
        // Check for iOS-specific environment variables and paths
        env::var("IPHONEOS_DEPLOYMENT_TARGET").is_ok() ||
        std::path::Path::new("/proc/ish").exists() ||
        env::var("SIMULATOR_DEVICE_NAME").is_ok() ||
        std::path::Path::new("/Applications").exists() && std::path::Path::new("/System/Library/CoreServices/SpringBoard.app").exists()
    }

    fn get_gpu_info() -> Result<String, RFetchError> {
        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = Command::new("system_profiler").arg("SPDisplaysDataType").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.trim().starts_with("Chipset Model:") {
                        if let Some(gpu) = line.split(':').nth(1) {
                            let gpu_name = gpu.trim().to_string();
                            // Look for additional info like cores
                            for next_line in output_str.lines() {
                                if next_line.trim().starts_with("Total Number of Cores:") {
                                    if let Some(cores) = next_line.split(':').nth(1) {
                                        return Ok(format!("{} ({} cores)", gpu_name, cores.trim()));
                                    }
                                }
                            }
                            return Ok(gpu_name);
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            // Special handling for Termux
            if Self::is_termux() {
                // Try to get GPU info from Android properties
                if let Ok(output) = Command::new("getprop").arg("ro.hardware.vulkan").output() {
                    let vulkan = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !vulkan.is_empty() && vulkan != "0" {
                        return Ok(format!("Vulkan: {}", vulkan));
                    }
                }
                
                if let Ok(output) = Command::new("getprop").arg("ro.hardware.egl").output() {
                    let egl = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !egl.is_empty() {
                        return Ok(format!("EGL: {}", egl));
                    }
                }
                
                // Try to get GPU renderer info
                if let Ok(output) = Command::new("getprop").arg("debug.egl.hw").output() {
                    let hw = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if !hw.is_empty() && hw != "0" {
                        return Ok("Hardware Accelerated".to_string());
                    }
                }
                
                return Ok("Integrated".to_string());
            }

            if let Ok(output) = Command::new("lspci").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.contains("VGA") || line.contains("3D") {
                        if let Some(gpu) = line.split(':').nth(2) {
                            return Ok(gpu.trim().to_string());
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = Command::new("wmic")
                .args(&["path", "win32_VideoController", "get", "name", "/value"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.starts_with("Name=") && !line.trim_end().ends_with("=") {
                        return Ok(line.replace("Name=", "").trim().to_string());
                    }
                }
            }
        }

        #[cfg(target_os = "ios")]
        {
            // For iOS, try to get GPU info from system_profiler if available (iSH environment)
            if let Ok(output) = Command::new("system_profiler").arg("SPDisplaysDataType").output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines() {
                    if line.trim().starts_with("Chipset Model:") {
                        if let Some(gpu) = line.split(':').nth(1) {
                            return Ok(gpu.trim().to_string());
                        }
                    }
                }
            }
            
            // Fallback: try to detect if we're on an Apple Silicon device
            if let Ok(output) = Command::new("uname").arg("-m").output() {
                let arch = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if arch.contains("arm64") || arch.contains("aarch64") {
                    return Ok("Apple GPU".to_string());
                }
            }
            
            // Final fallback for iOS
            return Ok("Integrated GPU".to_string());
        }

        Ok("unknown".to_string())
    }

    #[cfg(target_os = "linux")]
    fn get_memory_info() -> Result<MemoryInfo, RFetchError> {
        if let Ok(content) = fs::read_to_string("/proc/meminfo") {
            let mut total = 0u64;
            let mut available = 0u64;

            for line in content.lines() {
                if line.starts_with("MemTotal:") {
                    total = line.split_whitespace()
                        .nth(1)
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0) * 1024; // Convert from KB to bytes
                } else if line.starts_with("MemAvailable:") {
                    available = line.split_whitespace()
                        .nth(1)
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(0) * 1024; // Convert from KB to bytes
                }
            }

            let used = total - available;
            let percentage = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };

            return Ok(MemoryInfo {
                total,
                used,
                available,
                percentage,
            });
        }

        Err(RFetchError::system_info("Could not read memory information"))
    }

    #[cfg(target_os = "macos")]
    fn get_memory_info() -> Result<MemoryInfo, RFetchError> {
        if let Ok(output) = Command::new("vm_stat").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            let mut page_size = 4096u64; // Default page size
            let mut pages_free = 0u64;
            let mut pages_active = 0u64;
            let mut pages_inactive = 0u64;
            let mut pages_speculative = 0u64;
            let mut pages_wired = 0u64;
            let mut pages_compressed = 0u64;
            
            for line in output_str.lines() {
                if line.contains("page size of") {
                    if let Some(start) = line.find("page size of ") {
                        let size_part = &line[start + 13..];
                        if let Some(end) = size_part.find(" bytes") {
                            if let Ok(size) = size_part[..end].parse::<u64>() {
                                page_size = size;
                            }
                        }
                    }
                } else if line.starts_with("Pages free:") {
                    pages_free = line.split_whitespace().nth(2)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("Pages active:") {
                    pages_active = line.split_whitespace().nth(2)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("Pages inactive:") {
                    pages_inactive = line.split_whitespace().nth(2)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("Pages speculative:") {
                    pages_speculative = line.split_whitespace().nth(2)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("Pages wired down:") {
                    pages_wired = line.split_whitespace().nth(3)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("Pages occupied by compressor:") {
                    pages_compressed = line.split_whitespace().nth(4)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                }
            }
            
            // Calculate memory usage
            let total_pages = pages_free + pages_active + pages_inactive + pages_speculative + pages_wired + pages_compressed;
            let used_pages = pages_active + pages_inactive + pages_speculative + pages_wired + pages_compressed;
            let available_pages = pages_free + pages_inactive + pages_speculative;
            
            let total = total_pages * page_size;
            let used = used_pages * page_size;
            let available = available_pages * page_size;
            
            let percentage = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            
            return Ok(MemoryInfo {
                total,
                used,
                available,
                percentage,
            });
        }
        
        Err(RFetchError::system_info("Could not read memory information"))
    }

    #[cfg(target_os = "windows")]
    fn get_memory_info() -> Result<MemoryInfo, RFetchError> {
        // Simplified Windows memory info
        Ok(MemoryInfo {
            total: 0,
            used: 0,
            available: 0,
            percentage: 0.0,
        })
    }

    #[cfg(target_os = "ios")]
    fn get_memory_info() -> Result<MemoryInfo, RFetchError> {
        // For iOS, try to use vm_stat if available (iSH environment)
        if let Ok(output) = Command::new("vm_stat").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            
            let mut page_size = 4096u64; // Default page size
            let mut pages_free = 0u64;
            let mut pages_active = 0u64;
            let mut pages_inactive = 0u64;
            let mut pages_wired = 0u64;
            
            for line in output_str.lines() {
                if line.contains("page size of") {
                    if let Some(start) = line.find("page size of ") {
                        let size_part = &line[start + 13..];
                        if let Some(end) = size_part.find(" bytes") {
                            if let Ok(size) = size_part[..end].parse::<u64>() {
                                page_size = size;
                            }
                        }
                    }
                } else if line.starts_with("Pages free:") {
                    pages_free = line.split_whitespace().nth(2)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("Pages active:") {
                    pages_active = line.split_whitespace().nth(2)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("Pages inactive:") {
                    pages_inactive = line.split_whitespace().nth(2)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                } else if line.starts_with("Pages wired down:") {
                    pages_wired = line.split_whitespace().nth(3)
                        .and_then(|s| s.trim_end_matches('.').parse().ok())
                        .unwrap_or(0);
                }
            }
            
            let total_pages = pages_free + pages_active + pages_inactive + pages_wired;
            let used_pages = pages_active + pages_inactive + pages_wired;
            let available_pages = pages_free + pages_inactive;
            
            let total = total_pages * page_size;
            let used = used_pages * page_size;
            let available = available_pages * page_size;
            
            let percentage = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            
            return Ok(MemoryInfo {
                total,
                used,
                available,
                percentage,
            });
        }
        
        // Fallback for iOS
        Ok(MemoryInfo {
            total: 0,
            used: 0,
            available: 0,
            percentage: 0.0,
        })
    }

    fn get_disk_info() -> Result<Vec<DiskInfo>, RFetchError> {
        let mut disks = Vec::new();

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = Command::new("df").args(&["-h"]).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 9 && !parts[0].starts_with("map") && !parts[0].starts_with("devfs") {
                        // Focus on main disk (usually /)
                        if parts[8] == "/" {
                            disks.push(DiskInfo {
                                device: parts[0].to_string(),
                                mount_point: parts[8].to_string(),
                                total: Self::parse_size_string(parts[1]),
                                used: Self::parse_size_string(parts[2]),
                                available: Self::parse_size_string(parts[3]),
                                percentage: parts[4].trim_end_matches('%').parse().unwrap_or(0.0),
                                filesystem: "APFS".to_string(),
                            });
                            break; // Only show main disk
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(output) = Command::new("df").args(&["-h", "--output=source,target,size,used,avail,pcent,fstype"]).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 7 && !parts[0].starts_with("tmpfs") && !parts[0].starts_with("devtmpfs") {
                        // Focus on main disk (usually /)
                        if parts[1] == "/" {
                            disks.push(DiskInfo {
                                device: parts[0].to_string(),
                                mount_point: parts[1].to_string(),
                                total: Self::parse_size_string(parts[2]),
                                used: Self::parse_size_string(parts[3]),
                                available: Self::parse_size_string(parts[4]),
                                percentage: parts[5].trim_end_matches('%').parse().unwrap_or(0.0),
                                filesystem: parts.get(6).unwrap_or(&"unknown").to_string(),
                            });
                            break; // Only show main disk
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Ok(output) = Command::new("wmic")
                .args(&["logicaldisk", "get", "size,freespace,caption", "/value"])
                .output()
            {
                let output_str = String::from_utf8_lossy(&output.stdout);
                let mut caption = String::new();
                let mut size = 0u64;
                let mut free_space = 0u64;
                
                for line in output_str.lines() {
                    if line.starts_with("Caption=") {
                        caption = line.replace("Caption=", "").trim().to_string();
                    } else if line.starts_with("FreeSpace=") {
                        free_space = line.replace("FreeSpace=", "").trim().parse().unwrap_or(0);
                    } else if line.starts_with("Size=") {
                        size = line.replace("Size=", "").trim().parse().unwrap_or(0);
                        
                        if !caption.is_empty() && size > 0 && caption == "C:" {
                            let used = size - free_space;
                            let percentage = if size > 0 {
                                (used as f64 / size as f64) * 100.0
                            } else {
                                0.0
                            };
                            
                            disks.push(DiskInfo {
                                device: caption.clone(),
                                mount_point: caption.clone(),
                                total: size,
                                used,
                                available: free_space,
                                percentage,
                                filesystem: "NTFS".to_string(),
                            });
                            break; // Only show C: drive
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "ios")]
        {
            // For iOS, try to use df command if available (iSH environment)
            if let Ok(output) = Command::new("df").args(&["-h"]).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                for line in output_str.lines().skip(1) {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 9 && !parts[0].starts_with("devfs") && !parts[0].starts_with("map") {
                        // Focus on main disk (usually /)
                        if parts[8] == "/" {
                            disks.push(DiskInfo {
                                device: parts[0].to_string(),
                                mount_point: parts[8].to_string(),
                                total: Self::parse_size_string(parts[1]),
                                used: Self::parse_size_string(parts[2]),
                                available: Self::parse_size_string(parts[3]),
                                percentage: parts[4].trim_end_matches('%').parse().unwrap_or(0.0),
                                filesystem: "APFS".to_string(),
                            });
                            break; // Only show main disk
                        }
                    }
                }
            } else {
                // Fallback for iOS when df is not available
                disks.push(DiskInfo {
                    device: "iOS Storage".to_string(),
                    mount_point: "/".to_string(),
                    total: 0,
                    used: 0,
                    available: 0,
                    percentage: 0.0,
                    filesystem: "APFS".to_string(),
                });
            }
        }

        Ok(disks)
    }

    // Helper function to parse size strings like "228Gi", "10Gi", etc.
    fn parse_size_string(size_str: &str) -> u64 {
        let size_str = size_str.trim();
        if size_str.is_empty() {
            return 0;
        }
        
        let (number_part, unit) = if size_str.ends_with("Ti") {
            (size_str.trim_end_matches("Ti"), 1024u64.pow(4))
        } else if size_str.ends_with("Gi") {
            (size_str.trim_end_matches("Gi"), 1024u64.pow(3))
        } else if size_str.ends_with("Mi") {
            (size_str.trim_end_matches("Mi"), 1024u64.pow(2))
        } else if size_str.ends_with("Ki") {
            (size_str.trim_end_matches("Ki"), 1024u64)
        } else if size_str.ends_with("T") {
            (size_str.trim_end_matches("T"), 1000u64.pow(4))
        } else if size_str.ends_with("G") {
            (size_str.trim_end_matches("G"), 1000u64.pow(3))
        } else if size_str.ends_with("M") {
            (size_str.trim_end_matches("M"), 1000u64.pow(2))
        } else if size_str.ends_with("K") {
            (size_str.trim_end_matches("K"), 1000u64)
        } else {
            (size_str, 1u64)
        };
        
        if let Ok(number) = number_part.parse::<f64>() {
            (number * unit as f64) as u64
        } else {
            0
        }
    }

    fn get_battery_info() -> Result<BatteryInfo, RFetchError> {
        #[cfg(target_os = "linux")]
        {
            let battery_path = "/sys/class/power_supply/BAT0";
            if let Ok(capacity) = fs::read_to_string(format!("{}/capacity", battery_path)) {
                let percentage = capacity.trim().parse().unwrap_or(0);
                let status = fs::read_to_string(format!("{}/status", battery_path))
                    .unwrap_or_else(|_| "Unknown".to_string())
                    .trim()
                    .to_string();

                return Ok(BatteryInfo {
                    percentage,
                    status,
                    time_remaining: None,
                });
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(output) = Command::new("pmset").args(&["-g", "batt"]).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                // Parse macOS battery info
                // Example output: "Now drawing from 'AC Power'\n -InternalBattery-0 (id=1234567)	100%; charged; 0:00 remaining present: true"
                
                let is_ac_power = output_str.contains("AC Power");
                
                for line in output_str.lines() {
                    if line.contains("InternalBattery") {
                        // Look for percentage
                        if let Some(percent_start) = line.find('\t') {
                            let after_tab = &line[percent_start + 1..];
                            if let Some(percent_end) = after_tab.find('%') {
                                if let Ok(percentage) = after_tab[..percent_end].trim().parse::<u8>() {
                                    // Look for status - be more specific with the parsing
                                    let status = if after_tab.contains("charging") {
                                        "Charging"
                                    } else if after_tab.contains("discharging") {
                                        "Discharging"
                                    } else if after_tab.contains("charged") {
                                        if is_ac_power {
                                            "Charged (AC)"
                                        } else {
                                            "Charged"
                                        }
                                    } else if is_ac_power {
                                        "AC Power"
                                    } else {
                                        "Unknown"
                                    };

                                    // Look for time remaining
                                    let time_remaining = if let Some(time_start) = after_tab.find(';') {
                                        let time_part = &after_tab[time_start + 1..];
                                        if let Some(remaining_pos) = time_part.find("remaining") {
                                            let time_str = time_part[..remaining_pos].trim();
                                            if time_str != "0:00" && !time_str.is_empty() && !time_str.contains(';') {
                                                Some(time_str.to_string())
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    };

                                    return Ok(BatteryInfo {
                                        percentage,
                                        status: status.to_string(),
                                        time_remaining,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        #[cfg(target_os = "ios")]
        {
            // For iOS, try to use pmset if available (iSH environment)
            if let Ok(output) = Command::new("pmset").args(&["-g", "batt"]).output() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                
                for line in output_str.lines() {
                    if line.contains("InternalBattery") || line.contains("Battery") {
                        // Look for percentage
                        if let Some(percent_start) = line.find('\t') {
                            let after_tab = &line[percent_start + 1..];
                            if let Some(percent_end) = after_tab.find('%') {
                                if let Ok(percentage) = after_tab[..percent_end].trim().parse::<u8>() {
                                    let status = if after_tab.contains("charging") {
                                        "Charging"
                                    } else if after_tab.contains("discharging") {
                                        "Discharging"
                                    } else if after_tab.contains("charged") {
                                        "Charged"
                                    } else {
                                        "Unknown"
                                    };

                                    return Ok(BatteryInfo {
                                        percentage,
                                        status: status.to_string(),
                                        time_remaining: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
            
            // Fallback for iOS - assume device has battery
            return Ok(BatteryInfo {
                percentage: 100,
                status: "Unknown".to_string(),
                time_remaining: None,
            });
        }

        Err(RFetchError::system_info("No battery found"))
    }

    fn get_locale() -> String {
        env::var("LANG").unwrap_or_else(|_| "unknown".to_string())
    }

    fn get_local_ip() -> Result<String, RFetchError> {
        #[cfg(unix)]
        {
            if let Ok(output) = Command::new("hostname").arg("-I").output() {
                let ip = String::from_utf8_lossy(&output.stdout)
                    .split_whitespace()
                    .next()
                    .unwrap_or("unknown")
                    .to_string();
                return Ok(ip);
            }
        }

        Ok("unknown".to_string())
    }

    fn get_logged_users() -> Vec<String> {
        if let Ok(output) = Command::new("who").output() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            return output_str
                .lines()
                .filter_map(|line| line.split_whitespace().next())
                .map(|s| s.to_string())
                .collect();
        }
        Vec::new()
    }

    fn generate_color_bar() -> Vec<String> {
        let colors = ["■"; 8];
        colors.iter().map(|&s| s.to_string()).collect()
    }
}