# junbitsu — Zero-Touch Device Provisioning

YAML-driven provisioning: unlock, flash, install, configure, enroll. Consumes FastbootTransport, AdbTransport, SparseImageParser traits.

## Build & Test

```bash
cargo build
cargo test
cargo run -- provision <config.yaml>
```

## Conventions

- Edition 2024, Rust 1.91.0+, MIT, clippy pedantic
- Release: codegen-units=1, lto=true, opt-level="z", strip=true
