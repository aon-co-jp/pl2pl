#!/bin/bash
# 生成 Python の TODO を種類ごとに数える
cd /tmp/pc_py || exit 1
grep -h "TODO" *.py | sed 's/^ *//' | cut -c1-110 | sort | uniq -c | sort -rn | head -${1:-25}
