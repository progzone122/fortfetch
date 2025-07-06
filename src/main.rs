use crate::config::{BLACK, BLUE, CYAN, DARK_GRAY, GAP, GREEN, LIGHT_BLUE, LIGHT_GRAY, MAX_KEY_LENGTH, NC, PURPLE, RED, WHITE, YELLOW};
use fortfetch::{get_cpu_model, get_desktop_environment, get_disk_info, get_gpu_model, get_memory_info, get_package_count, Uptime};

mod config;

fn get_info() -> Vec<String> {
    let mut info_lines = Vec::new();

    let hostname = sys_info::hostname().unwrap_or("?".to_string());
    let ditro = sys_info::linux_os_release().unwrap().pretty_name.unwrap_or("?".to_string());
    let kernel_version = sys_info::os_release().unwrap_or("?".to_string());
    let uptime = Uptime::new().unwrap();
    let package_count = get_package_count()
        .map(|n| n.to_string())
        .unwrap_or_else(|| "?".to_string());
    let cpu = get_cpu_model().unwrap_or("?".to_string());
    let gpu = get_gpu_model().unwrap_or("?".to_string());
    let ram = get_memory_info().unwrap_or("?".to_string());
    let disk = get_disk_info().unwrap_or("?".to_string());
    let de = get_desktop_environment().unwrap_or("?".to_string());

    let fmt_line = |key: &str, val: &str| format!("{:<width$} {}", key, val, width = MAX_KEY_LENGTH);

    info_lines.push(fmt_line("Абонент", &hostname));
    info_lines.push(fmt_line("Текущий тариф", &ditro));
    info_lines.push(fmt_line("Прошивка роутера", &kernel_version));
    info_lines.push(fmt_line("Админ панель", &de));
    info_lines.push(fmt_line("Найдено вирусов", &package_count));
    info_lines.push(fmt_line("Последнее пополнение", &format!("{} назад", uptime.get())));
    info_lines.push(fmt_line("ЦП", &cpu));
    info_lines.push(fmt_line("ГПУ", &gpu));
    info_lines.push(fmt_line("Остаток интернета", &ram));
    info_lines.push(fmt_line("Остаток на дискете", &disk));

    info_lines.push("".to_string());
    info_lines.push(get_color_blocks_line());

    info_lines
}

fn get_color_blocks_line() -> String {
    let colors = [
        BLACK, DARK_GRAY, LIGHT_GRAY, WHITE,
        BLUE, LIGHT_BLUE, CYAN, GREEN,
        PURPLE, YELLOW, RED,
    ];

    let mut line = String::new();

    for &color in &colors {
        line.push_str(color);
        line.push_str("██");
        line.push_str(NC);
        line.push(' ');
    }

    line
}


fn main() {
    let logo = config::get_logo();
    let info_lines = get_info();

    let logo_width = logo.iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(40);

    let gap_str = " ".repeat(GAP);
    let lines_count = logo.len().max(info_lines.len());

    for i in 0..lines_count {
        let logo_line = logo.get(i).map(String::as_str).unwrap_or("");

        let info_line = info_lines.get(i).map(String::as_str).unwrap_or("");

        println!("{:<logo_width$}{}{}", logo_line, gap_str, info_line,
                 logo_width = logo_width);
    }
}