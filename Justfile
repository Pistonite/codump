set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Install/update dependencies
install:
    rustup update
    cargo install cargo-watch
    cargo install txtpp

# Generate readme
readme:
    txtpp README.md

# Pre-commit checks
pre-commit: && readme 
    cargo clippy --all-targets --all-features -- -D warnings
    cargo fmt
    cargo doc
    cargo test

# Build and open docs
doc: pre-commit
    cargo doc --open
