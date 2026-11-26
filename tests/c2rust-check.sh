#!/bin/bash
# pl2pl c2rust の出力を gcc と rustc で比べる(WSL)。使い方: bash tests/c2rust-check.sh [サンプル.c ...]
set -u
export PATH="$HOME/.cargo/bin:$PATH"
cd "$(dirname "$0")/.."
B=${PL2PL_BIN:-target/debug/pl2pl}
files=("$@"); [ ${#files[@]} -eq 0 ] && files=(tests/samples/c2rust/*.c)
rc=0
for f in "${files[@]}"; do
  n=$(basename "$f" .c); d=/tmp/c2r-$n; mkdir -p $d
  gcc -o $d/c.out "$f" -lm 2>$d/gcc.err || { echo "NG gcc $n"; cat $d/gcc.err; rc=1; continue; }
  $B c2rust "$f" --out $d/rs >/dev/null
  r=$(ls $d/rs/*.rs | head -1)
  rustc -o $d/r.out "$r" 2>$d/rustc.err || { echo "NG rustc $n"; head -20 $d/rustc.err; rc=1; continue; }
  (cd $d && ./c.out >c.txt; ./r.out >r.txt)
  if diff -q $d/c.txt $d/r.txt >/dev/null; then echo "OK $n ($(wc -l <$d/c.txt) lines)"; else echo "DIFF $n"; diff $d/c.txt $d/r.txt | head; rc=1; fi
done
exit $rc
