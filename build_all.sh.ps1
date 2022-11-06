#!/bin/env /bin/sh
echo "Debug-build features: minimal"
cargo build --no-default-features --features= &&
echo "Debug-build features: logging" &&
cargo build --no-default-features --features=logging &&
echo "Debug-build features: render" &&
cargo build --no-default-features --features=render &&
echo "Debug-build features: logging,render" &&
cargo build --no-default-features --features=logging,render &&
echo "Debug-build features: globe_view" &&
cargo build --no-default-features --features=globe_view &&
echo "Debug-build features: logging,globe_view" &&
cargo build --no-default-features --features=logging,globe_view &&
echo "Release-build features: minimal"
cargo build --release --no-default-features --features= &&
echo "Release-build features: logging" &&
cargo build --release --no-default-features --features=logging &&
echo "Release-build features: render" &&
cargo build --release --no-default-features --features=render &&
echo "Release-build features: logging,render" &&
cargo build --release --no-default-features --features=logging,render &&
echo "Release-build features: globe_view" &&
cargo build --release --no-default-features --features=globe_view &&
echo "Release-build features: logging,globe_view" &&
cargo build --release --no-default-features --features=logging,globe_view &&
echo "Done!"