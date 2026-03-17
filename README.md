# fastarp

A fast ARP network scanner written in Rust. Discovers all active hosts on a local subnet by sending ARP requests in parallel, reporting each host's MAC address, IP address, and round-trip time.

## Installation

```bash
cargo install fastarp
```

## Usage

Root/sudo is required for raw socket access. Since `sudo` resets your `PATH`, use one of these approaches:

```bash
# Use the full path to the binary
sudo ~/.cargo/bin/fastarp eth0

# Or preserve your PATH
sudo env "PATH=$PATH" fastarp eth0

# Or copy to a system-wide location (one-time setup)
sudo cp ~/.cargo/bin/fastarp /usr/local/bin/
sudo fastarp eth0
```

Run without arguments to list available network interfaces:

```bash
sudo fastarp
```

### Example output

```
08:00:27:8a:5c:04  192.168.1.1      0.45ms
3c:7c:3f:a2:33:10  192.168.1.42     1.23ms
a4:83:e7:10:01:bf  192.168.1.100    0.87ms
3 nodes scanned in 2104ms
```

## License

MIT
