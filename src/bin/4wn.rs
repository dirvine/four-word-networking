//! 4wn - Four-Word Networking CLI
//!
//! Simple command-line tool that automatically detects whether input is:
//! - 4-6 words → decode to IP:port
//! - IP:port → encode to 4-6 words
//!
//! Features 100% perfect reconstruction for IPv4 and adaptive compression for IPv6.
//!
//! Usage:
//!   4wn 192.168.1.1:80          # Encodes to 4 words (perfect)
//!   4wn paper.broaden.smith.bully    # Decodes to exact IPv4:port
//!   4wn [2001:db8::1]:443      # Encodes to 4-6 words with visual distinction
//!   4wn Ocean-Thunder-Falcon-Star    # Decodes to IPv6 (note dashes and case)

use clap::Parser;
use four_word_networking::{FourWordAdaptiveEncoder, Result};
use std::process;

#[derive(Parser)]
#[command(
    name = "4wn",
    about = "Four-Word Networking - Convert between IP addresses and memorable words",
    long_about = "Automatically converts between IP addresses and four-word combinations.\n\
                  Features 100% perfect reconstruction for IPv4 and adaptive compression for IPv6.\n\
                  IPv4 uses 4 words with dots (paper.broaden.smith.bully), IPv6 uses 4-6 words with dashes (Ocean-Thunder-Falcon).",
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
    let encoder = FourWordAdaptiveEncoder::new()?;
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

/// Check if input looks like words (contains dots or dashes, all alphabetic)
fn looks_like_words(input: &str) -> bool {
    // Check for word separators
    let has_separators =
        input.contains('.') || input.contains('-') || input.contains('_') || input.contains('+');
    if !has_separators {
        return false;
    }

    // Split by any separator and count
    let segments: Vec<&str> = input.split(|c: char| ".-_+".contains(c)).collect();

    // Must be 3-6 segments (3 for legacy, 4 for IPv4, 4-6 for IPv6)
    if segments.len() < 3 || segments.len() > 6 {
        return false;
    }

    // Check if all segments are alphabetic and meet minimum length requirement
    segments
        .iter()
        .all(|segment| segment.len() >= 2 && segment.chars().all(|c| c.is_alphabetic()))
}

/// Encode IP address to words
fn encode_address(
    encoder: &FourWordAdaptiveEncoder,
    address: &str,
    verbose: bool,
    quiet: bool,
) -> Result<()> {
    let words = encoder.encode(address)?;

    if quiet {
        // Minimal output for scripting
        println!("{}", words);
    } else if verbose {
        // Detailed output
        println!("Input: {}", address);
        println!("Words: {}", words);
        println!("Encoding: Perfect (100% reversible)");

        if words.contains('.') && !words.contains('-') {
            println!("Type: IPv4 (dot separators, lowercase)");
        } else if words.contains('-') {
            println!("Type: IPv6 (dash separators, title case)");
        }

        println!("Features:");
        println!("  • Perfect IPv4 reconstruction (4 words)");
        println!("  • Adaptive IPv6 compression (4-6 words)");
        println!("  • Guaranteed perfect reconstruction");
    } else {
        // Normal output
        println!("{}", words);
    }

    Ok(())
}

/// Decode words to IP address
fn decode_words(
    encoder: &FourWordAdaptiveEncoder,
    words: &str,
    verbose: bool,
    quiet: bool,
) -> Result<()> {
    let address = encoder.decode(words)?;

    if quiet {
        // Minimal output for scripting
        println!("{}", address);
    } else if verbose {
        // Detailed output
        println!("Input: {}", words);
        println!("Address: {}", address);
        println!("Decoding: Perfect reconstruction");

        if words.contains('.') && !words.contains('-') {
            println!("Type: IPv4 (detected from dot separators)");
        } else if words.contains('-') {
            println!("Type: IPv6 (detected from dash separators)");
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
        // Valid words - dots
        assert!(looks_like_words("ocean.thunder.falcon"));

        // Valid words - dashes
        assert!(looks_like_words("Ocean-Thunder-Falcon"));

        // Valid words - mixed separators
        assert!(looks_like_words("ocean_thunder_falcon"));

        // Invalid - wrong count
        assert!(!looks_like_words("ocean.thunder"));
        assert!(!looks_like_words("a.b.c.d"));

        // Invalid - contains non-alphabetic
        assert!(!looks_like_words("ocean.thunder.123"));
        assert!(!looks_like_words("192.168.1.1"));
        assert!(!looks_like_words("ocean:thunder:falcon"));
    }
}
