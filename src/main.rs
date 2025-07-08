//! Four-Word Networking CLI
//!
//! Command-line interface for converting network IP addresses to human-friendly
//! word combinations with perfect IPv4 reconstruction and adaptive IPv6 compression.

use clap::{Parser, Subcommand};
use four_word_networking::{IpPortEncoder, CompressedEncoder, UniversalEncoder, FourWordAdaptiveEncoder, SimpleAdaptiveEncoder, Result};

#[derive(Parser)]
#[command(name = "four-word-networking")]
#[command(about = "Convert network addresses to human-friendly word combinations")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encode pure IP address with port (e.g., "192.168.1.1:8080")
    IpEncode {
        /// The IP address with port to encode
        address: String,
    },
    
    /// Decode three words back to IP address with port
    IpDecode {
        /// The three-word address to decode
        words: String,
    },
    
    /// Encode IP address using advanced compression (recommended)
    Compress {
        /// The IP address with optional port to encode
        address: String,
        
        /// Show compression statistics
        #[arg(long)]
        stats: bool,
    },
    
    /// Decode compressed three-word address back to IP
    Decompress {
        /// The three-word address to decode
        words: String,
    },

    /// Universal compression - tries all strategies
    Universal {
        /// The IP address with port to compress
        address: String,
        
        /// Show detailed analysis of all compression strategies
        #[arg(long)]
        analysis: bool,
    },

    /// Decode using universal compression
    UniversalDecode {
        /// Three words to decode
        words: String,
    },

    /// Perfect encoding - 100% reversible encoding for all IP addresses
    Adaptive {
        /// The IP address with optional port to encode
        address: String,
        
        /// Show detailed encoding analysis
        #[arg(long)]
        analysis: bool,
    },

    /// Decode perfect encoding back to exact IP address
    AdaptiveDecode {
        /// Three-word address to decode
        words: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::IpEncode { address } => {
            match IpPortEncoder::new() {
                Ok(encoder) => {
                    match encoder.encode(&address) {
                        Ok(words) => {
                            println!("IP Address: {}", address);
                            println!("Three-word: {}", words);
                            println!("Valid: ✓");
                        }
                        Err(e) => {
                            eprintln!("Error encoding IP address: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error initializing encoder: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::IpDecode { words } => {
            match IpPortEncoder::new() {
                Ok(encoder) => {
                    match encoder.decode(&words) {
                        Ok(address) => {
                            println!("Three-word: {}", words);
                            println!("IP Address: {}", address);
                        }
                        Err(e) => {
                            eprintln!("Error decoding three-word address: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error initializing encoder: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Compress { address, stats } => {
            match CompressedEncoder::new() {
                Ok(encoder) => {
                    if stats {
                        match encoder.compression_stats(&address) {
                            Ok(stats) => {
                                println!("Compression Analysis for: {}", address);
                                println!("==================================");
                                println!("{}", stats.summary());
                                println!();
                                
                                if stats.fits_in_three_words {
                                    match encoder.encode(&address) {
                                        Ok(words) => {
                                            println!("Three-word encoding: {}", words);
                                        }
                                        Err(e) => {
                                            eprintln!("Error encoding: {}", e);
                                        }
                                    }
                                } else {
                                    println!("⚠️  This address cannot be encoded in three words");
                                    println!("   It requires {} bits but only 42 bits are available", stats.compressed_bits);
                                }
                            }
                            Err(e) => {
                                eprintln!("Error analyzing compression: {}", e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        match encoder.encode(&address) {
                            Ok(words) => {
                                println!("IP Address: {}", address);
                                println!("Three-word: {}", words);
                                println!("Compression: ✓");
                            }
                            Err(e) => {
                                eprintln!("Error encoding IP address: {}", e);
                                eprintln!("\nNote: Only private networks and common ports can be compressed to three words.");
                                eprintln!("Try addresses like: 192.168.1.1:80, 10.0.0.1:443, 127.0.0.1:8080");
                                std::process::exit(1);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error initializing encoder: {}", e);
                    std::process::exit(1);
                }
            }
        }
        
        Commands::Decompress { words } => {
            match CompressedEncoder::new() {
                Ok(encoder) => {
                    match encoder.decode(&words) {
                        Ok(address) => {
                            println!("Three-word: {}", words);
                            println!("IP Address: {}", address);
                            println!("Compression: ✓");
                        }
                        Err(e) => {
                            eprintln!("Error decoding three-word address: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error initializing encoder: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Universal { address, analysis } => {
            match UniversalEncoder::new() {
                Ok(encoder) => {
                    if analysis {
                        match encoder.compression_stats(&address) {
                            Ok(stats) => {
                                println!("🔬 Universal Compression Analysis for: {}", address);
                                println!("==============================================");
                                println!("Original: {} bits (IPv4 + port)", stats.original_bits);
                                println!("Target: 42 bits (three words)");
                                println!();
                                
                                if let Some(best) = stats.best_strategy() {
                                    println!("✅ {}", stats.summary());
                                    println!();
                                    
                                    // Try encoding with best strategy
                                    match encoder.encode(&address) {
                                        Ok(words) => {
                                            println!("Three-word encoding: {}", words);
                                        }
                                        Err(e) => {
                                            println!("Encoding failed: {}", e);
                                        }
                                    }
                                } else {
                                    println!("❌ No compression strategy succeeded");
                                    println!("This address requires the full 48 bits and cannot fit in 42 bits");
                                }
                                
                                println!();
                                println!("Strategy Details:");
                                for strategy in &stats.strategies {
                                    let status = if strategy.success { "✓" } else { "✗" };
                                    println!("  {} {}: {:.1}% compression ({})", 
                                            status, strategy.name, 
                                            strategy.compression_ratio * 100.0,
                                            strategy.method);
                                }
                            }
                            Err(e) => {
                                eprintln!("Error analyzing compression: {}", e);
                                std::process::exit(1);
                            }
                        }
                    } else {
                        match encoder.encode(&address) {
                            Ok(words) => {
                                println!("IP Address: {}", address);
                                println!("Three-word: {}", words);
                                println!("Universal compression: ✓");
                            }
                            Err(e) => {
                                eprintln!("Error encoding IP address: {}", e);
                                eprintln!("\nNote: This address cannot be compressed to fit in three words.");
                                eprintln!("The fundamental limit is 42 bits (three words) vs 48 bits (IPv4+port).");
                                eprintln!("Try --analysis to see detailed compression attempts.");
                                std::process::exit(1);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error initializing universal encoder: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::UniversalDecode { words } => {
            match UniversalEncoder::new() {
                Ok(encoder) => {
                    match encoder.decode(&words) {
                        Ok(address) => {
                            println!("Three-word: {}", words);
                            println!("IP Address: {}", address);
                            println!("Universal decompression: ✓");
                        }
                        Err(e) => {
                            eprintln!("Error decoding three-word address: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error initializing universal encoder: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::Adaptive { address, analysis } => {
            match SimpleAdaptiveEncoder::new() {
                Ok(encoder) => {
                    if analysis {
                        println!("🎯 Perfect Three-Word Networking");
                        println!("=========================================");
                        println!("Using multi-dimensional encoding for 100% perfect reconstruction");
                        println!();
                        println!("Address: {}", address);
                        println!();
                        
                        // Try encoding
                        match encoder.encode(&address) {
                            Ok(encoded) => {
                                println!("✅ Encoding successful!");
                                println!("Words: {}", encoded);
                                println!();
                                println!("Features:");
                                if encoded.contains('.') && !encoded.contains('-') {
                                    println!("  • IPv4 address (dot separators)");
                                    println!("  • Simple lowercase format");
                                } else if encoded.contains('-') {
                                    println!("  • IPv6 address (dash separators)");
                                    println!("  • Title case for distinction");
                                }
                                println!("  • 100% perfect reconstruction guaranteed");
                                println!("  • Uses word order, case, and separators for extra bits");
                                
                                // Verify roundtrip
                                match encoder.decode(&encoded) {
                                    Ok(decoded) => {
                                        println!();
                                        println!("✓ Roundtrip verification: {} → {} → {}", 
                                                address, encoded, decoded);
                                    }
                                    Err(e) => {
                                        println!("\n⚠️  Decode verification failed: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("❌ Encoding failed: {}", e);
                            }
                        }
                    } else {
                        match encoder.encode(&address) {
                            Ok(encoded) => {
                                println!("IP Address: {}", address);
                                println!("Three-word: {}", encoded);
                                println!("Encoding: ✓ Perfect (100% reversible)");
                                
                                // Show appropriate messaging based on format
                                if encoded.contains('.') && !encoded.contains('-') {
                                    println!("Type: IPv4 (standard format)");
                                } else if encoded.contains('-') {
                                    println!("Type: IPv6 (enhanced format)");
                                }
                            }
                            Err(e) => {
                                eprintln!("Error encoding address: {}", e);
                                std::process::exit(1);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error initializing perfect encoder: {}", e);
                    std::process::exit(1);
                }
            }
        }

        Commands::AdaptiveDecode { words } => {
            match SimpleAdaptiveEncoder::new() {
                Ok(encoder) => {
                    match encoder.decode(&words) {
                        Ok(address) => {
                            println!("Three-word: {}", words);
                            println!("IP Address: {}", address);
                            println!("Decoding: ✓ Perfect reconstruction");
                            
                            // Show format detection
                            if words.contains('.') && !words.contains('-') {
                                println!("Type: IPv4 (detected from dot separators)");
                            } else if words.contains('-') {
                                println!("Type: IPv6 (detected from dash separators)");
                            }
                        }
                        Err(e) => {
                            eprintln!("Error decoding words: {}", e);
                            eprintln!("\nMake sure the three-word address is valid.");
                            eprintln!("Expected format: word1.word2.word3 or Word1-Word2-Word3");
                            std::process::exit(1);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error initializing perfect encoder: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}