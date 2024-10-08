#![allow(unused)]

use std::net::Ipv4Addr;
use std::time::{Duration, Instant};
use std::fmt::Debug;
use serde::Deserialize;

// Stage 1 and 2 structures

#[derive(Debug)]
pub enum Flow {
    TCPFlow(FlowData),
    UDPFlow(FlowData),
    ICMPFlow(FlowData)
}

#[derive(Debug)]
pub struct FlowData {
    pub src_ip: Ipv4Addr,
    pub dst_ip: Ipv4Addr,
    pub src_port: u16,
    pub dst_port: u16,
    pub recorded_ttl_client: u8,
    pub recorded_ttl_server: u8,
    pub initial_ttl_client: u8,
    pub initial_ttl_server: u8,
    pub fwd_packets_count: u32,
    pub bwd_packets_count: u32,
    pub fwd_total_payload_length: u32,
    pub bwd_total_payload_length: u32,
    pub timestamp: Instant,
    pub total_duration: Duration
}

// Stage 2 structures

#[derive(Debug,Clone)]
pub enum PayloadType {
    Empty,
    Text(Vec<String>),
    Replay(Vec<Vec<u8>>),
    Random(Vec<usize>)
}

pub trait EdgeType : Debug {
    fn get_payload_type(&self) -> &PayloadType;
}

// Stage 2 and 3 structures

#[derive(Debug,Clone,Copy)]
pub enum NoiseType {
    None,
    Deleted,
    Reemitted,
    Transposed,
    Added
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PacketDirection {
    Forward, // client to server
    Backward, // server to client
}

#[derive(Debug,Clone)]
pub enum Payload {
    Empty,
    Replay(Vec<u8>),
    Random(usize)
}

impl Payload {
    pub fn get_payload_size(&self) -> usize {
        match &self {
            Payload::Empty => 0,
            Payload::Replay(l) => l.len(),
            Payload::Random(len) => *len,
        }
    }

}

pub trait Protocol {
    fn get_noise_type(&self) -> NoiseType;
    fn get_direction(&self) -> PacketDirection;
    fn get_ts(&self) -> Instant;
}

#[derive(Debug)]
pub struct PacketsIR<T: Protocol> { // Intermediate representation (as output by stage 2)
    pub packets_info: Vec<T>,
    pub flow: Flow
}

// Stage 3 structures

pub struct Packet {
    // should be replaced by the Packet structure defined in the network library
}
