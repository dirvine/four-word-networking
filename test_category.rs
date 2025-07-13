use std::net::Ipv6Addr;
use std::str::FromStr;

fn categorize_address(ip: &Ipv6Addr) -> &'static str {
    let segments = ip.segments();

    // Check for loopback ::1
    if ip.is_loopback() {
        return "Loopback";
    }

    // Check for unspecified ::
    if ip.is_unspecified() {
        return "Unspecified";
    }

    // Check for link-local fe80::/10
    if segments[0] & 0xFFC0 == 0xFE80 {
        return "LinkLocal";
    }

    // Check for unique local fc00::/7
    if segments[0] & 0xFE00 == 0xFC00 {
        return "UniqueLocal";
    }

    // Check for documentation 2001:db8::/32
    if segments[0] == 0x2001 && segments[1] == 0x0DB8 {
        return "Documentation";
    }

    // Check for global unicast 2000::/3
    if segments[0] & 0xE000 == 0x2000 {
        return "GlobalUnicast";
    }

    // Everything else (multicast, etc.)
    "Special"
}

fn main() {
    let ip = Ipv6Addr::from_str("2001:db8:85a3::8a2e:370:7334").unwrap();
    let segments = ip.segments();
    
    println!("Address: {}", ip);
    println!("Segments: {:x?}", segments);
    println!("segments[0] = 0x{:x} (should be 0x2001)", segments[0]);
    println!("segments[1] = 0x{:x} (should be 0xdb8)", segments[1]);
    println!("segments[0] == 0x2001: {}", segments[0] == 0x2001);
    println!("segments[1] == 0x0DB8: {}", segments[1] == 0x0DB8);
    println!("Category: {}", categorize_address(&ip));
}