pub mod Cli;
pub mod Port;
pub mod Socket;

use colored::*;
use regex::Regex;

pub fn print_invalid_address_status() {
    println!();
    println!(
        " [{}] ERROR : {}",
        "-".bold().red(),
        "Invalid Address".bold().red()
    );
}
pub fn is_valid_ip(ip: &str) -> bool {
    let ip_regex = Regex::new(r"^((25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9]{2}|[1-9]?[0-9])$").unwrap();
    ip_regex.is_match(ip)
}

pub fn is_valid_domain(domain_name: &str) -> bool {
    let domain_regex =
        Regex::new(r"^(?i:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\.)+[a-z]{2,}$").unwrap();
    domain_regex.is_match(domain_name)
}

pub fn print_usage() {
    println!();
    println!(
        " Usage: port_scan [{}] [{}]",
        "OPTION".yellow().italic(),
        "IP".yellow().italic()
    );
    println!(" try {} for more info", "--help".red().bold());
}

pub fn logo_banner() {
    let ascii_text = r###"
  _____           _      _____                 
 |  __ \         | |    / ____|                
 | |__) |__  _ __| |_  | (___   ___ __ _ _ __  
 |  ___/ _ \| '__| __|  \___ \ / __/ _` | '_ \ 
 | |  | (_) | |  | |_   ____) | (_| (_| | | | |
 |_|   \___/|_|   \__| |_____/ \___\__,_|_| |_| -- rs"###;
    println!("{}", ascii_text.bold().green());
}
