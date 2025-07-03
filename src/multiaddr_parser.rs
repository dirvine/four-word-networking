//! Multiaddr Parser Module
//!
//! Parses multiaddr strings into structured components that can be deterministically
//! mapped to three-word addresses without requiring external registry lookups.

use crate::error::{ThreeWordError, Result};
use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// Represents different IP address types found in multiaddrs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IpType {
    IPv4,
    IPv6,
    DNS4,
    DNS6,
    DNS,
    Unix,
    P2P,
    Onion,
    Onion3,
    Garlic64,
    Garlic32,
    Memory,
    CIDv1,
    SCTP,
    UTP,
    /// Unknown IP type with string name
    Unknown(String),
}

/// Represents different transport protocols found in multiaddrs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Protocol {
    TCP,
    UDP,
    DCCP,
    SCTP,
    UTP,
    QUIC,
    QuicV1,
    WS,
    WSS,
    WebSocket,
    TLS,
    Noise,
    Yamux,
    MPLEX,
    HTTP,
    HTTPS,
    HTTPPath,
    P2PCircuit,
    P2PWebSocket,
    P2PWebSocketStar,
    P2PStardust,
    WebRTC,
    WebRTCDirect,
    WebTransport,
    Certhash,
    Plaintextv2,
    /// Unknown protocol with string name and optional port requirement
    Unknown(String, bool), // (protocol_name, has_port)
}

/// Represents a parsed multiaddr with its constituent components
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParsedMultiaddr {
    pub ip_type: IpType,
    pub address: String,
    pub protocol: Protocol,
    pub port: u16,
    pub additional_protocols: Vec<Protocol>,
}

impl ParsedMultiaddr {
    /// Parse a multiaddr string into its components
    pub fn parse(multiaddr: &str) -> Result<Self> {
        if !multiaddr.starts_with('/') {
            return Err(ThreeWordError::InvalidMultiaddr(
                format!("Multiaddr must start with '/', got: {}", multiaddr)
            ));
        }

        let parts: Vec<&str> = multiaddr.split('/').filter(|s| !s.is_empty()).collect();
        
        if parts.len() < 3 {
            return Err(ThreeWordError::InvalidMultiaddr(
                format!("Multiaddr must have at least 3 parts, got: {}", parts.len())
            ));
        }

        // Parse IP type and address
        let (ip_type, address) = Self::parse_ip_component(&parts[0], &parts[1])?;
        
        // Parse protocol and port
        let (protocol, port) = if parts.len() > 3 {
            Self::parse_protocol_component(&parts[2], &parts[3])?
        } else {
            // Handle protocols without separate port (like /ip4/127.0.0.1/quic)
            Self::parse_protocol_component(&parts[2], "0")?
        };
        
        // Parse additional protocols
        let additional_protocols = if parts.len() > 4 {
            Self::parse_additional_protocols(&parts[4..])?
        } else if parts.len() == 4 && matches!(protocol, Protocol::UDP | Protocol::TCP) {
            // If we have 4 parts and the 3rd is UDP/TCP, the 4th might be an additional protocol
            if parts[3].parse::<u16>().is_err() {
                Self::parse_additional_protocols(&parts[3..])?
            } else {
                vec![]
            }
        } else {
            vec![]
        };

        Ok(ParsedMultiaddr {
            ip_type,
            address,
            protocol,
            port,
            additional_protocols,
        })
    }

    /// Convert parsed components back to multiaddr string
    pub fn to_multiaddr(&self) -> String {
        let mut result = String::new();
        
        // Add IP type and address
        match self.ip_type {
            IpType::IPv4 => result.push_str(&format!("/ip4/{}", self.address)),
            IpType::IPv6 => result.push_str(&format!("/ip6/{}", self.address)),
            IpType::DNS4 => result.push_str(&format!("/dns4/{}", self.address)),
            IpType::DNS6 => result.push_str(&format!("/dns6/{}", self.address)),
            IpType::DNS => result.push_str(&format!("/dns/{}", self.address)),
            IpType::Unix => result.push_str(&format!("/unix/{}", self.address)),
            IpType::P2P => result.push_str(&format!("/p2p/{}", self.address)),
            IpType::Onion => result.push_str(&format!("/onion/{}", self.address)),
            IpType::Onion3 => result.push_str(&format!("/onion3/{}", self.address)),
            IpType::Garlic64 => result.push_str(&format!("/garlic64/{}", self.address)),
            IpType::Garlic32 => result.push_str(&format!("/garlic32/{}", self.address)),
            IpType::Memory => result.push_str(&format!("/memory/{}", self.address)),
            IpType::CIDv1 => result.push_str(&format!("/cid/{}", self.address)),
            IpType::SCTP => result.push_str(&format!("/sctp/{}", self.address)),
            IpType::UTP => result.push_str(&format!("/utp/{}", self.address)),
            IpType::Unknown(ref name) => result.push_str(&format!("/{}/{}", name, self.address)),
        }
        
        // Add protocol and port
        match self.protocol {
            Protocol::TCP => result.push_str(&format!("/tcp/{}", self.port)),
            Protocol::UDP => result.push_str(&format!("/udp/{}", self.port)),
            Protocol::DCCP => result.push_str(&format!("/dccp/{}", self.port)),
            Protocol::SCTP => result.push_str(&format!("/sctp/{}", self.port)),
            Protocol::UTP => result.push_str(&format!("/utp/{}", self.port)),
            Protocol::QUIC => result.push_str("/quic"),
            Protocol::QuicV1 => result.push_str("/quic-v1"),
            Protocol::WS => result.push_str("/ws"),
            Protocol::WSS => result.push_str("/wss"),
            Protocol::WebSocket => result.push_str("/websocket"),
            Protocol::TLS => result.push_str("/tls"),
            Protocol::Noise => result.push_str("/noise"),
            Protocol::Yamux => result.push_str("/yamux"),
            Protocol::MPLEX => result.push_str("/mplex"),
            Protocol::HTTP => result.push_str("/http"),
            Protocol::HTTPS => result.push_str("/https"),
            Protocol::HTTPPath => result.push_str("/http-path"),
            Protocol::P2PCircuit => result.push_str("/p2p-circuit"),
            Protocol::P2PWebSocket => result.push_str("/p2p-websocket"),
            Protocol::P2PWebSocketStar => result.push_str("/p2p-websocket-star"),
            Protocol::P2PStardust => result.push_str("/p2p-stardust"),
            Protocol::WebRTC => result.push_str("/webrtc"),
            Protocol::WebRTCDirect => result.push_str("/webrtc-direct"),
            Protocol::WebTransport => result.push_str("/webtransport"),
            Protocol::Certhash => result.push_str("/certhash"),
            Protocol::Plaintextv2 => result.push_str("/plaintextv2"),
            Protocol::Unknown(ref name, has_port) => {
                if has_port {
                    result.push_str(&format!("/{}/{}", name, self.port));
                } else {
                    result.push_str(&format!("/{}", name));
                }
            },
        }
        
        // Add additional protocols
        for protocol in &self.additional_protocols {
            match protocol {
                Protocol::QUIC => result.push_str("/quic"),
                Protocol::WS => result.push_str("/ws"),
                Protocol::WSS => result.push_str("/wss"),
                Protocol::TLS => result.push_str("/tls"),
                Protocol::HTTP => result.push_str("/http"),
                Protocol::HTTPS => result.push_str("/https"),
                Protocol::P2PCircuit => result.push_str("/p2p-circuit"),
                Protocol::WebRTC => result.push_str("/webrtc"),
                Protocol::WebTransport => result.push_str("/webtransport"),
                _ => {} // Skip protocols that don't appear as additional
            }
        }
        
        result
    }

    fn parse_ip_component(ip_type_str: &str, address_str: &str) -> Result<(IpType, String)> {
        let ip_type = match ip_type_str {
            "ip4" => {
                // Validate IPv4 address
                Ipv4Addr::from_str(address_str)
                    .map_err(|e| ThreeWordError::InvalidMultiaddr(
                        format!("Invalid IPv4 address '{}': {}", address_str, e)
                    ))?;
                IpType::IPv4
            }
            "ip6" => {
                // Validate IPv6 address
                Ipv6Addr::from_str(address_str)
                    .map_err(|e| ThreeWordError::InvalidMultiaddr(
                        format!("Invalid IPv6 address '{}': {}", address_str, e)
                    ))?;
                IpType::IPv6
            }
            "dns4" => IpType::DNS4,
            "dns6" => IpType::DNS6,
            "dns" => IpType::DNS,
            "unix" => IpType::Unix,
            "p2p" => IpType::P2P,
            "onion" => IpType::Onion,
            "onion3" => IpType::Onion3,
            "garlic64" => IpType::Garlic64,
            "garlic32" => IpType::Garlic32,
            "memory" => IpType::Memory,
            "cid" => IpType::CIDv1,
            "sctp" => IpType::SCTP,
            "utp" => IpType::UTP,
            _ => {
                // Handle unknown IP types gracefully
                IpType::Unknown(ip_type_str.to_string())
            },
        };

        Ok((ip_type, address_str.to_string()))
    }

    fn parse_protocol_component(protocol_str: &str, port_str: &str) -> Result<(Protocol, u16)> {
        let protocol = match protocol_str {
            "tcp" => Protocol::TCP,
            "udp" => Protocol::UDP,
            "dccp" => Protocol::DCCP,
            "sctp" => Protocol::SCTP,
            "utp" => Protocol::UTP,
            "quic" => return Ok((Protocol::QUIC, 0)),
            "quic-v1" => return Ok((Protocol::QuicV1, 0)),
            "ws" => return Ok((Protocol::WS, 0)),
            "wss" => return Ok((Protocol::WSS, 0)),
            "websocket" => return Ok((Protocol::WebSocket, 0)),
            "tls" => return Ok((Protocol::TLS, 0)),
            "noise" => return Ok((Protocol::Noise, 0)),
            "yamux" => return Ok((Protocol::Yamux, 0)),
            "mplex" => return Ok((Protocol::MPLEX, 0)),
            "http" => return Ok((Protocol::HTTP, 0)),
            "https" => return Ok((Protocol::HTTPS, 0)),
            "http-path" => return Ok((Protocol::HTTPPath, 0)),
            "p2p-circuit" => return Ok((Protocol::P2PCircuit, 0)),
            "p2p-websocket" => return Ok((Protocol::P2PWebSocket, 0)),
            "p2p-websocket-star" => return Ok((Protocol::P2PWebSocketStar, 0)),
            "p2p-stardust" => return Ok((Protocol::P2PStardust, 0)),
            "webrtc" => return Ok((Protocol::WebRTC, 0)),
            "webrtc-direct" => return Ok((Protocol::WebRTCDirect, 0)),
            "webtransport" => return Ok((Protocol::WebTransport, 0)),
            "certhash" => return Ok((Protocol::Certhash, 0)),
            "plaintextv2" => return Ok((Protocol::Plaintextv2, 0)),
            _ => {
                // Handle unknown protocols gracefully - assume they need a port if port_str is provided
                let has_port = !port_str.is_empty() && port_str != "0";
                if has_port {
                    let port = port_str.parse::<u16>()
                        .map_err(|e| ThreeWordError::InvalidMultiaddr(
                            format!("Invalid port '{}' for unknown protocol '{}': {}", port_str, protocol_str, e)
                        ))?;
                    return Ok((Protocol::Unknown(protocol_str.to_string(), true), port));
                } else {
                    return Ok((Protocol::Unknown(protocol_str.to_string(), false), 0));
                }
            },
        };

        let port = port_str.parse::<u16>()
            .map_err(|e| ThreeWordError::InvalidMultiaddr(
                format!("Invalid port '{}': {}", port_str, e)
            ))?;

        Ok((protocol, port))
    }

    fn parse_additional_protocols(parts: &[&str]) -> Result<Vec<Protocol>> {
        let mut protocols = Vec::new();
        
        for part in parts {
            let protocol = match *part {
                "quic" => Protocol::QUIC,
                "ws" => Protocol::WS,
                "wss" => Protocol::WSS,
                "tls" => Protocol::TLS,
                "http" => Protocol::HTTP,
                "https" => Protocol::HTTPS,
                "p2p-circuit" => Protocol::P2PCircuit,
                "webrtc" => Protocol::WebRTC,
                "webtransport" => Protocol::WebTransport,
                _ => return Err(ThreeWordError::InvalidMultiaddr(
                    format!("Unknown additional protocol: {}", part)
                )),
            };
            protocols.push(protocol);
        }
        
        Ok(protocols)
    }

    /// Get a compact hash representation of the address for compression
    pub fn address_hash(&self) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        self.address.hash(&mut hasher);
        hasher.finish()
    }

    /// Get the primary protocol (first non-transport protocol)
    pub fn primary_protocol(&self) -> Protocol {
        if !self.additional_protocols.is_empty() {
            self.additional_protocols[0].clone()
        } else {
            self.protocol.clone()
        }
    }
}

impl IpType {
    /// Convert IP type to string representation
    pub fn to_string(&self) -> String {
        match self {
            IpType::IPv4 => "ipv4".to_string(),
            IpType::IPv6 => "ipv6".to_string(),
            IpType::DNS4 => "dns4".to_string(),
            IpType::DNS6 => "dns6".to_string(),
            IpType::DNS => "dns".to_string(),
            IpType::Unix => "unix".to_string(),
            IpType::P2P => "p2p".to_string(),
            IpType::Onion => "onion".to_string(),
            IpType::Onion3 => "onion3".to_string(),
            IpType::Garlic64 => "garlic64".to_string(),
            IpType::Garlic32 => "garlic32".to_string(),
            IpType::Memory => "memory".to_string(),
            IpType::CIDv1 => "cid".to_string(),
            IpType::SCTP => "sctp".to_string(),
            IpType::UTP => "utp".to_string(),
            IpType::Unknown(name) => name.clone(),
        }
    }
    
    /// Get word index for this IP type (for context words)
    pub fn word_index(&self) -> usize {
        match self {
            IpType::IPv4 => 0,
            IpType::IPv6 => 1,
            IpType::DNS4 => 2,
            IpType::DNS6 => 3,
            IpType::DNS => 4,
            IpType::Unix => 5,
            IpType::P2P => 6,
            IpType::Onion => 7,
            IpType::Onion3 => 8,
            IpType::Garlic64 => 9,
            IpType::Garlic32 => 10,
            IpType::Memory => 11,
            IpType::CIDv1 => 12,
            IpType::SCTP => 13,
            IpType::UTP => 14,
            IpType::Unknown(ref name) => {
                // Use hash of the unknown type name for consistent mapping
                15 + (name.len() % 100)
            },
        }
    }

    /// Get IP type from word index
    pub fn from_word_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(IpType::IPv4),
            1 => Some(IpType::IPv6),
            2 => Some(IpType::DNS4),
            3 => Some(IpType::DNS6),
            4 => Some(IpType::DNS),
            5 => Some(IpType::Unix),
            6 => Some(IpType::P2P),
            7 => Some(IpType::Onion),
            8 => Some(IpType::Onion3),
            9 => Some(IpType::Garlic64),
            10 => Some(IpType::Garlic32),
            11 => Some(IpType::Memory),
            12 => Some(IpType::CIDv1),
            13 => Some(IpType::SCTP),
            14 => Some(IpType::UTP),
            _ => None,
        }
    }
}

impl Protocol {
    /// Get word index for this protocol (for quality words)
    pub fn word_index(&self) -> usize {
        match self {
            Protocol::TCP => 0,
            Protocol::UDP => 1,
            Protocol::DCCP => 2,
            Protocol::SCTP => 3,
            Protocol::UTP => 4,
            Protocol::QUIC => 5,
            Protocol::QuicV1 => 6,
            Protocol::WS => 7,
            Protocol::WSS => 8,
            Protocol::WebSocket => 9,
            Protocol::TLS => 10,
            Protocol::Noise => 11,
            Protocol::Yamux => 12,
            Protocol::MPLEX => 13,
            Protocol::HTTP => 14,
            Protocol::HTTPS => 15,
            Protocol::HTTPPath => 16,
            Protocol::P2PCircuit => 17,
            Protocol::P2PWebSocket => 18,
            Protocol::P2PWebSocketStar => 19,
            Protocol::P2PStardust => 20,
            Protocol::WebRTC => 21,
            Protocol::WebRTCDirect => 22,
            Protocol::WebTransport => 23,
            Protocol::Certhash => 24,
            Protocol::Plaintextv2 => 25,
            Protocol::Unknown(ref name, _) => {
                // Use hash of the unknown protocol name for consistent mapping
                26 + (name.len() % 100)
            },
        }
    }

    /// Get protocol from word index
    pub fn from_word_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Protocol::TCP),
            1 => Some(Protocol::UDP),
            2 => Some(Protocol::DCCP),
            3 => Some(Protocol::SCTP),
            4 => Some(Protocol::UTP),
            5 => Some(Protocol::QUIC),
            6 => Some(Protocol::QuicV1),
            7 => Some(Protocol::WS),
            8 => Some(Protocol::WSS),
            9 => Some(Protocol::WebSocket),
            10 => Some(Protocol::TLS),
            11 => Some(Protocol::Noise),
            12 => Some(Protocol::Yamux),
            13 => Some(Protocol::MPLEX),
            14 => Some(Protocol::HTTP),
            15 => Some(Protocol::HTTPS),
            16 => Some(Protocol::HTTPPath),
            17 => Some(Protocol::P2PCircuit),
            18 => Some(Protocol::P2PWebSocket),
            19 => Some(Protocol::P2PWebSocketStar),
            20 => Some(Protocol::P2PStardust),
            21 => Some(Protocol::WebRTC),
            22 => Some(Protocol::WebRTCDirect),
            23 => Some(Protocol::WebTransport),
            24 => Some(Protocol::Certhash),
            25 => Some(Protocol::Plaintextv2),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ipv4_tcp() {
        let multiaddr = "/ip4/192.168.1.1/tcp/8080";
        let parsed = ParsedMultiaddr::parse(multiaddr).unwrap();
        
        assert_eq!(parsed.ip_type, IpType::IPv4);
        assert_eq!(parsed.address, "192.168.1.1");
        assert_eq!(parsed.protocol, Protocol::TCP);
        assert_eq!(parsed.port, 8080);
        assert!(parsed.additional_protocols.is_empty());
        
        // Test round trip
        assert_eq!(parsed.to_multiaddr(), multiaddr);
    }

    #[test]
    fn test_parse_ipv6_udp_quic() {
        let multiaddr = "/ip6/2001:db8::1/udp/9000/quic";
        let parsed = ParsedMultiaddr::parse(multiaddr).unwrap();
        
        assert_eq!(parsed.ip_type, IpType::IPv6);
        assert_eq!(parsed.address, "2001:db8::1");
        assert_eq!(parsed.protocol, Protocol::UDP);
        assert_eq!(parsed.port, 9000);
        assert_eq!(parsed.additional_protocols, vec![Protocol::QUIC]);
        
        // Test round trip
        assert_eq!(parsed.to_multiaddr(), multiaddr);
    }

    #[test]
    fn test_parse_dns4_tcp() {
        let multiaddr = "/dns4/example.com/tcp/80";
        let parsed = ParsedMultiaddr::parse(multiaddr).unwrap();
        
        assert_eq!(parsed.ip_type, IpType::DNS4);
        assert_eq!(parsed.address, "example.com");
        assert_eq!(parsed.protocol, Protocol::TCP);
        assert_eq!(parsed.port, 80);
        
        // Test round trip
        assert_eq!(parsed.to_multiaddr(), multiaddr);
    }

    #[test]
    fn test_parse_invalid_multiaddr() {
        // Missing leading slash
        assert!(ParsedMultiaddr::parse("ip4/127.0.0.1/tcp/8080").is_err());
        
        // Too few parts
        assert!(ParsedMultiaddr::parse("/ip4").is_err());
        
        // Invalid IPv4
        assert!(ParsedMultiaddr::parse("/ip4/invalid/tcp/8080").is_err());
        
        // Invalid port
        assert!(ParsedMultiaddr::parse("/ip4/127.0.0.1/tcp/invalid").is_err());
    }

    #[test]
    fn test_ip_type_word_indices() {
        assert_eq!(IpType::IPv4.word_index(), 0);
        assert_eq!(IpType::IPv6.word_index(), 1);
        assert_eq!(IpType::DNS4.word_index(), 2);
        
        assert_eq!(IpType::from_word_index(0), Some(IpType::IPv4));
        assert_eq!(IpType::from_word_index(1), Some(IpType::IPv6));
        assert_eq!(IpType::from_word_index(999), None);
    }

    #[test]
    fn test_protocol_word_indices() {
        assert_eq!(Protocol::TCP.word_index(), 0);
        assert_eq!(Protocol::UDP.word_index(), 1);
        assert_eq!(Protocol::QUIC.word_index(), 5);
        
        assert_eq!(Protocol::from_word_index(0), Some(Protocol::TCP));
        assert_eq!(Protocol::from_word_index(1), Some(Protocol::UDP));
        assert_eq!(Protocol::from_word_index(999), None);
    }
    
    #[test]
    fn test_unknown_protocol_parsing() {
        let multiaddr = "/ip4/192.168.1.1/future-protocol/1234";
        let parsed = ParsedMultiaddr::parse(multiaddr).unwrap();
        
        assert_eq!(parsed.ip_type, IpType::IPv4);
        assert_eq!(parsed.address, "192.168.1.1");
        assert_eq!(parsed.protocol, Protocol::Unknown("future-protocol".to_string(), true));
        assert_eq!(parsed.port, 1234);
        
        // Test round trip
        let reconstructed = parsed.to_multiaddr();
        assert_eq!(reconstructed, multiaddr);
    }
    
    #[test]
    fn test_unknown_ip_type_parsing() {
        let multiaddr = "/future-ip/test-address/tcp/8080";
        let parsed = ParsedMultiaddr::parse(multiaddr).unwrap();
        
        assert_eq!(parsed.ip_type, IpType::Unknown("future-ip".to_string()));
        assert_eq!(parsed.address, "test-address");
        assert_eq!(parsed.protocol, Protocol::TCP);
        assert_eq!(parsed.port, 8080);
        
        // Test round trip
        let reconstructed = parsed.to_multiaddr();
        assert_eq!(reconstructed, multiaddr);
    }
    
    #[test]
    fn test_extended_protocol_support() {
        let test_multiaddrs = vec![
            "/onion/example.onion:80/tcp/8080",
            "/onion3/example.onion/tls",
            "/garlic64/garlic-address/noise",
            "/memory/mem-addr/yamux",
            "/cid/QmHash/webrtc-direct",
        ];
        
        for multiaddr in test_multiaddrs {
            let parsed = ParsedMultiaddr::parse(multiaddr).unwrap();
            let _reconstructed = parsed.to_multiaddr();
            
            // Check that we can parse all these formats
            assert!(parsed.ip_type != IpType::IPv4); // Should be something else
            
            println!("✅ Parsed: {} → {:?}", multiaddr, parsed.ip_type);
        }
    }
}