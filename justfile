default: 
    just --list

# Build in debug mode
build:
    cargo build

# Build in release mode
build-release:
    cargo build --release

# Run all tests
test:
    cargo test --workspace

# Run clippy lints
lint:
    cargo clippy --workspace -- -D warnings

# Format code
fmt:
    cargo fmt --all

# Check formatting without modifying
fmt-check:
    cargo fmt --all -- --check

# Run the scanner (requires sudo)
run iface:
    sudo cargo run --release -- {{iface}}

# Publish lib_arp then fastarp to crates.io
release:
    cargo release --package fastarp --execute

# Dry-run release (no publish, no git push)
release-dry:
    cargo release --package fastarp
