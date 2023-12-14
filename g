set -o pipefail

#./t
#./board
rm -f out.wav
cargo run --bin sim --features for_host 2>&1 | tee out
