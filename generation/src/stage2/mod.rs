#![allow(unused)]

use crate::structs::*;
use crate::tcp::*;
use crate::udp::*;
use crate::icmp::*;
use std::fs::File;
use std::fs;
use std::ffi::OsStr;
use std::path::PathBuf;
use rand_pcg::Pcg32;
use rand::prelude::*;
use std::net::Ipv4Addr;
use std::time::{Duration, Instant};

mod automaton;

pub struct Stage2 {
    // TODO: map port -> automata
    tcp_automata: Vec<automaton::TimedAutomaton<TCPEdgeTuple>>,
    udp_automata: Vec<automaton::TimedAutomaton<UDPEdgeTuple>>,
    icmp_automata: Vec<automaton::TimedAutomaton<ICMPEdgeTuple>>,
    rng: Pcg32,
}

impl Stage2 {

    pub fn new(seed: u64) -> Self {
        Stage2 { tcp_automata: vec![], udp_automata: vec![], icmp_automata: vec![], rng: Pcg32::seed_from_u64(seed) }
    }

    pub fn import_automata_from_dir(&mut self, directory_name: &str) -> u32 {
        let mut nb = 0;
        let paths = fs::read_dir(directory_name).expect("Cannot read directory");
        for p in paths {
            let p = p.expect("Cannot open path").path();
            if !p.is_dir() && p.extension() == Some(OsStr::new("json")) {
                match self.import_automata(&p) {
                    Ok(()) => {
                        println!("Automaton {:?} is loaded",p.file_name().unwrap());
                        nb += 1
                    },
                    Err(s) => println!("Could not load automaton {:?} ({})",p.file_name().unwrap(), s),
                }
            }
        }
        println!("{} automata have been loaded",nb);
        nb
    }

    pub fn import_automata(&mut self, filename: &PathBuf) -> std::io::Result<()> {
        let f = File::open(filename)?;
        let a : automaton::JsonAutomaton = serde_json::from_reader(f)?;
        match a.protocol {
            automaton::JsonProtocol::TCP => {
                self.tcp_automata.push(automaton::TimedAutomaton::<TCPEdgeTuple>::import_timed_automaton(a,parse_tcp_symbol)); },
            automaton::JsonProtocol::UDP => {
                self.udp_automata.push(automaton::TimedAutomaton::<UDPEdgeTuple>::import_timed_automaton(a,parse_udp_symbol)); },
            automaton::JsonProtocol::ICMP => {
                self.icmp_automata.push(automaton::TimedAutomaton::<ICMPEdgeTuple>::import_timed_automaton(a,parse_icmp_symbol)); },
        }
        Ok(())
    }

    pub fn generate_tcp_packets_info(&mut self, flow: FlowData) -> PacketsIR<TCPPacketInfo> {
        // TODO: select correct TCP automata depending on the port
        let automata = &self.tcp_automata[0];
        // let automata = automata.intersect_automata(&automaton::new_packet_number_constraints_automaton::<TCPPacketInfo>(&flow));
        // let automata = automata.intersect_automata(&automaton::new_tcp_flags_constraints_automaton(&flow));
        let packets_info = automata.sample(&mut self.rng, flow.timestamp, create_tcp_header);
        PacketsIR::<TCPPacketInfo> { packets_info, flow: Flow::TCPFlow(flow) }
    }

    pub fn generate_tcp_packets_info_no_flow(&mut self, ts: Instant) -> PacketsIR<TCPPacketInfo> {
        let automata = &self.tcp_automata[0];
        let packets_info = automata.sample(&mut self.rng, ts, create_tcp_header);
        // Reconstruct flow from sample
        let flow = Flow::TCPFlow(FlowData {
            src_ip: Ipv4Addr::new(192, 168, 1, 8),
            dst_ip: Ipv4Addr::new(192, 168, 1, 14),
            src_port: 34200,
            dst_port: 8080,
            recorded_ttl_client: 23,
            recorded_ttl_server: 68,
            initial_ttl_client: 255,
            initial_ttl_server: 255,
            fwd_packets_count: packets_info.iter().filter(|p| p.direction == PacketDirection::Forward).count() as u32,
            bwd_packets_count: packets_info.iter().filter(|p| p.direction == PacketDirection::Backward).count() as u32,
            fwd_total_payload_length: packets_info.iter().filter(|p| p.direction == PacketDirection::Forward).map(|p| p.payload.get_payload_size()).sum::<usize>() as u32,
            bwd_total_payload_length: packets_info.iter().filter(|p| p.direction == PacketDirection::Backward).map(|p| p.payload.get_payload_size()).sum::<usize>() as u32,
            timestamp: ts,
            total_duration: packets_info.last().unwrap().ts.duration_since(ts)
            } );
        PacketsIR::<TCPPacketInfo> { packets_info, flow }
    }

    pub fn generate_udp_packets_info(&self, flow: FlowData) -> PacketsIR<UDPPacketInfo> {
        panic!("Not implemented");
    }

    pub fn generate_icmp_packets_info(&self, flow: FlowData) -> PacketsIR<ICMPPacketInfo> {
        panic!("Not implemented");
    }

}
