set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

# Install/update dependencies
install:
    rustup update
    cargo install cargo-watch

# Pre-commit checks
pre-commit: && readme clean
    cargo clippy --all-targets --all-features -- -D warnings
    cargo fmt
    cargo doc
    cargo test

# Build and open docs
doc: pre-commit
    cargo doc --open
