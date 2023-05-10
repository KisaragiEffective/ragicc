#!/bin/bash
self_dir="$(dirname "$0")"
. "$self_dir/../lib.sh"
cargo build
find "$(readlink -e "$self_dir/step1")" -name '*.sh' -print0 | xargs -0 -P "$(nproc)" --replace sh '{}'
