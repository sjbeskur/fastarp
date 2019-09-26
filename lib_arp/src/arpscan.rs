use std::collections::{ HashMap };
use std::time::{ SystemTime };
use std::thread;
use std::thread::{JoinHandle};
use std::sync::mpsc::{ self, Sender, Receiver };
use std::net::{ IpAddr, Ipv4Addr };

use ipnetwork::*;
use pnet::datalink::{ self, Channel, NetworkInterface, MacAddr };
use pnet::packet::{ Packet, MutablePacket };
use pnet::packet::ethernet::{ MutableEthernetPacket, EtherTypes, EthernetPacket };
use pnet::packet::arp::{ ArpHardwareTypes, ArpOperations, ArpOperation, ArpPacket, MutableArpPacket };
use super::arpnode::{ArpNode};
use super::ArpResult;

type SenderChannel = Sender<(Ipv4Addr, MacAddr, Ipv4Addr, SystemTime)>;
type ReceiverChannel = Receiver<(Ipv4Addr, MacAddr, Ipv4Addr, SystemTime)>;


pub fn scan_v4(iface_name: &str) -> std::collections::HashMap<String, ArpNode>{
    let ips = get_iface_ips(iface_name);
    let iface = validate_interface(iface_name.to_string()).unwrap();
    let source_network = iface.ips.iter().find(|x| x.is_ipv4()).unwrap();
    let source_ip      = source_network.ip();

    let (hdl, reciever) = listen_for_arp(iface.clone());

    let chunk_size = compute_chunk_size(ips.len());
    println!("chunk_size = {}", chunk_size);
    let chunks: Vec<_> = ips.chunks(chunk_size).map(|c| c.to_owned()).collect();

    let mut handles = vec![];
    
    let mut idx = 0;
    for list in chunks{
        let intf = iface.clone();
        handles.push( std::thread::spawn( move ||
            send_arp(intf, source_ip.clone(), &list )
          ));
        idx = idx + 1;
    }

    let mut timer_map: HashMap<Ipv4Addr,SystemTime> = HashMap::new();
    for h in handles{
        if let Ok(map) = h.join(){
            timer_map.extend(map);
        }
    }

    let mut nodes = std::collections::HashMap::new();
    while let Ok(arp) = reciever.try_recv(){
        let t = match timer_map.get(&arp.0){                
            Some(t_sent) => { 
                let arrived = arp.3;
                match arrived.duration_since(*t_sent){
                    Ok(time_dif) => { time_dif.as_millis() },
                    _ => { 0 as u128 }
                }
                
            },
            _ => { 
                println!("[not found] {}", arp.0);
                0 as u128 }
        };

        let n = ArpNode { mac_address: arp.1.to_string(), ipv4_address: arp.0.to_string(), ipv4_target: arp.2.to_string(), ping_ms: t  };
        nodes.insert(n.mac_address.clone(), n);
    }
  //  hdl.thread().unpark();
 //   readline();
    nodes
}

fn compute_chunk_size(list_size: usize) -> usize{

    if list_size < 256 {
        return 1;
    }

    if list_size > 65536{
        warn!("this network is to big for nemesis to support and we should Panic");
        return 410;  
        
    }

    // this is basically the log for computing population doubling time
    // applied to then scale chunk sizes based on the size of the list
    // see adm spreadsheet for chunks/ hosts/ threads
    let hosts = list_size as f32;
    let two = 2.0_f32;
    let onedot5 = 1.5_f32;
    let distance = (hosts.log10() / two.log10() - 8.0) as i32;
    let rslt = onedot5.powi(distance) * 16.0;
        
    return rslt as usize;
}


fn send_arp(iface: NetworkInterface,  source_ip: IpAddr, ip_list: &[Ipv4Addr] ) -> HashMap<Ipv4Addr,SystemTime> {
    let mut timer_map: HashMap<Ipv4Addr,SystemTime> = HashMap::new();
    for target_ip in ip_list.iter(){
        match source_ip{
            IpAddr::V4(source_v4) =>{
                let r = match send_arp_packet(iface.clone(), source_v4, *target_ip){
                    Ok(it) => { timer_map.insert(it.0, it.1); },
                    Err(_) => {  /* do nothing*/  }
                };
            },
            _ => {
                println!("unsupported address type");
            }
        }
    }
    return timer_map;
}


pub fn get_iface_ips(target_iface: &str) -> Vec<Ipv4Addr>{
    let iface = validate_interface(target_iface.to_string()).unwrap();
    let source_network = iface.ips.iter().find(|x| x.is_ipv4()).unwrap();
    let mut ips_v4: Vec<Ipv4Addr> = Vec::new();
    if let &IpNetwork::V4(network) = source_network{
        for ip in network.iter(){
            ips_v4.push(ip);
        }
    }
    ips_v4
}
//impl ArpScan{

/*
    pub fn scan(&self ) -> HashMap<String, ArpNode>{
        let source_network = self.iface.ips.iter().find(|x| x.is_ipv4()).unwrap();
        let source_ip      = source_network.ip();
                    
        let mut timer_map: HashMap<Ipv4Addr,SystemTime> = HashMap::new();

        match source_network {
            &IpNetwork::V4(source_networkv4) => {
                for target_ipv4 in source_networkv4.iter() {
                    match source_ip {
                        IpAddr::V4(source_ipv4) => {
                            let now = SystemTime::now();
                            timer_map.insert(target_ipv4, now);
                            super::arpscan::send_arp_packet(self.iface.clone(), source_ipv4, target_ipv4);
                        },
                        e => {
                            error!("Error while parsing to IPv4 address: {}", e);
                            panic!("Error while parsing to IPv4 address: {}", e)
                        }
                    }
                }
            },
            e => {
                error!("Error while attempting to get network for interface: {}", e);
                panic!("Error while attempting to get network for interface: {}", e)
            }
        }        

        let mut node_map: HashMap<String, ArpNode> = HashMap::new();

        loop {
            match self.receiver.try_recv() {
                Ok((ipv4_addr, mac_addr, arrived)) => {
                    let mut elapsed = 0;                            
                    if let Some(sent) = timer_map.get(&ipv4_addr){
                        if arrived > *sent {
                            elapsed = arrived.duration_since(*sent).unwrap().as_millis();
                        }else{
                            debug!("arp response recieved from: {} outside of expected window (non critical)",ipv4_addr);
                        }
                    }
                    let node = ArpNode{
                        mac_address: mac_addr.to_string(),
                        ipv4_address: ipv4_addr.to_string(),
                        ping_ms: elapsed as u64
                    };
                    node_map.insert(mac_addr.to_string(), node.clone());
                },
                Err(_) => break
            }
        }

        node_map        
    }
*/



pub fn listen_for_arp( interface: NetworkInterface ) -> (JoinHandle<()>,  ReceiverChannel) {

    let (sender, receiver): (SenderChannel, ReceiverChannel) = mpsc::channel();

    let h = thread::spawn(move || {
        let _tid = thread::current().id();

        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("Unknown channel type"),
            Err(e) => panic!("Error happened {}", e)
        };

        //loop {
            //match rx.next() {
                while let Ok(data) = rx.next() {
                    let ethernet_packet = EthernetPacket::new(data).unwrap();
                    let ethernet_payload = ethernet_packet.payload();
                    let arp_packet = ArpPacket::new(ethernet_payload).unwrap();
                    let arp_reply_op = ArpOperation::new(2_u16);

                    if arp_packet.get_operation() == arp_reply_op {                    
                        let result: (Ipv4Addr, MacAddr, Ipv4Addr, SystemTime) = (arp_packet.get_sender_proto_addr(), arp_packet.get_sender_hw_addr(), arp_packet.get_target_proto_addr(), SystemTime::now() ) ;
                        trace!("{}\t{} {}", result.0, result.1, result.2 );
                        match sender.send(result){  // send the result to the mpsc channel
                          Ok(_r) => { 
                                trace!("arp-packet sent OK to: {}",result.0);                                     
                          },
                          _e => { error!("Error sending message to: {}", result.0) }
                        }                            
                    }
                }
              // , Err(e) => error!("An error occurred while reading packet: {:?}", e)
            //}
        //}
        println!("Listen exiting");
    });
    println!("Listen thread spawned");
    return ( h, receiver);

}


pub fn send_arp_packet( interface: NetworkInterface, source_ip: Ipv4Addr,  target_ip: Ipv4Addr) -> ArpResult<(Ipv4Addr, SystemTime)>{

    let source_mac     = interface.mac_address();
    let target_mac     = MacAddr::new(255,255,255, 255,255,255);

    trace!("sending arp packet to {} - {} ", target_ip.to_string(), source_mac);//chrono::Local::now());

    let (mut tx, _) = match datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e)
    };

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(target_mac);
    ethernet_packet.set_source(source_mac);
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(target_mac);
    arp_packet.set_target_proto_addr(target_ip);
    ethernet_packet.set_payload(arp_packet.packet_mut());
            
    let sent = SystemTime::now();
    tx.send_to(ethernet_packet.packet(), Some(interface));        

    Ok(( target_ip, sent ))
}

pub fn validate_interface(target_iface: String) -> crate::ArpResult<NetworkInterface>{
    //let iface_match = | iface: &NetworkInterface | iface.name == target_iface ;
    let interfaces = datalink::interfaces();
    interfaces.into_iter()
            //.filter(iface_match)
            .find(| iface | iface.name == target_iface)
            .ok_or_else( || crate::ArpErrors::ArpError(format!("Invalid Network Interface. No such device {}.",target_iface)))
            .and_then(|iface| validate_iface(iface))
    
}


pub fn validate_iface(target_iface: NetworkInterface) -> crate::ArpResult<NetworkInterface>{

    if target_iface.is_loopback(){
        return Err(crate::ArpErrors::ArpError( format!("Invalid Network Interface. Target interface {} is loopback.",target_iface.name) ) );
    }

    if target_iface.ips.is_empty(){
        return Err(crate::ArpErrors::ArpError( format!("Invalid Network Interface. Target interface {} has no associated network address.",target_iface)));
    }
    Ok(target_iface)
}
