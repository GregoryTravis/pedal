set -x
set -e

pushd host && \
cargo run --bin test --features for_host && \
cargo test --features for_host && \
popd && \

pushd lib/shared && \
./build-edsl-patches.sh && \
cargo test --lib --features for_host && \
popd
