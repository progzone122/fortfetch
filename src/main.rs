use crate::config::{BLACK, BLUE, CYAN, DARK_GRAY, GAP, GREEN, LIGHT_BLUE, LIGHT_GRAY, NC, PURPLE, RED, WHITE, YELLOW};
use fortfetch::{get_battery_info, get_cpu_model, get_cpu_usage, get_desktop_environment, get_disk_info, get_gpu_model, get_load_average, get_locale_info, get_network_info, get_package_count, get_processes_count, get_resolution, get_shell, get_temperature, get_terminal, get_users_count, Uptime};
use std::process::Command;
use std::fs;
use std::env;

mod config;

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

    for i in 0..max_lines {
        if i < logo.len() {
            let logo_line = &logo[i];
            print!("{}", logo_line);
            
            let logo_line_width = get_string_length(logo_line);
            let padding = max_logo_width - logo_line_width + GAP;
            print!("{}", " ".repeat(padding));
        } else {
            print!("{}", " ".repeat(max_logo_width + GAP));
        }
        
        if i < info_lines.len() {
            println!("{}", info_lines[i]);
        } else {
            println!();
        }
    }

    println!();
}
