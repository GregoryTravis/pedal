pushd dsp ; cargo clean ; popd
pushd high_pass ; ./cln ; popd
pushd dsp ; cargo build ; popd
pushd high_pass ; ./bld ; popd
exit

echo nope
exit

# NOPE rm -rf high_pass
./gen-patch.sh high_pass
cd high_pass
./g
exit
