#!/bin/bash

die() {
  tput setaf 7
  tput setab 1
  echo "$@" >&2
  tput sgr0
  exit 1
}

simple_assert() {
  phase="$1"
  source="$2"
  outcome="$3"
  s="$(mktemp)"
  mv "$s" "$s.s"
  s="$s.s"
  e="$(mktemp)"
  # avoid 'missing .note.GNU-stack section implies executable stack'
  if cargo run -- --source "$source" > "$s" 2>/dev/null; then
      cc -o "$e" "$s" -Wa,--noexecstack || die "[$phase] test: cc failed"
  else
      die "[$phase] test: cargo was failed"
  fi

  "$e"
  exit_code="$?"
  rm "$s" 2>/dev/null || true
  rm "$e" 2>/dev/null || true
  if [ "$exit_code" -eq "$outcome" ]; then
    :
  else
    die "[$phase] evaluate: $2 = $exit_code != $outcome"
  fi
}
