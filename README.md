# Get the toolchain for WASI

```
rustup target add wasm32-wasi
```

# Build

```
cargo build --release --target wasm32-wasi
```
(binary is under target/release/wasm32-wasi/wasm-gcd.wasm)

# Reference:

https://blog.dkwr.de/development/rust-wasm-compilation/