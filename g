set -o pipefail

# Clean
#(pushd dsp && cargo clean && popd && cargo clean && pushd shim && make clean && popd)

# Board exe
cargo build --lib --target thumbv7em-none-eabihf 2>&1 | tee out
#pushd shim && make clean_exes && make PEDAL_MAIN=reso_main all && popd
exit

# Sim
cargo run --bin sim --features stdd 2>&1 | tee out
exit

cargo build --lib --target thumbv7em-none-eabihf 2>&1 | tee out
exit

# Build dsp for board
# cargo build --target thumbv7em-none-eabihf
# Build dsp for host
# cargo build --features stdd

#(pushd dsp && cargo clean && popd && cargo clean && pushd shim && make clean && popd)
(pushd dsp && cargo build --lib --target thumbv7em-none-eabihf && popd && \
  cargo build --lib --target thumbv7em-none-eabihf && \
  pushd shim && make clean_exes && make PEDAL_MAIN=reso_main all && popd) 2>&1 | tee out
mv out bout
exit
cargo run --features stdd 2>&1 | tee out
mv out hout
echo ----
tail -12 bout
echo ----
tail -2 hout

exit

#cargo run 2>&1 | tee out
cargo run --features stdd 2>&1 | tee out

#./bld reso_main 2>&1 | tee out
