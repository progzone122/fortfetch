use std::{env, fs};
use std::process::Command;

pub struct Uptime {
    days: u64,
    hours: u64,
    minutes: u64,
}

impl Uptime {
    pub fn new() -> Option<Uptime> {

        let uptime_secs = Self::uptime()?;

        Some(Uptime {
            days: uptime_secs / 86400,
            hours: (uptime_secs % 86400) / 3600,
            minutes: (uptime_secs % 3600) / 60,
        })
    }

    fn uptime() -> Option<u64> {
        let contents = fs::read_to_string("/proc/uptime").ok()?;
        // первые числа секунды uptime
        let first_part = contents.split_whitespace().next()?;
        let seconds_float: f64 = first_part.parse().ok()?;
        Some(seconds_float as u64)
    }

    pub fn get(&self) -> String {
        format!("{}д. {}ч. {}м.", self.days, self.hours, self.minutes)
    }
}

pub fn get_package_count() -> Option<usize> {
    // Debian/Ubuntu (dpkg)
    if let Ok(contents) = fs::read_to_string("/var/lib/dpkg/status") {
        let count = contents.lines().filter(|line| line.starts_with("Package:")).count();
        return Some(count);
    }

    // RPM-based (Fedora, RHEL, etc.)
    if fs::metadata("/var/lib/rpm").is_ok() {
        let count = fs::read_dir("/var/lib/rpm")
            .ok()?
            .filter_map(Result::ok)
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map_or(false, |ext| ext == "rpm")
            })
            .count();
        return Some(count);
    }

    // Arch Linux (pacman)
    if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
        let count = entries
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_dir())
            .count();
        return Some(count);
    }

    None
}

pub fn get_cpu_model() -> Option<String> {
    let contents = fs::read_to_string("/proc/cpuinfo").ok()?;

    for line in contents.lines() {
        if line.starts_with("model name") {
            return line.split(':').nth(1).map(|s| s.trim().to_string());
        }
    }

    None
}

pub fn get_gpu_model() -> Option<String> {
    if Command::new("which")
        .arg("lspci")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        if let Ok(output) = Command::new("lspci").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            if let Some(line) = stdout
                .lines()
                .find(|line| line.to_lowercase().contains("vga") || line.to_lowercase().contains("3d") || line.to_lowercase().contains("2d"))
            {
                if let Some(info) = line.splitn(3, ':').nth(2) {
                    return Some(info.trim().to_string());
                }
            }
        }
    }
    None
}

pub fn get_memory_info() -> Option<String> {
    let mem = sys_info::mem_info().ok()?;

    let used_kb = mem.total.saturating_sub(mem.avail);
    let total_gb = mem.total as f64 / 1024.0 / 1024.0;
    let used_gb = used_kb as f64 / 1024.0 / 1024.0;
    let percentage = (used_kb * 100) / mem.total;

    Some(format!("{:.2}GB / {:.2}GB ({}%)", used_gb, total_gb, percentage))
}

pub fn get_disk_info() -> Option<String> {
    let disk = sys_info::disk_info().ok()?;

    let used_mb = disk.total.saturating_sub(disk.free);
    let total_gb = disk.total as f64 / 1024.0;
    let used_gb = used_mb as f64 / 1024.0;
    let percentage = (used_mb * 100) / disk.total;

    Some(format!("{:.2}GB / {:.2}GB ({}%)", used_gb, total_gb, percentage))
}

pub fn get_desktop_environment() -> Option<String> {
    let keys = ["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION", "GDMSESSION"];

    for key in keys.iter() {
        if let Ok(val) = env::var(key) {
            if !val.is_empty() {
                return Some(val);
            }
        }
    }

    None
}

pub fn get_shell() -> String {
    std::env::var("SHELL")
        .unwrap_or_else(|_| "?".to_string())
        .split('/')
        .last()
        .unwrap_or("?")
        .to_string()
}

pub fn get_terminal() -> String {
    std::env::var("TERM")
        .unwrap_or_else(|_| "?".to_string())
}

pub fn get_resolution() -> String {
    if let Ok(output) = Command::new("xrandr").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.contains("*") && line.contains("x") {
                if let Some(res) = line.split_whitespace().find(|s| s.contains("x")) {
                    return res.to_string();
                }
            }
        }
    }

    if let Ok(output) = Command::new("xdpyinfo").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.trim().starts_with("dimensions:") {
                if let Some(dims) = line.split(':').nth(1) {
                    if let Some(res) = dims.trim().split_whitespace().next() {
                        return res.to_string();
                    }
                }
            }
        }
    }

    if env::var("WAYLAND_DISPLAY").is_ok() {
        if let Ok(output) = Command::new("wlr-randr").output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if line.contains("current") && line.contains("x") {
                    if let Some(res) = line.split_whitespace().find(|s| s.contains("x")) {
                        return res.to_string();
                    }
                }
            }
        }
    }

    "?".to_string()
}

pub fn get_load_average() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = contents.split_whitespace().collect();
        if parts.len() >= 3 {
            return format!("{} {} {}", parts[0], parts[1], parts[2]);
        }
    }
    "?".to_string()
}

pub fn get_users_count() -> String {
    if let Ok(output) = Command::new("who").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let count = stdout.lines().count();
        return count.to_string();
    }
    "?".to_string()
}

pub fn get_battery_info() -> String {
    if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.file_name().unwrap_or_default().to_string_lossy().starts_with("BAT") {
                if let Ok(capacity) = fs::read_to_string(path.join("capacity")) {
                    if let Ok(status) = fs::read_to_string(path.join("status")) {
                        return format!("{}% [{}]", capacity.trim(), status.trim());
                    }
                }
            }
        }
    }
    "Подключен к сети".to_string()
}

pub fn get_temperature() -> String {
    if let Ok(temp_str) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if let Ok(temp) = temp_str.trim().parse::<i32>() {
            return format!("{}°C", temp / 1000);
        }
    }
    "?".to_string()
}

pub fn get_processes_count() -> String {
    if let Ok(entries) = fs::read_dir("/proc") {
        let count = entries
            .flatten()
            .filter(|entry| {
                entry.file_name().to_string_lossy().chars().all(|c| c.is_ascii_digit())
            })
            .count();
        return count.to_string();
    }
    "?".to_string()
}

pub fn get_cpu_usage() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/stat") {
        if let Some(cpu_line) = contents.lines().next() {
            let parts: Vec<&str> = cpu_line.split_whitespace().collect();
            if parts.len() >= 8 {
                let idle: u64 = parts[4].parse().unwrap_or(0);
                let total: u64 = parts[1..8].iter()
                    .map(|s| s.parse::<u64>().unwrap_or(0))
                    .sum();
                if total > 0 {
                    let usage = 100 - (idle * 100 / total);
                    return format!("{}%", usage);
                }
            }
        }
    }
    "?".to_string()
}

pub fn get_network_info() -> String {
    if let Ok(output) = Command::new("ip").args(&["route", "show", "default"]).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            if let Some(interface) = line.split_whitespace().nth(4) {
                if let Ok(ip_output) = Command::new("ip").args(&["addr", "show", interface]).output() {
                    let ip_stdout = String::from_utf8_lossy(&ip_output.stdout);
                    for ip_line in ip_stdout.lines() {
                        if ip_line.contains("inet ") && !ip_line.contains("127.0.0.1") {
                            if let Some(ip) = ip_line.split_whitespace().nth(1) {
                                if let Some(clean_ip) = ip.split('/').next() {
                                    return format!("{} ({})", interface, clean_ip);
                                }
                            }
                        }
                    }
                }
                return format!("{}", interface);
            }
        }
    }
    "Нет соединения".to_string()
}

pub fn get_locale_info() -> String {
    env::var("LANG").unwrap_or_else(|_| "?".to_string())
}