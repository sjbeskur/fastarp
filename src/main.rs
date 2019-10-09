
#[macro_use] extern crate log;

extern crate lib_arp;
extern crate pnet;
extern crate time;

use lib_arp::*;
use time::PreciseTime;

fn main() {

    println!("Fast Arp experiment");
    
    is_user_sudo();

    let args: Vec<String> = std::env::args().collect();    
    if args.len() < 2 {
        show_usage();
        std::process::exit(1);
    }

    let interface = &args[1];
    
    let start = PreciseTime::now();

    match lib_arp::scan_v4(interface){
        Ok(nodes) => {
            let end = PreciseTime::now();
            let scan_time = start.to(end).num_milliseconds() as f64;
            dump_nodes(&nodes);
            println!("{count} nodes scanned in {time:.2}", time = scan_time / 1000.0, count = nodes.len())
        }
        Err(e) => error!("Error:{}", e)
    }
}

fn dump_nodes(nodes: &std::collections::HashMap<String, ArpNode>){
    for (_k, n) in nodes {
        println!("{:?}", n);
    }

}

fn is_user_sudo(){
    if users::get_effective_uid() != 0 {
        println!("ERROR: Must run nemesis as root");
        std::process::exit(1);
    }
}

fn show_usage(){
    println!("\nUsage:");
    println!("sudo ./fastarp <interface_name>\n");
}

