#!/bin/bash
cd /tmp/pc_py || exit 1
for f in *.py; do python3 -m py_compile "$f" 2>&1 | head -5; done
