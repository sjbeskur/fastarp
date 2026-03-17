use clap::Parser;
use pnet::datalink;

/// Fast ARP scanner — discover hosts on a local subnet
#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// Network interface to scan (e.g. eth0, wlan0). Omit to list available interfaces.
    interface: Option<String>,
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

    let node_map = match lib_arp::scan_v4(&interface) {
        Ok(nodes) => nodes,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let scan_time = start.elapsed().as_millis();

    for (_k, n) in &node_map {
        println!("{n}");
    }

    println!("{count} nodes scanned in {time}ms", time = scan_time, count = node_map.len());
}
