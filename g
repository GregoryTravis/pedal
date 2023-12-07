set -o pipefail
(pushd dsp && cargo build; r=$?; popd && exit $r) && \
  cargo build && \
  (pushd shim; make clean_exes; make all; r=$?; popd; exit $r) && \
  psh

exit

pushd dsp ; cargo clean ; popd
pushd high_pass ; ./cln ; popd
pushd low_pass ; ./cln ; popd
pushd dsp ; cargo build ; popd
pushd high_pass ; ./bld ; popd
pushd low_pass ; ./bld ; popd
exit

echo nope
exit

# NOPE rm -rf high_pass
./gen-patch.sh high_pass
cd high_pass
./g
exit
