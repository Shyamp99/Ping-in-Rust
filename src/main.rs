#![allow(warnings)]

mod lib;
use dns_lookup;
use rand;
// use std::io;
use std::net;
use std::io;
use std::io::Error;
use std::net::*;
use std::net::IpAddr;
use std::net::IpAddr::{V4, V6};
use std::net::{Ipv4Addr, Ipv6Addr};
use dns_lookup::lookup_host;
use std::sync::{Arc, Mutex, RwLock};



fn get_ip_addr(input: String, ipv: i8) -> std::net::IpAddr{
    let mut ipaddr;
    //it's an ip addr
    if input.chars().next().unwrap().is_digit(10){
        //ipv6
        if input.contains(":"){
            // ipv= 6;
            ipaddr =  input.parse().unwrap();
            let split_str: Vec<&str> = input.split(':').collect();
            ipaddr = IpAddr::V6(Ipv6Addr::new(split_str[0].parse::<u16>().unwrap(), split_str[1].parse::<u16>().unwrap(), 
                                    split_str[2].parse::<u16>().unwrap(), split_str[3].parse::<u16>().unwrap(), 
                                    split_str[4].parse::<u16>().unwrap(), split_str[5].parse::<u16>().unwrap(), 
                                    split_str[6].parse::<u16>().unwrap(), split_str[7].parse::<u16>().unwrap()));
        }
        //ipv4
        else{
            // ipv = 4;
            ipaddr =  input.parse().unwrap();
            let split_str: Vec<&str> = input.split('.').collect();
            ipaddr = IpAddr::V4(Ipv4Addr::new(split_str[0].parse::<u8>().unwrap(), split_str[1].parse::<u8>().unwrap(), 
                                    split_str[2].parse::<u8>().unwrap(), split_str[3].parse::<u8>().unwrap()));
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
    // ctrlc::set_handler(move || {
    //     ctrl_c_handler(timeout_count, total_count);
    // }).expect("Error setting Ctrl-C handler");


    let argv1 = std::env::args().nth(1).expect("No Argument Given");
    let argv2 = std::env::args().nth(2).expect("No Argument Given");
    let mut ipv:i8 = 0;
    ipv = argv2.parse().unwrap();
    
    let mut ipaddr: std::net::IpAddr = get_ip_addr(argv1.clone(), ipv.clone());
    println!("IPv{} address: {}", argv2, ipaddr);

     //calls function from lib.rs that handles everything
    lib::send_packet(ipaddr, ipv);

}