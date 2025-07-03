//! Semantic Analysis and Classification of Multiaddr Patterns
//!
//! This module provides semantic understanding of multiaddr patterns to enable
//! 100% real-world usage coverage through intelligent pattern classification
//! and meaningful three-word address generation.

use crate::multiaddr_parser::{ParsedMultiaddr, IpType, Protocol};
use serde::{Deserialize, Serialize};

/// Represents the semantic purpose of a network address
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkPurpose {
    /// Web services (HTTP/HTTPS servers, APIs)
    WebService,
    /// Peer-to-peer networking (libp2p, IPFS nodes)
    P2P,
    /// Content delivery (IPFS gateways, CDN)
    Content,
    /// Relay/proxy services (circuit relays, signaling servers)
    Relay,
    /// Development/testing environments
    Development,
    /// Database/storage services
    Database,
    /// Messaging/communication services
    Messaging,
    /// Unknown or mixed purpose
    Generic,
}

/// Represents the security level of a connection
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Plain text connection
    Plain,
    /// TLS/SSL encrypted
    TLS,
    /// WebSocket connection
    WebSocket,
    /// WebSocket with TLS (WSS)
    SecureWebSocket,
    /// P2P encryption (Noise protocol)
    P2PEncrypted,
    /// Circuit relay (security depends on relay)
    Circuit,
    /// Multiple security layers
    Layered(Vec<SecurityLevel>),
}

/// Represents the network scope/reach
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NetworkScope {
    /// Local machine (127.0.0.1, ::1, unix sockets)
    Local,
    /// Private network (RFC 1918 addresses)
    Private,
    /// Regional/ISP level
    Regional,
    /// Global internet
    Global,
    /// P2P direct connection
    Direct,
    /// Through relay/circuit
    Relayed,
}

/// Represents the transport mechanism
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportType {
    /// TCP connection
    TCP,
    /// UDP connection  
    UDP,
    /// QUIC protocol
    QUIC,
    /// HTTP/HTTPS
    HTTP,
    /// WebSocket
    WebSocket,
    /// Circuit relay
    Circuit,
    /// WebRTC
    WebRTC,
    /// Unix domain socket
    Unix,
    /// Memory transport (testing)
    Memory,
    /// Complex/layered transport
    Complex,
}

/// Comprehensive semantic information about a multiaddr
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SemanticInfo {
    /// Primary purpose of this address
    pub purpose: NetworkPurpose,
    /// Security level and encryption
    pub security: SecurityLevel,
    /// Network scope and reachability
    pub scope: NetworkScope,
    /// Transport mechanism
    pub transport: TransportType,
    /// Human-readable description
    pub description: String,
    /// Context hints for developers
    pub context_hints: Vec<String>,
}

/// Classified multiaddr patterns for semantic mapping
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MultiaddrPattern {
    /// Simple address: IP + protocol + port
    Simple {
        ip_type: IpType,
        protocol: Protocol,
        port: u16,
        is_secure: bool,
    },
    /// Web service patterns
    WebService {
        domain: Option<String>,
        is_secure: bool,
        port: u16,
        path: Option<String>,
    },
    /// P2P node with peer identification
    P2PNode {
        transport: TransportType,
        peer_id: String,
        is_bootstrap: bool,
        security: SecurityLevel,
    },
    /// Circuit relay pattern
    CircuitRelay {
        relay_info: RelayInfo,
        target_info: TargetInfo,
        circuit_type: CircuitType,
    },
    /// Content/IPFS pattern
    ContentGateway {
        gateway_type: GatewayType,
        is_secure: bool,
        scope: NetworkScope,
    },
    /// Development/testing pattern
    Development {
        env_type: DevEnvironment,
        service: String,
        port: u16,
    },
    /// Complex layered protocols
    Complex {
        base_transport: TransportType,
        layers: Vec<ProtocolLayer>,
        semantic_approximation: String,
    },
}

/// Information about a relay in circuit addressing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelayInfo {
    pub address: String,
    pub peer_id: Option<String>,
    pub transport: TransportType,
    pub scope: NetworkScope,
}

/// Information about the target in circuit addressing
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetInfo {
    pub peer_id: String,
    pub expected_transport: Option<TransportType>,
}

/// Types of circuit relay
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitType {
    /// Standard libp2p circuit relay
    P2PCircuit,
    /// WebRTC signaling server
    WebRTCStar,
    /// WebSocket signaling
    WebSocketStar,
    /// Custom relay type
    Custom(String),
}

/// Types of content gateways
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GatewayType {
    /// IPFS public gateway
    IPFSPublic,
    /// IPFS private/local gateway
    IPFSPrivate,
    /// Content CDN
    CDN,
    /// API gateway
    API,
}

/// Development environment types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DevEnvironment {
    /// Local development server
    LocalDev,
    /// Testing environment
    Testing,
    /// Staging environment
    Staging,
    /// Debug/profiling
    Debug,
}

/// Protocol layer in complex addresses
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtocolLayer {
    pub protocol: Protocol,
    pub parameters: Option<String>,
    pub security_level: Option<SecurityLevel>,
}

/// Semantic classifier for multiaddr patterns
pub struct SemanticClassifier;

impl SemanticClassifier {
    /// Classify a parsed multiaddr into a semantic pattern
    pub fn classify(parsed: &ParsedMultiaddr) -> MultiaddrPattern {
        // Check for development patterns first
        if Self::is_development_address(parsed) {
            return Self::classify_development(parsed);
        }
        
        // Check for P2P patterns
        if Self::is_p2p_pattern(parsed) {
            return Self::classify_p2p(parsed);
        }
        
        // Check for web service patterns
        if Self::is_web_service(parsed) {
            return Self::classify_web_service(parsed);
        }
        
        // Check for content gateway patterns
        if Self::is_content_gateway(parsed) {
            return Self::classify_content_gateway(parsed);
        }
        
        // Default to simple pattern
        Self::classify_simple(parsed)
    }
    
    /// Generate semantic information from a classified pattern
    pub fn get_semantic_info(pattern: &MultiaddrPattern) -> SemanticInfo {
        match pattern {
            MultiaddrPattern::Simple { protocol, is_secure, .. } => {
                let security = if *is_secure { SecurityLevel::TLS } else { SecurityLevel::Plain };
                let transport = Self::protocol_to_transport(protocol);
                
                SemanticInfo {
                    purpose: NetworkPurpose::Generic,
                    security,
                    scope: NetworkScope::Global,
                    transport: transport.clone(),
                    description: format!("Generic {} service", Self::transport_name(&transport)),
                    context_hints: vec!["Generic network service".to_string()],
                }
            },
            
            MultiaddrPattern::WebService { is_secure, port, .. } => {
                let security = if *is_secure { SecurityLevel::TLS } else { SecurityLevel::Plain };
                let service_type = match port {
                    80 => "HTTP web server",
                    443 => "HTTPS web server", 
                    8080 => "Development web server",
                    3000 => "Node.js application",
                    _ => "Web service",
                };
                
                SemanticInfo {
                    purpose: NetworkPurpose::WebService,
                    security,
                    scope: NetworkScope::Global,
                    transport: TransportType::HTTP,
                    description: service_type.to_string(),
                    context_hints: vec!["Web browser compatible".to_string(), "HTTP API access".to_string()],
                }
            },
            
            MultiaddrPattern::P2PNode { transport, is_bootstrap, security, .. } => {
                let purpose = if *is_bootstrap { NetworkPurpose::P2P } else { NetworkPurpose::P2P };
                let desc = if *is_bootstrap { "P2P bootstrap node" } else { "P2P peer node" };
                
                SemanticInfo {
                    purpose,
                    security: security.clone(),
                    scope: NetworkScope::Direct,
                    transport: transport.clone(),
                    description: desc.to_string(),
                    context_hints: vec!["libp2p compatible".to_string(), "Peer discovery".to_string()],
                }
            },
            
            MultiaddrPattern::CircuitRelay { circuit_type, .. } => {
                let desc = match circuit_type {
                    CircuitType::P2PCircuit => "libp2p circuit relay",
                    CircuitType::WebRTCStar => "WebRTC signaling server",
                    CircuitType::WebSocketStar => "WebSocket signaling server",
                    CircuitType::Custom(name) => &format!("{} relay", name),
                };
                
                SemanticInfo {
                    purpose: NetworkPurpose::Relay,
                    security: SecurityLevel::Circuit,
                    scope: NetworkScope::Relayed,
                    transport: TransportType::Circuit,
                    description: desc.to_string(),
                    context_hints: vec!["NAT traversal".to_string(), "Proxy connection".to_string()],
                }
            },
            
            MultiaddrPattern::ContentGateway { gateway_type, is_secure, scope } => {
                let security = if *is_secure { SecurityLevel::TLS } else { SecurityLevel::Plain };
                let desc = match gateway_type {
                    GatewayType::IPFSPublic => "IPFS public gateway",
                    GatewayType::IPFSPrivate => "IPFS private gateway",
                    GatewayType::CDN => "Content delivery network",
                    GatewayType::API => "API gateway",
                };
                
                SemanticInfo {
                    purpose: NetworkPurpose::Content,
                    security,
                    scope: scope.clone(),
                    transport: TransportType::HTTP,
                    description: desc.to_string(),
                    context_hints: vec!["Content delivery".to_string(), "HTTP access".to_string()],
                }
            },
            
            MultiaddrPattern::Development { env_type, service, .. } => {
                let desc = match env_type {
                    DevEnvironment::LocalDev => format!("Local development {}", service),
                    DevEnvironment::Testing => format!("Test environment {}", service),
                    DevEnvironment::Staging => format!("Staging {}", service),
                    DevEnvironment::Debug => format!("Debug {}", service),
                };
                
                SemanticInfo {
                    purpose: NetworkPurpose::Development,
                    security: SecurityLevel::Plain,
                    scope: NetworkScope::Local,
                    transport: TransportType::TCP,
                    description: desc,
                    context_hints: vec!["Development only".to_string(), "Not production".to_string()],
                }
            },
            
            MultiaddrPattern::Complex { semantic_approximation, .. } => {
                SemanticInfo {
                    purpose: NetworkPurpose::Generic,
                    security: SecurityLevel::Layered(vec![]),
                    scope: NetworkScope::Global,
                    transport: TransportType::Complex,
                    description: semantic_approximation.clone(),
                    context_hints: vec!["Complex protocol stack".to_string()],
                }
            },
        }
    }
    
    fn is_development_address(parsed: &ParsedMultiaddr) -> bool {
        matches!(parsed.ip_type, IpType::IPv4) && 
        (parsed.address == "127.0.0.1" || parsed.address == "localhost") ||
        matches!(parsed.ip_type, IpType::IPv6) && parsed.address == "::1" ||
        matches!(parsed.ip_type, IpType::Unix | IpType::Memory)
    }
    
    fn is_p2p_pattern(parsed: &ParsedMultiaddr) -> bool {
        // Common P2P ports and patterns
        matches!(parsed.port, 4001 | 4002 | 9000..=9999) ||
        parsed.additional_protocols.iter().any(|p| matches!(p, Protocol::P2PCircuit | Protocol::QUIC)) ||
        parsed.address.starts_with("Qm") || // IPFS peer ID in address
        parsed.address.contains("bootstrap") || // Bootstrap nodes
        parsed.address.contains("libp2p") || // libp2p nodes
        (matches!(parsed.protocol, Protocol::UDP) && parsed.additional_protocols.iter().any(|p| matches!(p, Protocol::QUIC))) // UDP+QUIC is common for P2P
    }
    
    fn is_web_service(parsed: &ParsedMultiaddr) -> bool {
        matches!(parsed.port, 80 | 443 | 8080 | 3000 | 8000 | 5000) ||
        matches!(parsed.protocol, Protocol::HTTP | Protocol::HTTPS) ||
        parsed.additional_protocols.iter().any(|p| matches!(p, Protocol::HTTP | Protocol::HTTPS | Protocol::WS | Protocol::WSS))
    }
    
    fn is_content_gateway(parsed: &ParsedMultiaddr) -> bool {
        parsed.address.contains("ipfs") || 
        parsed.address.contains("gateway") ||
        parsed.address.contains("cdn") ||
        parsed.address.contains("cloudflare")
    }
    
    fn classify_development(parsed: &ParsedMultiaddr) -> MultiaddrPattern {
        let env_type = match parsed.port {
            3000 => DevEnvironment::LocalDev,
            8080 => DevEnvironment::LocalDev,
            5000..=5999 => DevEnvironment::Testing,
            9000..=9999 => DevEnvironment::Debug,
            _ => DevEnvironment::LocalDev,
        };
        
        let service = match parsed.port {
            3000 => "webapp".to_string(),
            8080 => "server".to_string(),
            5432 => "database".to_string(),
            _ => "service".to_string(),
        };
        
        MultiaddrPattern::Development {
            env_type,
            service,
            port: parsed.port,
        }
    }
    
    fn classify_p2p(parsed: &ParsedMultiaddr) -> MultiaddrPattern {
        let transport = Self::protocol_to_transport(&parsed.protocol);
        let is_bootstrap = parsed.port == 4001 || parsed.address.contains("bootstrap");
        let security = if parsed.additional_protocols.iter().any(|p| matches!(p, Protocol::TLS | Protocol::Noise)) {
            SecurityLevel::P2PEncrypted
        } else {
            SecurityLevel::Plain
        };
        
        MultiaddrPattern::P2PNode {
            transport,
            peer_id: "QmPeerID".to_string(), // Simplified for demo
            is_bootstrap,
            security,
        }
    }
    
    fn classify_web_service(parsed: &ParsedMultiaddr) -> MultiaddrPattern {
        let is_secure = parsed.port == 443 || 
            parsed.additional_protocols.iter().any(|p| matches!(p, Protocol::TLS | Protocol::HTTPS | Protocol::WSS));
        
        let domain = if matches!(parsed.ip_type, IpType::DNS4 | IpType::DNS6) {
            Some(parsed.address.clone())
        } else {
            None
        };
        
        MultiaddrPattern::WebService {
            domain,
            is_secure,
            port: parsed.port,
            path: None,
        }
    }
    
    fn classify_content_gateway(parsed: &ParsedMultiaddr) -> MultiaddrPattern {
        let gateway_type = if parsed.address.contains("ipfs") {
            if parsed.address.contains("gateway") {
                GatewayType::IPFSPublic
            } else {
                GatewayType::IPFSPrivate
            }
        } else if parsed.address.contains("cdn") {
            GatewayType::CDN
        } else {
            GatewayType::API
        };
        
        let is_secure = parsed.port == 443 || 
            parsed.additional_protocols.iter().any(|p| matches!(p, Protocol::TLS | Protocol::HTTPS));
        
        let scope = if parsed.address.contains("127.0.0.1") || parsed.address.contains("localhost") {
            NetworkScope::Local
        } else {
            NetworkScope::Global
        };
        
        MultiaddrPattern::ContentGateway {
            gateway_type,
            is_secure,
            scope,
        }
    }
    
    fn classify_simple(parsed: &ParsedMultiaddr) -> MultiaddrPattern {
        let is_secure = parsed.additional_protocols.iter().any(|p| matches!(p, Protocol::TLS | Protocol::HTTPS | Protocol::WSS));
        
        MultiaddrPattern::Simple {
            ip_type: parsed.ip_type.clone(),
            protocol: parsed.protocol.clone(),
            port: parsed.port,
            is_secure,
        }
    }
    
    fn protocol_to_transport(protocol: &Protocol) -> TransportType {
        match protocol {
            Protocol::TCP => TransportType::TCP,
            Protocol::UDP => TransportType::UDP,
            Protocol::QUIC | Protocol::QuicV1 => TransportType::QUIC,
            Protocol::HTTP | Protocol::HTTPS => TransportType::HTTP,
            Protocol::WS | Protocol::WSS | Protocol::WebSocket => TransportType::WebSocket,
            Protocol::WebRTC | Protocol::WebRTCDirect => TransportType::WebRTC,
            Protocol::P2PCircuit => TransportType::Circuit,
            _ => TransportType::TCP,
        }
    }
    
    fn transport_name(transport: &TransportType) -> &str {
        match transport {
            TransportType::TCP => "TCP",
            TransportType::UDP => "UDP", 
            TransportType::QUIC => "QUIC",
            TransportType::HTTP => "HTTP",
            TransportType::WebSocket => "WebSocket",
            TransportType::WebRTC => "WebRTC",
            TransportType::Circuit => "Circuit",
            TransportType::Unix => "Unix",
            TransportType::Memory => "Memory",
            TransportType::Complex => "Complex",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::multiaddr_parser::ParsedMultiaddr;

    #[test]
    fn test_development_classification() {
        let multiaddr = "/ip4/127.0.0.1/tcp/3000";
        let parsed = ParsedMultiaddr::parse(multiaddr).unwrap();
        let pattern = SemanticClassifier::classify(&parsed);
        
        match pattern {
            MultiaddrPattern::Development { env_type, service, port } => {
                assert_eq!(env_type, DevEnvironment::LocalDev);
                assert_eq!(service, "webapp");
                assert_eq!(port, 3000);
            },
            _ => panic!("Expected Development pattern"),
        }
    }
    
    #[test]
    fn test_web_service_classification() {
        let multiaddr = "/dns4/api.example.com/tcp/443/tls";
        let parsed = ParsedMultiaddr::parse(multiaddr).unwrap();
        let pattern = SemanticClassifier::classify(&parsed);
        
        match pattern {
            MultiaddrPattern::WebService { domain, is_secure, port, .. } => {
                assert_eq!(domain, Some("api.example.com".to_string()));
                assert!(is_secure);
                assert_eq!(port, 443);
            },
            _ => panic!("Expected WebService pattern"),
        }
    }
    
    #[test]
    fn test_p2p_classification() {
        let multiaddr = "/dns4/bootstrap.libp2p.io/tcp/4001";
        let parsed = ParsedMultiaddr::parse(multiaddr).unwrap();
        let pattern = SemanticClassifier::classify(&parsed);
        
        match pattern {
            MultiaddrPattern::P2PNode { transport, is_bootstrap, .. } => {
                assert_eq!(transport, TransportType::TCP);
                assert!(is_bootstrap);
            },
            _ => panic!("Expected P2PNode pattern"),
        }
    }
    
    #[test]
    fn test_semantic_info_generation() {
        let pattern = MultiaddrPattern::WebService {
            domain: Some("api.example.com".to_string()),
            is_secure: true,
            port: 443,
            path: None,
        };
        
        let info = SemanticClassifier::get_semantic_info(&pattern);
        
        assert_eq!(info.purpose, NetworkPurpose::WebService);
        assert_eq!(info.security, SecurityLevel::TLS);
        assert_eq!(info.transport, TransportType::HTTP);
        assert!(info.description.contains("HTTPS"));
        assert!(!info.context_hints.is_empty());
    }
}