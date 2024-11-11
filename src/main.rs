
//#[macro_use] extern crate log;

extern crate lib_arp;
extern crate pnet;
extern crate time;

use lib_arp::*;
use std::io::{ Read };
//use time::{PreciseTime};

fn main() {

    println!("Fast Arp experiment");
    
    //is_user_sudo();

    let args: Vec<String> = std::env::args().collect();    
    if args.len() < 2 {
        show_usage();
        std::process::exit(1);
    }

    let interface = &args[1];
    
    let start = std::time::Instant::now();

    let node_map = lib_arp::scan_v4(interface).unwrap();
    
    let end = std::time::Instant::now();
    let scan_time = start.elapsed().as_millis();

    dump_nodes(&node_map);

    println!("{count} nodes scanned in {time:.2}ms", time = scan_time, count = node_map.len());
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

