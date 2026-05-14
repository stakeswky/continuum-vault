#!/usr/bin/env bash
set -euo pipefail

# Allowed upstream-file edits: ONLY the `mod continuum;` insertion in src/main.rs.
# Use the merge-base against upstream/main so a pinned release branch does not
# treat later upstream commits as local modifications.
base_ref="${CONTINUUM_UPSTREAM_BASE:-upstream/main}"
base="$(git merge-base HEAD "$base_ref")"

diff="$(git diff "$base"...HEAD -- src/ ':!src/continuum/' ':!src/main.rs')"
if [ -n "$diff" ]; then
  echo "ERROR: edits found outside src/continuum/:"
  echo "$diff" | head -40
  exit 1
fi

mainrs_added="$(git diff "$base"...HEAD -- src/main.rs | grep -c '^+mod continuum;' || true)"
mainrs_other="$(git diff "$base"...HEAD -- src/main.rs | grep -E '^[+-][^+-]' | wc -l | tr -d ' ')"
if [ "$mainrs_added" != "1" ] || [ "$mainrs_other" -gt 2 ]; then
  echo "ERROR: src/main.rs has unexpected modifications"
  git diff "$base"...HEAD -- src/main.rs
  exit 1
fi

echo "ok: upstream untouched."
