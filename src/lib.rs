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