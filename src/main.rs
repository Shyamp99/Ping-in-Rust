#![allow(warnings)]
use pnet;
use dns_lookup;
use rand;
// use std::io;
// use std::net;
use std::net::IpAddr;
use std::net::IpAddr::{V4, V6};
use std::net::{Ipv4Addr, Ipv6Addr};
use dns_lookup::lookup_host;
use pnet::packet::{util, icmp, Packet};
use pnet::packet::icmp::{echo_reply, echo_request, IcmpPacket, IcmpTypes};
use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmpv6::{Icmpv6Types, MutableIcmpv6Packet};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::transport::{transport_channel, TransportReceiver, TransportSender};
use pnet::transport::TransportChannelType::Layer4;
use pnet::transport::TransportProtocol::{Ipv4, Ipv6};
use std::sync::{Arc, Mutex, RwLock};
// use pnet::packet::ipv4::Ipv4Packet;
// use pnet::packet::ipv6::Ipv6Packet;
// use pnet::packet::icmp::{echo_reply, echo_request, IcmpPacket, IcmpTypes};
// use pnet::packet::icmpv6::Icmpv6Packet;

fn ctrl_c_handler(timeout_count:i8, total_count:i8) -> (){

    std::process::exit(0);
}

fn send_v4_echo(ts: &mut TransportSender, ipaddr: IpAddr, ipv:i8) -> Result<usize, std::io::Error>{
    let mut vector: Vec<u8> = vec![0; 16];
    let mut pack = MutableEchoRequestPacket::new(&mut vector[..]).unwrap();
    pack.set_sequence_number(rand::random::<u16>());
    pack.set_identifier(rand::random::<u16>());
    pack.set_icmp_type(IcmpTypes::EchoRequest);
    
    let checksum = util::checksum(&pack.packet(), 1);
    pack.set_checksum(checksum);

    return ts.send_to(pack, ipaddr);
}

fn send_v6_echo(ts: &mut TransportSender, ipaddr: IpAddr, ipv:i8) -> Result<usize, std::io::Error>{
    let mut vector: Vec<u8> = vec![0; 16];
    let mut pack = MutableIcmpv6Packet::new(&mut vector[..]).unwrap();
    pack.set_icmpv6_type(Icmpv6Types::EchoRequest);

    let checksum = util::checksum(&pack.packet(), 1);
    pack.set_checksum(checksum);

    return ts.send_to(pack, ipaddr);

}

fn get_ip_addr(input: String, ipv: i8) -> std::net::IpAddr{
    let ipaddr;
    //it's an ip addr
    if input.chars().next().unwrap().is_digit(10){
        //ipv6
        if input.contains(":"){
            // ipv= 6;
            ipaddr =  input.parse().unwrap();
            // let split_str: Vec<&str> = input.split(':').collect();
            // ipaddr = iPaddr::V6(Ipv6Addr::new(split_str[0].parse::<i32>().unwrap(), split_str[1].parse::<i32>().unwrap(), 
                                    // split_str[2].parse::<i32>().unwrap(), split_str[3].parse::<i32>().unwrap(), 
                                    // split_str[4].parse::<i32>().unwrap(), split_str[5].parse::<i32>().unwrap(), 
                                    // split_str[6].parse::<i32>().unwrap(), split_str[7].parse::<i32>().unwrap()));
        }
        //ipv4
        else{
            // ipv = 4;
            ipaddr =  input.parse().unwrap();
            // let split_str: Vec<&str> = input.split('.').collect();
            // ipaddr = iPaddr::V4(Ipv4Addr::new(split_str[0].parse::<i32>().unwrap(), split_str[1].parse::<i32>().unwrap(), 
                                    // split_str[2].parse::<i32>().unwrap(), split_str[3].parse::<i32>().unwrap()));
        }
    }
    //is a hostname
    else{
        //NEED TO FIX IN TESTING: RN UNWRAP J GIVES VEC<std::net::IpAdr>
        let mut ipaddrs: Vec<std::net::IpAddr> = lookup_host(&input).unwrap();
        let mut temp = 0;
        //it's janky but it works
        ipaddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

        for ip in ipaddrs{
            if ipv == 6 && temp == 0{
                temp = 1;
                continue;
            }
            // println!("a: {}",  a);
            return ip;
        
        }
    }
    return ipaddr;
}

fn main() {
    let mut timeout_count:i8 = 0;
    let mut total_count:i8 = 0;
    ctrlc::set_handler(move || {
        ctrl_c_handler(timeout_count, total_count);
    }).expect("Error setting Ctrl-C handler");


    let argv1 = std::env::args().nth(1).expect("No Argument Given");
    let argv2 = std::env::args().nth(2).expect("No Argument Given");
    let mut ipv:i8 = 0;
    ipv = argv2.parse().unwrap();
    
    let mut ipaddr: std::net::IpAddr = get_ip_addr(argv1, ipv);
    println!("ipaddr: {}", ipaddr);

    let mut ts_arc:Arc<Mutex<TransportSender>>;
    if ipv == 4{
        let mut temp = transport_channel(4096, Layer4(Ipv6(IpNextHeaderProtocols::Icmp)));
        ts_arc = Arc::new(Mutex::new(temp.unwrap()));
        
        //for testing
        send_v4_echo(&mut ts_arc.lock().unwrap(), ipaddr, ipv);
    }
    else{
        let mut temp = transport_channel(4096, Layer4(Ipv4(IpNextHeaderProtocols::Icmpv6)));
        ts_arc = Arc::new(Mutex::new(temp));
        
        //for testing
        send_v6_echo(&mut ts_arc.lock().unwrap(), ipaddr, ipv);
    }
    
    // let mut result = send_v4_echo()

    // let mut result_packet;
    // //waiting for SIGINT to stop the loop:
    // while true{
    //     if ipv == 4{
    //         result_packet = send_v4_echo();
    //     }
    //     else {
    //         result_packet = send_v6_echo();
    //     }
    //     count+=1;
    // }

}


// mod Packet{
//     pub struct Packet{
//         typ:u8, 
//         checksum:u16, 
//         code:u8, 
//         rest:u32, 
//         type_ip: i8
//     }

//     pub fn new(typ:u8, code:u8, rest:u32, type_ip: i8 ) -> Packet{
//         let mut temp = typ as u16;
//         temp += code as u16;
//         temp += rest as u16;

//         Packet{
//             typ:typ,
//             code:code,
//             checksum:temp,
//             rest:rest,
//             type_ip:type_ip
//         }
//     }

//     impl Packet for IcmpPacket{

//     }

// }