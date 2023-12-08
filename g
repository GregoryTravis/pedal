set -o pipefail

#cargo run 2>&1 | tee out
cargo run --features stdd 2>&1 | tee out

#./bld reso_main 2>&1 | tee out
