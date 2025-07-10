//! 3wn - Three-Word Networking CLI
//!
//! Simple command-line tool that automatically detects whether input is:
//! - 3 words (IPv4) or 6/9 words (IPv6) → decode to IP:port
//! - IP:port → encode to 3 words (IPv4) or 6/9 words (IPv6)
//!
//! Features 100% perfect reconstruction for IPv4 and adaptive compression for IPv6.
//!
//! Usage:
//!   3wn 192.168.1.1:80          # Encodes to 3 words (perfect)
//!   3wn paper.broaden.smith     # Decodes to exact IPv4:port
//!   3wn [2001:db8::1]:443      # Encodes to 6 or 9 words with visual distinction
//!   3wn Ocean-Thunder-Falcon-Star-Book-April    # Decodes to IPv6 (note dashes and case)

use clap::Parser;
use std::process;
use three_word_networking::{Result, ThreeWordAdaptiveEncoder};

#[derive(Parser)]
#[command(
    name = "3wn",
    about = "Three-Word Networking - Convert between IP addresses and memorable words",
    long_about = "Automatically converts between IP addresses and three-word combinations.\n\
                  Features 100% perfect reconstruction for IPv4 and adaptive compression for IPv6.\n\
                  IPv4 uses 3 words with dots (paper.broaden.smith), IPv6 uses 6 or 9 words with dashes (Ocean-Thunder-Falcon-Star-Book-April).",
    version
)]
struct Cli {
    /// Input to convert (IP:port or words)
    /// Can be a single string or multiple words
    input: Vec<String>,

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
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    let encoder = ThreeWordAdaptiveEncoder::new()?;

    // Join input arguments
    let input = if cli.input.len() == 1 {
        // Single argument - could be IP or words with separators
        cli.input[0].trim().to_string()
    } else {
        // Multiple arguments - treat as space-separated words
        cli.input.join(" ")
    };

    // Detect input type based on content
    if looks_like_words(&input) {
        // Input is words, decode to IP:port
        decode_words(&encoder, &input, cli.verbose, cli.quiet)
    } else {
        // Input is IP:port, encode to words
        encode_address(&encoder, &input, cli.verbose, cli.quiet)
    }
}

/// Check if input looks like words (contains dots, dashes, spaces, all alphabetic)
fn looks_like_words(input: &str) -> bool {
    // Handle space-separated words or separator-based words
    let segments: Vec<&str> = if input.contains(' ') && !input.contains('-') && !input.contains(':')
    {
        // Space-separated words (not IPv6 with colons)
        input.split_whitespace().collect()
    } else if input.contains('.')
        || input.contains('-')
        || input.contains('_')
        || input.contains('+')
    {
        // Split by any separator
        input.split(|c: char| ".-_+".contains(c)).collect()
    } else {
        // No separators - not words
        return false;
    };

    // Must be 3 (IPv4), 6 or 9 (IPv6) segments
    if segments.len() != 3 && segments.len() != 6 && segments.len() != 9 {
        return false;
    }

    // Check if all segments are alphabetic and meet minimum length requirement
    segments
        .iter()
        .all(|segment| segment.len() >= 2 && segment.chars().all(|c| c.is_alphabetic()))
}

/// Encode IP address to words
fn encode_address(
    encoder: &ThreeWordAdaptiveEncoder,
    address: &str,
    verbose: bool,
    quiet: bool,
) -> Result<()> {
    let words = encoder.encode(address)?;

    if quiet {
        // Minimal output for scripting
        println!("{words}");
    } else if verbose {
        // Detailed output
        println!("Input: {address}");
        println!("Words: {words}");
        println!("Encoding: Perfect (100% reversible)");

        if words.contains('.') && !words.contains('-') {
            println!("Type: IPv4 (dot separators, lowercase)");
        } else if words.contains('-') {
            println!("Type: IPv6 (dash separators, title case)");
        }

        println!("Features:");
        println!("  • Perfect IPv4 reconstruction (3 words)");
        println!("  • Adaptive IPv6 compression (6 or 9 words)");
        println!("  • Guaranteed perfect reconstruction");
    } else {
        // Normal output
        println!("{words}");
    }

    Ok(())
}

/// Decode words to IP address
fn decode_words(
    encoder: &ThreeWordAdaptiveEncoder,
    words: &str,
    verbose: bool,
    quiet: bool,
) -> Result<()> {
    let address = encoder.decode(words)?;

    if quiet {
        // Minimal output for scripting
        println!("{address}");
    } else if verbose {
        // Detailed output
        println!("Input: {words}");
        println!("Address: {address}");
        println!("Decoding: Perfect reconstruction");

        if words.contains('.') && !words.contains('-') {
            println!("Type: IPv4 (detected from dot separators)");
        } else if words.contains('-') {
            println!("Type: IPv6 (detected from dash separators)");
        }
    } else {
        // Normal output
        println!("{address}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_looks_like_words() {
        // Valid words - 3 words with dots
        assert!(looks_like_words("ocean.thunder.falcon"));

        // Valid words - 6 words with dashes
        assert!(looks_like_words("Ocean-Thunder-Falcon-Star-Book-April"));

        // Valid words - 3 words with underscores
        assert!(looks_like_words("ocean_thunder_falcon"));

        // Invalid - wrong count
        assert!(!looks_like_words("ocean.thunder"));
        assert!(!looks_like_words("a.b.c.d.e"));

        // Invalid - contains non-alphabetic
        assert!(!looks_like_words("ocean.thunder.123"));
        assert!(!looks_like_words("192.168.1.1"));
        assert!(!looks_like_words("ocean:thunder:falcon"));
    }
}
