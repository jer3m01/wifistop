//! INET functions

use crate::arp;
use crate::utils;
use cached::proc_macro::cached;
use pnet::datalink::{MacAddr, NetworkInterface};
use std::fs::File;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};
use std::str::FromStr;
use std::time::{Duration, Instant};

/// Get default interface
#[cached]
pub fn default_interface() -> NetworkInterface {
    // Doesn't work on windows https://github.com/libpnet/libpnet/blob/4c4f7175c851aaac9db1b04b7d6ca83f7d839789/pnet_datalink/src/winpcap.rs#L332
    if !utils::os::is_windows() {
        return pnet::datalink::interfaces()
            .iter()
            .find(|e| e.is_up() && !e.is_loopback() && !e.ips.is_empty())
            .expect("No default interface")
            .clone();
    }

    let index = utils::windows::run_powershell(
        "Get-NetRoute -DestinationPrefix \"0.0.0.0/0\" \
    | Select-Object -last 1 \
    | Select-Object -ExpandProperty \"IfIndex\"",
    )
    .trim() // Remove trailing "\r\n"
    .parse::<u32>()
    .expect("Invalid interface id");

    pnet::datalink::interfaces()
        .iter()
        .find(|e| e.index == index && !e.ips.is_empty())
        .expect("No default interface [WINDOWS]")
        .clone()
}

/// Get self device IP
#[cached]
pub fn self_ip() -> Ipv4Addr {
    match default_interface()
        .ips
        .iter()
        .find(|a| a.is_ipv4())
        .expect("No IPv4 address for interface")
        .ip()
    {
        IpAddr::V4(ip) => ip,
        _ => panic!(),
    }
}

/// Get self device MAC
#[cached]
pub fn self_mac() -> MacAddr {
    default_interface()
        .mac
        .expect("No MAC address for interface")
}

/// Sets ip forwarding
pub fn ip_forward(enabled: bool) {
    if utils::os::is_windows() {
        utils::windows::run_powershell(&*format!(
            "Set-NetIPInterface -Forwarding {}",
            if enabled { "Enabled" } else { "Disabled" }
        ));
        return;
    }

    let mut f = File::create("/proc/sys/net/ipv4/ip_forward").unwrap();
    f.write(&[if enabled { '1' } else { '0' } as u8]);

    utils::nix::run_shell("iptables -F -t filter");
    utils::nix::run_shell("iptables -P FORWARD ACCEPT");
}

/// Get the gateway IP
#[cached]
pub fn gateway_ip() -> Ipv4Addr {
    if utils::os::is_windows() {
        let ip = utils::windows::run_powershell(
            "Get-NetRoute -DestinationPrefix \"0.0.0.0/0\" \
        | Select-Object -last 1 \
        | Select-Object -ExpandProperty \"NextHop\"",
        )
        .trim()
        .to_string(); // Remove trailing "\r\n"

        return Ipv4Addr::from_str(&*ip).expect("Invalid ip");
    }

    let raw = utils::nix::run_shell("ip route show | grep default | head -n 1");
    let ip = raw.split_whitespace().collect::<Vec<&str>>()[3];
    Ipv4Addr::from_str(&*ip).expect("Invalid ip")
}

/// Get the gateway MAC
#[cached]
pub fn gateway_mac() -> MacAddr {
    if utils::os::is_windows() {
        let raw_mac =
            utils::windows::run_powershell("arp -a | Select-String -Pattern \"192.168.1.1 \"")
                .trim()[gateway_ip().to_string().len()..]
                .trim()[..17]
                .replace("-", ":")
                .to_string();

        return MacAddr::from_str(&*raw_mac).expect("Invalid MAC address");
    }

    let raw = utils::nix::run_shell("ip neigh | grep " + gateway_ip().to_string() + " | head -n 1");
    let mac = raw.split_whitespace().collect::<Vec<&str>>()[4];
    MacAddr::from_str(&*mac).expect("Invalid MAC address")
}
