#!/bin/bash
# /tmp/pc_py の生成 Python を構文確認し、TODO の数と主要ルートの出力を見る
cd /tmp/pc_py || exit 1
ls
for f in $(find . -name '*.py'); do python3 -m py_compile "$f" 2>&1 | tail -3; done
echo "--- compiled"
grep -c "TODO" $(find . -name '*.py')
grep -n "get_owner\|add_owner" -A7 $(find . -name '*.py') | head -40
