#!/bin/sh
# spec: docs/repo/graphify.md
command -v graphify >/dev/null 2>&1 || exit 0
[ -f graphify-out/graph.json ] || exit 0
# All event workers request a pass. The lock owner drains requests arriving during a rebuild.
lock=graphify-out/.update.lock
pending=graphify-out/.update-pending
: > "$pending" || exit 0
(
  while [ -f "$pending" ]; do
    mkdir "$lock" 2>/dev/null || exit 0
    trap 'rmdir "$lock" 2>/dev/null' 0
    rm -f "$pending"
    if GRAPHIFY_BACKEND=none graphify update . && [ -x bin/doc-links ]; then
      bin/doc-links graph
    fi
    rmdir "$lock" || exit 0
    trap - 0
  done
) </dev/null >/dev/null 2>&1 &
exit 0
