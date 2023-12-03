```
cargo new advent2023 --bin
cargo add regex
RUST_BACKTRACE=1 cargo test --bin 001
cargo run --bin 001
rustfmt src/bin/001.rs
cargo clippy --fix
```

Do not try to use rust-mode in emacs: it is busted.