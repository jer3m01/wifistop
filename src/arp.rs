//! ARP functions

use pnet::datalink::{Channel, DataLinkReceiver, DataLinkSender, MacAddr, NetworkInterface};
use pnet::packet::arp::{
    Arp, ArpHardwareTypes, ArpOperation, ArpOperations, ArpPacket, MutableArpPacket,
};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::{FromPacket, MutablePacket, Packet};
use std::io::{ErrorKind, Result};
use std::net::{IpAddr, Ipv4Addr};

pub struct WSArpPacket<'a> {
    pub tx: &'a mut dyn DataLinkSender,
    pub src_ip: Ipv4Addr,
    pub src_mac: MacAddr,
    pub dest_ip: Ipv4Addr,
    pub dest_mac: MacAddr,
    pub op: ArpOperation,
}

pub fn send_arp_packet(packet: WSArpPacket) -> Result<()> {
    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();
    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(packet.op);
    arp_packet.set_sender_hw_addr(packet.src_mac);
    arp_packet.set_sender_proto_addr(packet.src_ip);
    arp_packet.set_target_hw_addr(packet.dest_mac);
    arp_packet.set_target_proto_addr(packet.dest_ip);

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();
    ethernet_packet.set_destination(packet.dest_mac);
    ethernet_packet.set_source(packet.src_mac);
    ethernet_packet.set_ethertype(EtherTypes::Arp);
    ethernet_packet.set_payload(arp_packet.packet_mut());

    match packet.tx.send_to(ethernet_packet.packet(), None) {
        Some(a) => a,
        None => panic!("send_to() returned None"),
    }
}
