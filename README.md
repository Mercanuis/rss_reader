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

## arguments

**input**
### usage
```bash
./rss_reader [-i|--input| [FILE]
```

Uses **FILE** as the input to the reader. Clashes with **url**

**url**
```bash
./rss_reader [-u|--url] [URL]
```

Uses **URL** as the input, calls via HTTPS with [Reqwest](https://docs.rs/reqwest/0.11.1/reqwest/index.html) to get the returned Response and [RSS](https://docs.rs/rss/1.10.0/rss/index.html) to parse it. Clashes with **input**

**trim**
```bash
./rss_reader [-t|--trim]
```

Sets any values that are determined and managed by the reader to be a maximum of 10 chars long (UTF-8)

**replace**
```bash
./rss_reader [-r|--replace] "FROM|TO"
```

Takes a string that is separated by **|** and replaces the **FROM** to **TO**. Any additional values in the string are ignored

**output**
```bash
./rss_reader [-o|--output¥ [FILE]
```

Uses the provided **FILE** as output
