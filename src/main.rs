use clap::Parser;
use colored::*;
use port_scan::{Port, *};
use std::time::Instant;

fn exit() {
    std::process::exit(0);
}

async fn Run() {
    let time_start = Instant::now();
    let args = Cli::Args::parse();

    if let Some(ip) = args.ip() {
        if !is_valid_ip(ip) && !is_valid_domain(ip) {
            logo_banner();
            print_invalid_address_status();
            print_usage();
            exit();
        }
        let ports = if args.common_ports() {
            Port::MOST_COMMON_PORT_100
        } else if args.well_known_ports() {
            Port::WELL_KNOWN_PORTS
        } else if let Some(ports) = args.ports() {
            &ports
        } else {
            Port::WELL_KNOWN_PORTS
        };
        let results = Socket::tcp_scan_ports(ip, ports).await;
        if results.is_empty() {
            println!();
            println!(
                " [{}] All Port : {}",
                "*".bold().yellow(),
                "Close".bold().red()
            );
        } else {
            println!();
            for result in results {
                println!(
                    " [{}] Port {}/TCP: {}",
                    "+".bold().green(),
                    result.to_string().bold().cyan(),
                    "Open".bold().green()
                );
                if args.banner() {
                    let banner_result = Socket::grab_banner(args.ip().unwrap(), result).await;
                    if let Ok(banner_string) = banner_result {
                        println!();
                        println!(" ------ {} ------ ", "Banner".bold().blue());
                        println!(" > {}", banner_string);
                        println!();
                    }
                }
            }
        }
    } else {
        logo_banner();
        print_usage();
        exit();
    }

    let time_elapsed = time_start.elapsed();
    let mut time_prefix = String::new();
    let time_use = if time_elapsed.as_millis() > 1000 {
        time_prefix = " s".to_string();
        time_elapsed.as_secs() as u128
    } else {
        time_prefix = " ms".to_string();
        time_elapsed.as_millis()
    };
    println!();
    println!(
        " [{}] Time Taken: {}{}",
        "~".magenta().bold(),
        time_use.to_string().bold().magenta(),
        time_prefix
    );
}

#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
async fn main() {
    Run().await;
}
