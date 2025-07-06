use crate::config::{BLACK, BLUE, CYAN, DARK_GRAY, GREEN, LIGHT_BLUE, LIGHT_GRAY, NC, PURPLE, RED, WHITE, YELLOW};
use fortfetch::{get_cpu_model, get_desktop_environment, get_disk_info, get_gpu_model, get_package_count, Uptime};
use std::process::Command;
use std::fs;
use std::env;

mod config;

fn get_shell() -> String {
    std::env::var("SHELL")
        .unwrap_or_else(|_| "?".to_string())
        .split('/')
        .last()
        .unwrap_or("?")
        .to_string()
}

fn get_terminal() -> String {
    std::env::var("TERM")
        .unwrap_or_else(|_| "?".to_string())
}

fn get_resolution() -> String {
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

fn get_load_average() -> String {
    if let Ok(contents) = fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = contents.split_whitespace().collect();
        if parts.len() >= 3 {
            return format!("{} {} {}", parts[0], parts[1], parts[2]);
        }
    }
    "?".to_string()
}

fn get_users_count() -> String {
    if let Ok(output) = Command::new("who").output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let count = stdout.lines().count();
        return count.to_string();
    }
    "?".to_string()
}

fn get_battery_info() -> String {
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

fn get_temperature() -> String {
    if let Ok(temp_str) = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        if let Ok(temp) = temp_str.trim().parse::<i32>() {
            return format!("{}°C", temp / 1000);
        }
    }
    "?".to_string()
}

fn get_processes_count() -> String {
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

fn get_cpu_usage() -> String {
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

fn get_network_info() -> String {
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

fn get_locale_info() -> String {
    env::var("LANG").unwrap_or_else(|_| "?".to_string())
}

fn get_string_length(s: &str) -> usize {
    let mut result = 0;
    let mut chars = s.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(c) = chars.next() {
                    if c.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result += 1;
        }
    }
    
    result
}

fn get_info() -> Vec<String> {
    let mut info_lines = Vec::new();

    let hostname = sys_info::hostname().unwrap_or("?".to_string());
    let username = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let ditro = sys_info::linux_os_release().unwrap().pretty_name.unwrap_or("?".to_string());
    let kernel_version = sys_info::os_release().unwrap_or("?".to_string());
    let uptime = Uptime::new().unwrap();
    let package_count = get_package_count()
        .map(|n| n.to_string())
        .unwrap_or_else(|| "?".to_string());
    let cpu = get_cpu_model().unwrap_or("?".to_string());
    let gpu = get_gpu_model().unwrap_or("?".to_string());
    let disk = get_disk_info().unwrap_or("?".to_string());
    let de = get_desktop_environment().unwrap_or("?".to_string());
    let shell = get_shell();
    let terminal = get_terminal();
    let resolution = get_resolution();
    let load_avg = get_load_average();
    let users_count = get_users_count();
    let battery = get_battery_info();
    let temperature = get_temperature();
    let processes = get_processes_count();
    let cpu_usage = get_cpu_usage();
    let network = get_network_info();
    let locale = get_locale_info();

    info_lines.push(format!("{}{}{}@{}{}{}", 
        WHITE, username, LIGHT_GRAY, LIGHT_BLUE, hostname, NC));
    
    info_lines.push(format!("{}{}{}",
        DARK_GRAY, "─".repeat(45), NC));
    
    info_lines.push(format!("{} {}Система{}", LIGHT_BLUE, WHITE, NC));
    info_lines.push(format!("{}├─ {}Абонент{}     {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, hostname, NC));
    info_lines.push(format!("{}├─ {}Тариф{}      {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, ditro, NC));
    info_lines.push(format!("{}├─ {}Прошивка{}   {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, kernel_version, NC));
    info_lines.push(format!("{}├─ {}Пополнение{} {}{}назад{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, uptime.get(), NC));
    info_lines.push(format!("{}├─ {}Вирусов{}    {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, package_count, NC));
    info_lines.push(format!("{}└─ {}Оболочка{}  {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, shell, NC));
    
    info_lines.push("".to_string());
    
    info_lines.push(format!("{} {}Железо{}", LIGHT_BLUE, WHITE, NC));
    info_lines.push(format!("{}├─ {}ЦП{}         {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, cpu, NC));
    info_lines.push(format!("{}├─ {}ГПУ{}        {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, gpu, NC));
    info_lines.push(format!("{}└─ {}Дискета{}    {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, disk, NC));
    
    info_lines.push("".to_string());
    
    info_lines.push(format!("{} {}Производительность{}", LIGHT_BLUE, WHITE, NC));
    info_lines.push(format!("{}├─ {}Загрузка ЦП{} {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, cpu_usage, NC));
    info_lines.push(format!("{}├─ {}Температура{} {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, temperature, NC));
    info_lines.push(format!("{}└─ {}Нагрузка{}   {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, load_avg, NC));
    
    info_lines.push("".to_string());
    
    info_lines.push(format!("{} {}Окружение{}", LIGHT_BLUE, WHITE, NC));
    info_lines.push(format!("{}├─ {}Админка{}    {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, de, NC));
    info_lines.push(format!("{}├─ {}Экран{}      {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, resolution, NC));
    info_lines.push(format!("{}├─ {}Терминал{}   {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, terminal, NC));
    info_lines.push(format!("{}├─ {}Локаль{}     {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, locale, NC));
    info_lines.push(format!("{}├─ {}Процессов{}  {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, processes, NC));
    info_lines.push(format!("{}├─ {}Юзеров{}     {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, users_count, NC));
    info_lines.push(format!("{}└─ {}Сеть{}       {}{}{}", 
        LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, network, NC));
    
    if !battery.contains("Подключен к сети") {
        info_lines.push("".to_string());
        info_lines.push(format!("{} {}Батарея{}     {}{}{}", 
            LIGHT_BLUE, WHITE, NC, LIGHT_GRAY, battery, NC));
    }
    
    info_lines.push("".to_string());
    
    let mut palette = format!("{}Цвета: ", LIGHT_GRAY);
    let colors = [BLACK, RED, GREEN, YELLOW, BLUE, PURPLE, CYAN, WHITE];
    for &color in &colors {
        palette.push_str(&format!("{}███{}", color, NC));
    }
    info_lines.push(palette);

    info_lines
}

fn main() {
    let logo = config::get_logo();
    let info_lines = get_info();

    let mut max_logo_width = 0;
    for line in &logo {
        let current_width = get_string_length(line);
        if current_width > max_logo_width {
            max_logo_width = current_width;
        }
    }

    println!();

    let max_lines = logo.len().max(info_lines.len());
    let gap = 4;

    for i in 0..max_lines {
        if i < logo.len() {
            let logo_line = &logo[i];
            print!("{}", logo_line);
            
            let logo_line_width = get_string_length(logo_line);
            let padding = max_logo_width - logo_line_width + gap;
            print!("{}", " ".repeat(padding));
        } else {
            print!("{}", " ".repeat(max_logo_width + gap));
        }
        
        if i < info_lines.len() {
            println!("{}", info_lines[i]);
        } else {
            println!();
        }
    }

    println!();
}
