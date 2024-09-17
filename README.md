
# Market Data Feed Parser

This Rust program is designed to parse and process quote messages from a market data feed. It can handle large files efficiently and re-order messages based on the 'quote accept time' if requested.


## Requirements

- Rust (latest stable version of `rustc`)

## How to Build

To build the project, ensure you have Rust installed and then run:

```sh
cargo build --release
```

## How to Run

### Parsing without Reordering

To parse and print the quote messages from a pcap file without reordering, use:

```sh
cargo run --release -- mdf-kospi200.20110216-0.pcap
```

### Parsing with Reordering

To parse and reorder the quote messages based on the quote accept time, use the `-r` flag:

```sh
cargo run --release -- -r mdf-kospi200.20110216-0.pcap
```