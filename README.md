# lightningcss vs rust-cssparser compile times

This repository compares the compile time costs of [lightningcss](https://github.com/parcel-bundler/lightningcss)
and [cssparser](https://github.com/servo/rust-cssparser).

Related lightningcss issue: https://github.com/parcel-bundler/lightningcss/issues/357

```
git clone git@github.com:chinedufn/rust-cssparser-and-lightningcss-compile-times.git
./run.sh
```

## Sample output:

Device | Processor | Memory
--- | --- | ---
MacBook Pro (2019, 16 inch) | Processor: 2.4 GHz 8-Core Intel Core i9 | Memory: 64 GB 2667 MHz DDR4 

#### Debug Build

(lightningcss compiles ~4x slower than rust-cssparser)

```
rust-cssparser debug build timing
real 0m11.188s user 0m29.876s sys 0m3.621s

lightningcss debug build timing
real 0m40.468s user 2m40.552s sys 0m16.839s
```

#### Release Build

(lightningcss compiles ~7x slower than rust-cssparser)

```
rust-cssparser release build time
real 0m9.391s user 0m25.043s sys 0m3.294s

lightningcss release build timing
real 1m10.555s user 6m12.164s sys 0m18.262s
```

## Method

The timing script used can be viewed at [run.sh](./run.sh).

In it we compile two debug builds, the first with `rust-cssparser` enabled and the second with `lightningcss` enabled.

The code used is copied below for convenience.

```toml
# via: ./Cargo.toml

[package]
name = "rust-cssparser-and-lightningcss-compile-times"
version = "0.1.0"
edition = "2021"

[features]
with-lightningcss = ["lightningcss"]
with-rust-cssparser = ["cssparser"]

[dependencies]
cssparser = {optional = true, version = "=0.29"}
lightningcss = {optional = true, version = "=1.0.0-alpha.38"}
```

```rust
// via: ./src/main.rs

fn main() {
    #[cfg(feature = "with-rust-cssparser")]
    run_cssparser();
    #[cfg(feature = "with-lightningcss")]
    run_lightningcss();
}

#[cfg(feature = "with-rust-cssparser")]
fn run_cssparser() {
    use cssparser::{Parser, ParserInput};

    let mut input = ParserInput::new("body { color: red; }");
    let mut parser = Parser::new(&mut input);

    for token in parser.next() {
        dbg!(&token);
    }
}

#[cfg(feature = "with-lightningcss")]
fn run_lightningcss() {
    use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
    let sheet = StyleSheet::parse("body { color: red; }", ParserOptions::default()).unwrap();
    println!("{}", sheet.to_css(PrinterOptions::default()).unwrap().code);
}
```

```sh
# via: ./run.sh

#!/bin/bash

set -e

cargo clean

exec 3>&1 4>&2
RUST_CSS_PARSER_TIME=$( { time cargo build --release --features with-rust-cssparser 1>&3 2>&4; } 2>&1 )
exec 3>&- 4>&-

cargo clean

exec 3>&1 4>&2
LIGHTNINGCSS_PARSER_TIME=$( { time cargo build --release --features with-lightningcss 1>&3 2>&4; } 2>&1 )
exec 3>&- 4>&-

echo "rust-cssparser debug build timing"
echo $RUST_CSS_PARSER_TIME

echo ""

echo "lightningcss debug build timing"
echo $LIGHTNINGCSS_PARSER_TIME
```
