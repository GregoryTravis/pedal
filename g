set -o pipefail

./bld high_pass_main 2>&1 | tee out
