
use icmp::IcmpSocket;
use pnet::packet::icmp::echo_request::{MutableEchoRequestPacket, EchoRequestPacket};
use pnet::packet::icmp::echo_reply::EchoReplyPacket;
use pnet::packet::icmp::{IcmpPacket, IcmpTypes, IcmpCode, checksum};
use pnet::packet::Packet;
use std::time::Instant;
use libc as c;
// use std::net::IpAddr;
// use std::net::IpAddr::{V4, V6};
// use std::net::{Ipv4Addr, Ipv6Addr};

// fn ctrl_c_handler(&mut total_packets:u16, &mut total_time:f64, &mut recieved:u128, &mut missed:u128, &mut max:f64 , &mut min:f64) -> (){
//     println!("\n -------------PING STATISTICS-------------");
//     println!("Total Packets Sent: {0}, Total Recieved: {1}, Total missed: {2}, {3} % packet loss", *total_packets, *recieved, *missed, *missed as f64/ *total_packets as f64);
//     println!("Total Time {0}ms, Average Time {1} ms, Max Time {2} ms, Min Time {3} ms ", *total_time, *total_time / *total_packets as f64,  *max, *min);
//     std::process::exit(0);
// }

fn make_packet(buf: &mut [u8], seq: u16, ) -> MutableEchoRequestPacket{
    let mut pack = MutableEchoRequestPacket::new(buf).unwrap();
    pack.set_sequence_number(seq);
    pack.set_icmp_type(IcmpTypes::EchoRequest);
    pack.set_identifier(seq);
    pack.set_icmp_code(IcmpCode::new(0));
    
    let pack_checksum = checksum(&IcmpPacket::new(pack.packet()).unwrap());
    pack.set_checksum(pack_checksum);
    return pack
}


pub fn send_packet(ipaddr: std::net::IpAddr, ipv:i8) {
    // logic: create, connect(ipaddr), make_packet(), socket.send(buffer), recv(buffer)
    // let socket = IcmpSocket::new();
    println!("{:?}", ipaddr);
    let mut total_packets:u16 = 0; 
    let mut missed:u128 = 0;
    let mut recieved:u128 = 0;

    let mut total_time:f64 = 0.0;
    let mut max:f64 = 0.0;
    let mut min:f64 = 0.0;
    

    // ctrlc::set_handler(move || {
    //     ctrl_c_handler(total_packets, total_time, recieved, missed, max, min );
    //     // break;
    //     // signal = true;
    // }).expect("Error setting Ctrl-C handler");

    let mut sock = match IcmpSocket::connect(ipaddr){
        Ok(s) => s,
        Err(e) => panic!("UNABLE TO OPEN ICMP SOCKET: {:?}", e),
    };
    if ipv == 6{
        println!("{:?}", sock.qos().unwrap());
        sock.set_qos(c::AF_INET6 as u8);
        println!("{:?}", sock.qos().unwrap());
    }
    //for loop for initial testing 
    for _ in 0..5{
    // while !signal {
        //make packet
        let min_size = EchoReplyPacket::minimum_packet_size(); 
        let mut buffer: Vec <u8> = vec![0;min_size];
        let mut recv_buffer: Vec <u8> = vec![0;min_size];

        //actual struct stored to check values during testing/getting respective info
        let _pack = make_packet(&mut buffer[..], total_packets);
        let start = Instant::now();
        
        //send packet
        match sock.send(&mut buffer){
            Ok(f) => f,
            Err(e) => panic!("ERROR AT SEND"),
        };
        //wait for recieve
        sock.recv(&mut recv_buffer);
        //check this to make sure logic works
        if let Some(recv_buffer) = EchoReplyPacket::new(&recv_buffer[..]){
            println!("{:?}", recv_buffer);
            recieved+=1;
        }

        let elapsed = start.elapsed().as_millis();
        if (elapsed as f64) > max {
            max = elapsed as f64;
        }
        else if (elapsed as f64) < min || min == 0.0 {
            min = elapsed as f64;
        }
        total_time += elapsed as f64;
        total_packets += 1;
        std::thread::sleep( std::time::Duration::from_millis(500));
    };
    
    println!("\n -------------PING STATISTICS-------------");
    println!("Total Packets Sent: {0}, Total Recieved: {1}, Total missed: {2}, {3} % packet loss", total_packets, recieved, missed, missed as f64/total_packets as f64);
    println!("Total Time {0}ms, Average Time {1} ms, Max Time {2} ms, Min Time {3} ms ", total_time, total_time/total_packets as f64,  max, min);

}

// fn handle_v6() {}



// pub fn run(ipv:i8, ipaddr: std::net::IpAddr, mut timeout_count:u128, mut total_count:u128){ }
