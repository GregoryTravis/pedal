set -x
set -e

pushd host
cargo run --bin test --features for_host
popd

pushd lib/shared
cargo test --lib --features for_host
popd
