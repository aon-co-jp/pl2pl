#!/bin/bash
# 実プロジェクト(Spring PetClinic REST)を変換して結果を見る。第1引数=プロジェクトのパス
export PATH=$HOME/.cargo/bin:/usr/bin:/bin
cd "$(dirname "$0")/.."
CARGO_TARGET_DIR=/tmp/jt_target cargo build --release -q 2>&1 | tail -2
rm -rf /tmp/pc_py
/tmp/jt_target/release/java_to convert "$1" --target ${2:-python-fastapi} --out /tmp/pc_py --name pc 2>&1 | tail -${3:-40}
