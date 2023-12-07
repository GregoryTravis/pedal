set -o pipefail

./bld reso_main 2>&1 | tee out
