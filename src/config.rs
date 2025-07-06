pub const BLACK: &str = "\x1b[0;30m";
pub const DARK_GRAY: &str = "\x1b[1;30m";
pub const LIGHT_GRAY: &str = "\x1b[0;37m";
pub const WHITE: &str = "\x1b[1;37m";
pub const BLUE: &str = "\x1b[0;34m";
pub const LIGHT_BLUE: &str = "\x1b[1;34m";
pub const CYAN: &str = "\x1b[0;36m";
pub const GREEN: &str = "\x1b[0;32m";
pub const PURPLE: &str = "\x1b[0;35m";
pub const YELLOW: &str = "\x1b[1;33m";
pub const RED: &str = "\x1b[0;31m";
pub const NC: &str = "\x1b[0m";

pub const GAP: usize = 4;
pub const MAX_KEY_LENGTH: usize = 22;

/// Print ASCII logo with colors
pub fn get_logo() -> Vec<String> {
    let accent = LIGHT_BLUE;
    let nc = NC;

    vec![
        format!("        {accent}░ ░░░░                            ░░░░░░{nc}"),
        format!("        {accent}░░░░░░                            ░░░░░░{nc}"),
        format!("        {accent}░░░▓▓░░░░   ░░░░░░░░░░░░░░░░░  ░░░░▒▒░░{nc}"),
        format!("         {accent}░░▓▓▓▓▓▓░░░░░▒▒▒▒▒▒▒▒▒▒▒▒░░░░░▒▒▒▒▒▒░░{nc}"),
        format!("           {accent}░▓▓▓▓▓▓▓▓▓░░░▒▒▒▒▒▒▒▒░░░▒▒▒▒▒▒▒▒▒▒░{nc}"),
        format!("           {accent}░░▓▓▓▓▓▓▓▓▓▓▒░░▒▒▒▒░░▒▒▒▒▒▒▒▒▒▒▒░░{nc}"),
        format!("        {accent}░░░▒▓▓░░░░░░░░░▓▓░░▒▒░░▒▒░░░░░░░░░▒▒░░░░{nc}"),
        format!("        {accent}░░▓▓▓░   ░██░░░░▓▓░░░▒▒▒░░▒░██░░░░░▒▒▒░░{nc}"),
        format!("        {accent}░░▓▓░░ ░░██░░░█░░▓▓░░▒▒░░▒░░░██░  ░░▒▒░{nc}"),
        format!("        {accent}░░▓▓░  ░░███▓█▓░░▓▓░░▒▒░░██▓███░░ ░░▒▒░░{nc}"),
        format!("        {accent}░░▓▓░░   ░░█▓░░░░▓▓░░▒▒░░░░▓█░░   ░░▒▒░░{nc}"),
        format!("         {accent}░░▓▓░░░░  ░░░░▒▓▓░░░░▒▒░░░░░░  ░░░░▒▒░░░{nc}"),
        format!("        {accent}░░░░▓▓▓▓░░░░░▓▓▓▒░░▒▒░░▒▒▒▒░░░░░▒▒▒▒░░░░{nc}"),
        format!("           {accent}░░░░▓▓▓▓▓▓▒░░░░░▒▒░░░░░▒▒▒▒▒▒▒░░░░{nc}"),
        format!("           {accent}░░░▒▒░░░░░▒▒▒▒▒░░░░▒▒▒▒▒░░░░░▒▒░░░{nc}"),
        format!("           {accent}░░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░░{nc}"),
        format!("              {accent}░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░{nc}"),
        format!("                 {accent}░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░{nc}"),
        format!("                 {accent}░░░░▒▒▒▒▒▒▒▒▒▒▒▒▒▒░░░░{nc}"),
        format!("                   {accent}░░░░▒▒▒▒▒▒▒▒▒▒░░░░{nc}"),
        format!("                      {accent}░░░▒▒▒▒▒▒░░░{nc}"),
        format!("                      {accent}░░░░░▒▒░░░░░{nc}"),
        format!("                         {accent}░░░░░░{nc}"),
    ]
}