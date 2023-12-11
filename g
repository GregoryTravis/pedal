set -o pipefail

#./board
cargo run --bin sim --features stdd 2>&1 | tee out
