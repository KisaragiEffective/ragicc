#!/bin/bash
# auto-generated; please do not modify manually
self_dir="$(dirname "$0")"
. "$self_dir/../../lib.sh"
s="$(mktemp)"
mv "$s" "$s.s"
s="$s.s"
e="$(mktemp)"
# avoid 'missing .note.GNU-stack section implies executable stack'
if cargo run -- --source "60" > "$s" 2>/dev/null; then
    cc -o "$e" "$s" -Wa,--noexecstack || die "$0: cc failed"
else
    die "$0: cargo was failed"
fi

"$e"

# shellcheck disable=SC2181
if [ "$?" -eq 60 ]; then
  exit 60
else
  die "$0: $? != 60"
fi
rm "$s"
rm "$e"
