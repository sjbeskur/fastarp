extern crate lib_arp;
extern crate pnet;
extern crate time;

use lib_arp::*;
use std::io::{ Read };
use time::{PreciseTime};

fn main() {

    println!("Fast Arp experiment");
    
    let args: Vec<String> = std::env::args().collect();    
    println!("{:?}", args);
    let interface = &args[1];
    
    let start = PreciseTime::now();

    let node_map = lib_arp::scan_v4(interface);
    
    let end = PreciseTime::now();
    let scan_time = start.to(end).num_milliseconds() as f64;

    dump_nodes(&node_map);

    println!("{count} nodes scanned in {time:.2}", time = scan_time / 1000.0, count = node_map.len());
}

fn dump_nodes(nodes: &std::collections::HashMap<String, ArpNode>){
    for (_k, n) in nodes {
        println!("{:?}", n);
    }

}

fn readline() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    Ok(())
}




