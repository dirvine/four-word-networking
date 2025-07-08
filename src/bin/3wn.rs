//! 3wn - Three-Word Networking CLI
//!
//! Simple command-line tool that automatically detects whether input is:
//! - 3-6 words → decode to IP:port
//! - IP:port → encode to words
//!
//! Usage:
//!   3wn 192.168.1.1:80          # Encodes to 3 words
//!   3wn ocean.thunder.falcon    # Decodes to IP:port
//!   3wn [2001:db8::1]:443      # Encodes to 4-6 words
//!   3wn book.book.smell.book    # Decodes to IPv6

use clap::Parser;
use std::process;
use three_word_networking::{AdaptiveEncoder, Result};

#[derive(Parser)]
#[command(
    name = "3wn",
    about = "Three-Word Networking - Convert between IP addresses and memorable words",
    long_about = "Automatically converts between IP addresses and three-word combinations.\n\
                  IPv4 addresses always produce exactly 3 words.\n\
                  IPv6 addresses produce 4-6 words for clear differentiation.",
    version
)]
struct Cli {
    /// Input to convert (IP:port or words)
    input: String,

    /// Show detailed information
    #[arg(short, long)]
    verbose: bool,

    /// Output format for scripting (minimal output)
    #[arg(short, long)]
    quiet: bool,
}

fn main() {
    let cli = Cli::parse();
    
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let encoder = AdaptiveEncoder::new()?;
    let input = cli.input.trim();
    
    // Detect input type based on content
    if looks_like_words(input) {
        // Input is words, decode to IP:port
        decode_words(&encoder, input, cli.verbose, cli.quiet)
    } else {
        // Input is IP:port, encode to words
        encode_address(&encoder, input, cli.verbose, cli.quiet)
    }
}

/// Check if input looks like words (contains dots but no colons or slashes)
fn looks_like_words(input: &str) -> bool {
    let word_count = input.split('.').count();
    
    // Must be 3-6 dot-separated segments
    if word_count < 3 || word_count > 6 {
        return false;
    }
    
    // Check if all segments are alphabetic
    input.split('.').all(|segment| {
        !segment.is_empty() && segment.chars().all(|c| c.is_alphabetic())
    })
}

/// Encode IP address to words
fn encode_address(encoder: &AdaptiveEncoder, address: &str, verbose: bool, quiet: bool) -> Result<()> {
    let result = encoder.encode(address)?;
    let words = result.words();
    
    if quiet {
        // Minimal output for scripting
        println!("{}", words);
    } else if verbose {
        // Detailed output
        println!("Input: {}", address);
        println!("Type: {}", result.address_type.description());
        println!("Words: {}", words);
        println!("Count: {} words", result.encoding.word_count);
        println!("Method: {}", result.compression_method);
        
        match result.address_type {
            three_word_networking::AddressType::Ipv4 => {
                println!("Note: IPv4 addresses always use exactly 3 words");
            }
            three_word_networking::AddressType::Ipv6 => {
                println!("Note: IPv6 addresses use 4-6 words for clear differentiation");
            }
        }
    } else {
        // Normal output
        println!("{}", words);
    }
    
    Ok(())
}

/// Decode words to IP address
fn decode_words(encoder: &AdaptiveEncoder, words: &str, verbose: bool, quiet: bool) -> Result<()> {
    let address = encoder.decode(words)?;
    let word_count = words.split('.').count();
    
    if quiet {
        // Minimal output for scripting
        println!("{}", address);
    } else if verbose {
        // Detailed output
        println!("Input: {}", words);
        println!("Count: {} words", word_count);
        println!("Address: {}", address);
        
        match word_count {
            3 => println!("Type: IPv4 (3 words always indicates IPv4)"),
            4..=6 => println!("Type: IPv6 (4-6 words always indicates IPv6)"),
            _ => println!("Type: Unknown"),
        }
    } else {
        // Normal output
        println!("{}", address);
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looks_like_words() {
        // Valid words
        assert!(looks_like_words("ocean.thunder.falcon"));
        assert!(looks_like_words("book.book.smell.book"));
        assert!(looks_like_words("alpha.beta.gamma.delta.epsilon"));
        
        // Invalid - wrong count
        assert!(!looks_like_words("ocean.thunder"));
        assert!(!looks_like_words("a.b.c.d.e.f.g"));
        
        // Invalid - contains non-alphabetic
        assert!(!looks_like_words("ocean.thunder.123"));
        assert!(!looks_like_words("192.168.1.1"));
        assert!(!looks_like_words("ocean:thunder:falcon"));
    }
}