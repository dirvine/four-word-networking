//! Three-Word Networking CLI
//!
//! Command-line interface for converting network multiaddresses to human-friendly
//! three-word addresses and vice versa.

use clap::{Parser, Subcommand};
use three_word_networking::{WordEncoder, ThreeWordAddress, AddressSpace, Result};

#[derive(Parser)]
#[command(name = "three-word-networking")]
#[command(about = "Convert network addresses to human-friendly three-word combinations")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Convert multiaddr to three-word address
    Encode {
        /// The multiaddress to convert (e.g., "/ip6/2001:db8::1/udp/9000/quic")
        multiaddr: String,
        
        /// Use base format only (no numeric suffix)
        #[arg(short, long)]
        base: bool,
    },
    
    /// Convert three-word address back to multiaddr (requires registry)
    Decode {
        /// The three-word address to convert (e.g., "ocean.thunder.falcon")
        words: String,
    },
    
    /// Validate that a three-word address is well-formed
    Validate {
        /// The three-word address to validate
        words: String,
    },
    
    /// Show information about the address space
    Info,
    
    /// Generate random examples
    Examples {
        /// Number of examples to generate
        #[arg(short, long, default_value = "5")]
        count: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let encoder = WordEncoder::new();

    match cli.command {
        Commands::Encode { multiaddr, base } => {
            match if base {
                encoder.encode_multiaddr_string_base(&multiaddr)
            } else {
                encoder.encode_multiaddr_string(&multiaddr)
            } {
                Ok(words) => {
                    println!("Multiaddr: {}", multiaddr);
                    println!("Three-word: {}", words);
                    if words.is_extended() {
                        println!("Base format: {}", words.base_address());
                    }
                    println!("Valid: ✓");
                }
                Err(e) => {
                    eprintln!("Error encoding multiaddr: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Decode { words } => {
            match ThreeWordAddress::from_string(&words) {
                Ok(addr) => {
                    match encoder.decode_to_multiaddr_string(&addr) {
                        Ok(multiaddr) => {
                            println!("Three-word: {}", words);
                            println!("Multiaddr: {}", multiaddr);
                        }
                        Err(e) => {
                            eprintln!("Error decoding to multiaddr: {}", e);
                            eprintln!("Note: Decoding requires a registry lookup which is not yet implemented.");
                            eprintln!("The three-word address is valid: {}", addr);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error parsing three-word address: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Validate { words } => {
            match ThreeWordAddress::from_string(&words) {
                Ok(addr) => {
                    match addr.validate(&encoder) {
                        Ok(()) => {
                            println!("Three-word address: {}", words);
                            println!("Format: ✓ Valid");
                            println!("Dictionary: ✓ All words found");
                            if addr.is_extended() {
                                println!("Type: Extended (with numeric suffix)");
                                println!("Base: {}", addr.base_address());
                                println!("Suffix: {}", addr.suffix.unwrap());
                            } else {
                                println!("Type: Base format");
                            }
                        }
                        Err(e) => {
                            println!("Three-word address: {}", words);
                            println!("Format: ✓ Valid");
                            println!("Dictionary: ✗ {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Three-word address: {}", words);
                    println!("Format: ✗ {}", e);
                }
            }
        }
        
        Commands::Info => {
            println!("Three-Word Networking Address Space");
            println!("===================================");
            println!();
            println!("Total Address Space: {}", AddressSpace::description());
            println!("Base Combinations: {}", AddressSpace::base_combinations());
            println!("Total Combinations: {}", AddressSpace::total_combinations());
            println!();
            println!("Format Examples:");
            println!("  Base: forest.lightning.compass");
            println!("  Extended: forest.lightning.compass.1847");
            println!();
            println!("Dictionary Structure:");
            println!("  Position 1 (Context): Geographic, network, scale contexts");
            println!("  Position 2 (Quality): Performance, purpose, status descriptors");
            println!("  Position 3 (Identity): Nature, objects, abstract concepts");
            println!();
            println!("Features:");
            println!("  ✓ Human-readable and memorable");
            println!("  ✓ Voice-friendly for phone/voice sharing");
            println!("  ✓ Error-resistant compared to long addresses");
            println!("  ✓ Deterministic (same multiaddr = same words)");
            println!("  ✓ Massive scale (quintillions of addresses)");
            println!("  ✓ Universal (works with any multiaddr format)");
        }
        
        Commands::Examples { count } => {
            println!("Three-Word Networking Examples");
            println!("=============================");
            println!();
            
            let example_multiaddrs = vec![
                "/ip6/2001:db8::1/udp/9000/quic",
                "/ip4/192.168.1.1/tcp/8080",
                "/ip6/::1/tcp/22",
                "/ip4/10.0.0.1/udp/5000/quic",
                "/ip6/fe80::1/tcp/443",
                "/ip4/203.0.113.1/tcp/443/tls",
                "/dns4/example.com/tcp/80",
                "/dns6/ipv6.google.com/tcp/443",
                "/ip4/127.0.0.1/tcp/8080/ws",
                "/ip6/2606:4700:4700::1111/udp/443/quic",
            ];
            
            let examples_to_show = std::cmp::min(count, example_multiaddrs.len());
            
            for (i, multiaddr) in example_multiaddrs.iter().take(examples_to_show).enumerate() {
                match encoder.encode_multiaddr_string(multiaddr) {
                    Ok(words) => {
                        println!("{}. Multiaddr: {}", i + 1, multiaddr);
                        println!("   Three-word: {}", words);
                        if words.is_extended() {
                            println!("   Base format: {}", words.base_address());
                        }
                        println!("   Voice-friendly: \"Connect to {}\"", words.to_string().replace('.', " "));
                        println!();
                    }
                    Err(e) => {
                        eprintln!("   Error: {}", e);
                        println!();
                    }
                }
            }
            
            println!("Use Cases:");
            println!("• Share P2P addresses over phone: \"Connect to ocean thunder falcon\"");
            println!("• QR codes with memorable text backup");
            println!("• User-friendly network configuration");
            println!("• Simplified peer discovery");
            println!("• Voice-activated network connections");
        }
    }

    Ok(())
}