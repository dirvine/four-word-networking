#!/usr/bin/env python3
"""
Security Comparison: Three-Word Addresses vs Traditional Passwords

Compares the entropy and security characteristics of three-word addresses
against traditional password schemes.

Usage:
    python3 security_comparison.py

Requirements:
    - None (standalone analysis)
"""

import math

def format_time(seconds):
    """Format time duration in human-readable format"""
    if seconds < 60:
        return f"{seconds:.1f} seconds"
    elif seconds < 3600:
        return f"{seconds/60:.1f} minutes"
    elif seconds < 86400:
        return f"{seconds/3600:.1f} hours"
    elif seconds < 31536000:
        return f"{seconds/86400:.1f} days"
    elif seconds < 31536000 * 1000:
        return f"{seconds/31536000:.1f} years"
    elif seconds < 31536000 * 1e6:
        return f"{seconds/31536000/1000:.1f} thousand years"
    elif seconds < 31536000 * 1e9:
        return f"{seconds/31536000/1e6:.1f} million years"
    else:
        return f"{seconds/31536000/1e9:.1f} billion years"

def main():
    print("Three-Word Address vs Strong Password: Security Comparison")
    print("="*70)

    # Three-word system
    three_word_bits = 48  # 3 × 16 bits

    print("\n📍 THREE-WORD ADDRESS SYSTEM")
    print("-"*40)
    print(f"Format: word1.word2.word3")
    print(f"Example: sunset.river.song")
    print(f"Total entropy: {three_word_bits} bits")
    print(f"Possible combinations: {2**three_word_bits:,}")

    # Password comparisons
    print("\n🔐 EQUIVALENT PASSWORD STRENGTH")
    print("-"*40)

    # Character set sizes
    charsets = [
        ("Lowercase only (a-z)", 26),
        ("Alphanumeric (a-z, 0-9)", 36),
        ("Mixed case + digits (a-zA-Z0-9)", 62),
        ("With symbols (!@#$%^&*)", 70),
        ("Full ASCII printable", 95)
    ]

    print("\nTo achieve 48 bits of entropy, you need:")
    for name, size in charsets:
        required_length = math.ceil(48 / math.log2(size))
        example_passwords = {
            26: "xkqvbnmwerty",  # lowercase
            36: "p7k3m9nw2",     # alphanumeric
            62: "Kj7mN2pQ",      # mixed case + digits
            70: "Tr0ub&3x",      # with symbols
            95: "9k#X$2p!"       # full ASCII
        }
        example = example_passwords.get(size, "N/A")[:required_length]
        print(f"• {name:<35} {required_length:>2} characters  (e.g., {example})")

    # Time to crack at different speeds
    print("\n⏱️  TIME TO CRACK (50% probability)")
    print("-"*40)

    crack_speeds = [
        ("Regular computer", 1e6),           # 1 million/sec
        ("Gaming GPU", 1e9),                 # 1 billion/sec
        ("Professional cracking rig", 1e12),  # 1 trillion/sec
        ("Nation-state resources", 1e15),     # 1 quadrillion/sec
    ]

    for speed_name, speed in crack_speeds:
        seconds = 2**(three_word_bits-1) / speed
        time_str = format_time(seconds)
        print(f"• {speed_name:<25} {time_str}")

    # Real-world comparison
    print("\n🌍 REAL-WORLD CONTEXT")
    print("-"*40)

    security_levels = [
        ("Typical user password", 30),
        ("'Strong' password (most sites)", 40),
        ("Three-word address", 48),
        ("NIST 2030 minimum", 112),
        ("AES-128 encryption", 128),
        ("Bitcoin private key", 256),
    ]

    for name, bits in security_levels:
        marker = "→" if bits == 48 else " "
        print(f"{marker} {name:<30} {bits:>3} bits")

    # Practical advantages
    print("\n✨ PRACTICAL ADVANTAGES")
    print("-"*40)
    print("Three-word address (sunset.river.song):")
    print("• ✓ Memorable - uses real words")
    print("• ✓ Voice-friendly - easy to say over phone")
    print("• ✓ Typo-resistant - spell checkers help")
    print("• ✓ Cross-cultural - works in any language")
    print("• ✓ No special characters needed")

    print("\nRandom 8-char password (Kj7$mN2p):")
    print("• ✗ Impossible to remember")
    print("• ✗ Hard to communicate verbally") 
    print("• ✗ Easy to mistype")
    print("• ✗ Requires password manager")
    print("• ✗ Special characters cause issues")

    # Summary
    print("\n📊 SUMMARY")
    print("-"*40)
    print(f"Three words provide {three_word_bits} bits of entropy - equivalent to:")
    print(f"• An 8-character random password with mixed case, digits & symbols")
    print(f"• 4.5 years to crack with a professional cracking rig")
    print(f"• 4,500 years with a nation-state level attack")
    print(f"\nBut unlike K#7mN2p$, you can actually remember 'sunset.river.song'!")

    # Extended analysis
    print("\n🔍 EXTENDED ANALYSIS")
    print("-"*40)
    
    print("\nCommon password policies vs three-word addresses:")
    policies = [
        ("8 chars, mixed case + digits", 8 * math.log2(62)),
        ("12 chars, mixed case + digits", 12 * math.log2(62)),
        ("16 chars, mixed case + digits", 16 * math.log2(62)),
        ("Three-word address", 48),
        ("Four random words (XKCD)", 4 * math.log2(7776)),  # Diceware
    ]
    
    for policy, entropy in policies:
        memorable = "✓" if "word" in policy.lower() else "✗"
        print(f"• {policy:<30} {entropy:>6.1f} bits  Memorable: {memorable}")

if __name__ == "__main__":
    main()