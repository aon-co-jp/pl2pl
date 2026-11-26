#!/bin/sh
# pl2pl c2cpp / cpp2c / rust2c の出力を gcc・g++・rustc で実際にビルド・実行し、元の出力と一致するか確かめる(WSL/Linux で実行)。
# c2f / rust2f は gfortran がある時だけ比較する(無ければ「未確認」と表示)。
# 使い方: sh tests/xlang-check.sh <pl2pl の出力ディレクトリ>  (出力は c2f c2cpp cpp2c rust2c rust2f の下位フォルダ)
set -e
S=$(dirname "$0")/samples/xlang; O=$1; T=$(mktemp -d); ng=0
cmp_out() { if [ "$2" = "$3" ]; then echo "一致: $1"; else echo "不一致: $1"; echo "--- 元"; echo "$2"; echo "--- 変換後"; echo "$3"; ng=1; fi; }
gcc -std=c11 -Wall -o $T/c1 $S/c1.c; A=$($T/c1)
g++ -Wall -o $T/c1pp $O/c2cpp/c1.cpp; cmp_out "c2cpp (gcc ↔ g++)" "$A" "$($T/c1pp)"
g++ -Wall -o $T/cpp1 $S/cpp1.cpp; B=$($T/cpp1)
gcc -std=c11 -Wall -o $T/cpp1c $O/cpp2c/cpp1.c; cmp_out "cpp2c (g++ ↔ gcc)" "$B" "$($T/cpp1c)"
rustc -O -o $T/r1 $S/r1.rs; R=$($T/r1)
gcc -std=c11 -Wall -o $T/r1c $O/rust2c/r1.c; cmp_out "rust2c (rustc ↔ gcc)" "$R" "$($T/r1c)"
if command -v gfortran >/dev/null; then
  gfortran -o $T/c1f $O/c2f/c1.f90; cmp_out "c2f (gcc ↔ gfortran)" "$A" "$($T/c1f)"
  gfortran -o $T/r1f $O/rust2f/r1.f90; cmp_out "rust2f (rustc ↔ gfortran)" "$R" "$($T/r1f)"
else
  echo "未確認: c2f / rust2f(gfortran がありません)"
fi
rm -rf $T; exit $ng
