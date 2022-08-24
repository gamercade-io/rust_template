# Gamercade Rust Template

A way to get up and running with [Gamercade](https://gamercade.io) quick and easy. Comes bundled with the [gamercade_rs](https://crates.io/crates/gamercade_rs) crate.

This project is already configured to output the .wasm binaries via the `.cargo/config` file.

Learn more about [Gamercade](https://gamercade.io), or head over to the main project repository: [gamercade_console](https://github.com/gamercade-io/gamercade_console)

## How to butput a .wasm File:

1. If you don't already have it, install the wasm target by running `rustup target add wasm32-unknown-unknown`.
1. Invoke `cargo build` or `cargo build --release` as you would normally. This project will default to building for target `wasm32-unknown-unknown`
1. If successful, the output will be in `./target/wasm32-unknown-unknown/`, inside of `debug` or `release` respectively.
1. The file name will be from `Cargo.toml`. Which in this case, is `rust_template.wasm`
1. You can then bundle this `.wasm` with the editor to generate playable `.gcrom` files. [Learn more here](https://github.com/gamercade-io/gamercade_console#building-bundling-and-running-a-game)