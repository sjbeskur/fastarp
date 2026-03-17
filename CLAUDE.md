# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

fastarp is a fast ARP network scanner written in Rust. Given a network interface name, it discovers all active hosts on the local subnet by sending ARP requests in parallel and collecting replies, reporting each host's MAC address, IP address, and round-trip time.

## Build and Test Commands

```bash
cargo build --release
cargo test --workspace

# Run (requires root/sudo for raw socket access)
sudo cargo run --release -- <interface_name>
# Example: sudo cargo run --release -- eth0
```

## Architecture

This is a Cargo workspace with two members:

- **`crates/fastarp`** (binary) — CLI entrypoint in `crates/fastarp/src/main.rs`. Parses the interface name argument, calls `lib_arp::scan_v4()`, and prints discovered nodes with timing.

- **`crates/lib_arp`** (library) — Core scanning logic in `crates/lib_arp/src/`:
  - **`arpscan.rs`** — Main pipeline: `scan_v4(iface_name)` resolves all IPv4 addresses in the subnet, spawns a listener thread to capture ARP replies via mpsc channel, divides IPs into chunks (`compute_chunk_size` uses logarithmic scaling), spawns worker threads to send ARP requests via pnet, then collects responses and calculates RTT.
  - **`arpnode.rs`** — `ArpNode` struct: mac_address, ipv4_address, ipv4_target, ping_ms.
  - **`lib.rs`** — Re-exports public API and defines `ArpResult<T>` / `ArpErrors`.

**Data flow**: `main` → `scan_v4` → `get_iface_ips` (subnet enumeration) + `listen_for_arp` (receiver thread) + parallel `send_arp` workers → collect into `HashMap<String, ArpNode>`.

## Key Dependencies

- `pnet` — Raw packet construction and datalink interface access
- `ipnetwork` — CIDR/subnet parsing and IP enumeration
