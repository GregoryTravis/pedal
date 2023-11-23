rm -rf high_pass
./gen-patch.sh high_pass
cd high_pass
./g
exit

(./bld && ./psh) 2>&1 | tee out
