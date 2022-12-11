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
