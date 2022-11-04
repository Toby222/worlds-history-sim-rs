#!/bin/env /bin/sh
cargo build --no-default-features --features= &&
cargo build --no-default-features --features=logging &&
cargo build --no-default-features --features=render &&
cargo build --no-default-features --features=logging,render &&
cargo build --no-default-features --features=globe_view &&
cargo build --no-default-features --features=logging,globe_view &&
cargo build --release --no-default-features --features= &&
cargo build --release --no-default-features --features=logging &&
cargo build --release --no-default-features --features=render &&
cargo build --release --no-default-features --features=logging,render &&
cargo build --release --no-default-features --features=globe_view &&
cargo build --release --no-default-features --features=logging,globe_view