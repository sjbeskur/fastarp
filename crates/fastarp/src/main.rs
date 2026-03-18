use clap::Parser;
use pnet::datalink;

/// Fast ARP scanner — discover hosts on a local subnet
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Network interface to scan (e.g. eth0, wlan0). Omit to list available interfaces.
    interface: Option<String>,

    /// Timeout in milliseconds to wait for ARP replies (default: 500)
    #[arg(short, long, default_value_t = 500)]
    timeout: u64,
}

fn main() {
    let cli = Cli::parse();

    let Some(interface) = cli.interface else {
        println!("Available interfaces:");
        for iface in datalink::interfaces() {
            let ips: Vec<String> = iface.ips.iter().map(|ip| ip.to_string()).collect();
            let addrs = if ips.is_empty() { String::from("no address") } else { ips.join(", ") };
            println!("  {:<16} {}", iface.name, addrs);
        }
        return;
    };

    let start = std::time::Instant::now();

    let result = match fastarp_core::scan_v4_with_timeout(&interface, cli.timeout) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let scan_time = start.elapsed().as_millis();

    for (_k, n) in &result.nodes {
        println!("{n}");
    }

    println!(
        "{found} hosts found out of {total} IPs on {subnet} in {time}ms",
        found = result.nodes.len(),
        total = result.total_ips,
        subnet = result.subnet,
        time = scan_time,
    );
}
