#!/bin/env /bin/sh
echo "Debug-build with features: minimal"
cargo build --no-default-features --features= &&
echo "Debug-build with features: logging" &&
cargo build --no-default-features --features=logging &&
echo "Debug-build with features: render" &&
cargo build --no-default-features --features=render &&
echo "Debug-build with features: logging render" &&
cargo build --no-default-features --features="logging,render" &&
echo "Release-build with features: minimal"
cargo build --release --no-default-features --features= &&
echo "Release-build with features: logging" &&
cargo build --release --no-default-features --features=logging &&
echo "Release-build with features: render" &&
cargo build --release --no-default-features --features=render &&
echo "Release-build with features: logging render" &&
cargo build --release --no-default-features --features="logging,render" &&
echo "Done!"