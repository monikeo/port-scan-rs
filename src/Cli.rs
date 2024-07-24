use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "Port Scanner",
    version = "1.0",
    about = "Port Scanner written in Rust",
    author = "M0NI KE0"
)]
pub struct Args {
    #[arg(required = true, help = "IP address or Domain name to scan")]
    ip: Option<String>,
    #[arg(
        short = 'p',
        long = "port",
        value_name = "Port",
        value_delimiter = ',',
        required = false,
        help = "Comma-separated list of ports to scan",
    )]
    ports: Option<Vec<u16>>,
    #[arg(
        short = 't', 
        long = "thread", 
        required = false, 
        help = "Number of thread to use"
    )]
    thread_use: Option<u16>,
    #[arg(
        short = 'u', 
        long = "udp", 
        help = "UDP connection mode, default is using TCP connection",  
        required = false, 
        action = clap::ArgAction::SetTrue
    )]
    udp_scan_mode: bool,
    #[arg(
        short = 'b',
        long = "banner",
        help = "Scanning with Banner",
        required = false,
        action = clap::ArgAction::SetTrue
    )]
    banner: bool,
    #[arg(
        long = "parallel", 
        help = "Running the task into parallel mode, default is async", 
        required = false, 
        action = clap::ArgAction::SetTrue
    )]
    parallel: bool,
    #[arg(
        long = "common_ports", 
        required = false, 
        action = clap::ArgAction::SetTrue, 
        help = "Scan common ports"
    )]
    common_ports: bool,
    #[arg(
        long = "well_known_ports", 
        required = false, 
        action = clap::ArgAction::SetTrue,
        help = "Scan well known ports [1--1024]",
    )]
    well_known_ports: bool,
}

impl Args {
    pub fn ip(&self) -> Option<&String> {
        self.ip.as_ref()
    }
    pub fn ports(&self) -> Option<&Vec<u16>> {
        self.ports.as_ref()
    }
    pub fn common_ports(&self) -> bool {
        self.common_ports
    }
    pub fn well_known_ports(&self) -> bool {
        self.well_known_ports
    }
    pub fn banner(&self) -> bool {
        self.banner
    }
}


