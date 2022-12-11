#!/bin/env /bin/sh
echo "Debug-build with features: minimal"
cargo build -j 6 --no-default-features --features= &&
echo "Debug-build with features: logging" &&
cargo build -j 6 --no-default-features --features=logging &&
echo "Debug-build with features: render" &&
cargo build -j 6 --no-default-features --features=render &&
echo "Debug-build with features: logging render" &&
cargo build -j 6 --no-default-features --features="logging,render" &&
echo "Release-build with features: minimal"
cargo build -j 6 --release --no-default-features --features= &&
echo "Release-build with features: logging" &&
cargo build -j 6 --release --no-default-features --features=logging &&
echo "Release-build with features: render" &&
cargo build -j 6 --release --no-default-features --features=render &&
echo "Release-build with features: logging render" &&
cargo build -j 6 --release --no-default-features --features="logging,render" &&
echo "Done!"