#!/usr/bin/env bash
# PostToolUse hook (Edit|Write|MultiEdit): after a .rs edit, format the file,
# then type-check the crate and its tests. Exit 2 sends the compiler output
# back to Claude. Other files are checked at commit time by `vp staged`.
set -uo pipefail

file=$(node -e 'let s="";process.stdin.on("data",d=>s+=d).on("end",()=>{try{process.stdout.write(JSON.parse(s).tool_input?.file_path??"")}catch{}})')
[[ "$file" == *.rs ]] || exit 0

root="${CLAUDE_PROJECT_DIR:-$(git rev-parse --show-toplevel)}"
cd "$root" || exit 0
rel="${file#"$root"/}"

if ! out=$(mise exec -- rustfmt --edition 2024 "$file" 2>&1); then
  printf 'rustfmt failed on %s (syntax error?):\n%s\n' "$rel" "$out" >&2
  exit 2
fi
if ! out=$(mise exec -- cargo check --all-targets --quiet --message-format short 2>&1); then
  printf 'Formatted %s, but the crate does not compile:\n%s\n' "$rel" "$out" >&2
  exit 2
fi
exit 0
