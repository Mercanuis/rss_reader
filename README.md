# rss_reader
Small project and exercise with  rust to make an RSS reader

## usage

There are two ways to go about using the reader

#### Via cargo
Requires installation of [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
```bash
cargo build
cargo run -- [ARGS]
```

#### Via binary
You can also run the binary (provides the ability to use shortcuts as cargo needs 'long' version arguments)
```bash
cargo build //builds the binary, usually in target for most JetBrains IDEs
cd 'target'
./rss_reader [ARGS]
```
