set -o pipefail

./bld ahigh_pass_main 2>&1 | tee out
