Cursion
===

[![Latest Version](https://img.shields.io/crates/v/cursion.svg)](https://crates.io/crates/cursion)
[![docs](https://docs.rs/cursion/badge.svg)](https://docs.rs/cursion)

A pure Rust, cursor optimization library to avoid flickering in terminal.

## Examples

Run unoptimized exmaple. You may notice a flickering.

```bash
$ cargo run --examples draw_unoptimized
```

Run optimized example. You don't see a flickering.

```bash
$ cargo run --examples draw_optimized
```
