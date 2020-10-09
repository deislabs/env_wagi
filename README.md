# env_wagi: Example WAGI program

WAGI (Web Assembly Gateway Interface) is an adaptation of CGI for WebAssembly WASI modules.

This repo contains a Rust implementation of a WAGI program that merely dumps arguments and environment variables. It is designed for debugging a WAGI server.

## Building

You can test the output of this command using `cargo run`.

However, when building it for WAGI, you need to compile this as Web Assembly with WASI support:

```
$ cargo build --target wasm32-wasi --release
```

(We recommend using `--release` because it considerably reduces the size.)

The resulting binary will be written to `target/wasm32-wasi/release/hello_wagi.wasm`.