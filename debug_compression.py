#!/usr/bin/env python3
"""
Debug script to analyze IPv6 address segments and compression logic.

UV Dependencies:
# Install UV if not available: curl -LsSf https://astral.sh/uv/install.sh | sh
# Run this script: uv run python debug_compression.py
"""

# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///

import socket
import struct

def analyze_ipv6_address(addr_str):
    """Analyze an IPv6 address and show its segments."""
    print(f"Analyzing address: {addr_str}")
    
    # Parse the address
    addr_bytes = socket.inet_pton(socket.AF_INET6, addr_str)
    
    # Convert to 16-bit segments (big-endian)
    segments = struct.unpack('>8H', addr_bytes)
    
    print(f"Segments: {[hex(s) for s in segments]}")
    print(f"Segments (decimal): {list(segments)}")
    
    # Check segments 4-7 (interface ID)
    interface_segments = segments[4:8]
    print(f"Interface ID segments (4-7): {[hex(s) for s in interface_segments]}")
    
    non_zero_interface = [(i+4, seg) for i, seg in enumerate(interface_segments) if seg != 0]
    print(f"Non-zero interface segments: {non_zero_interface}")
    print(f"Is interface empty? {len(non_zero_interface) == 0}")
    
    return segments, non_zero_interface

if __name__ == "__main__":
    analyze_ipv6_address("2001:db8:85a3::8a2e:370:7334")