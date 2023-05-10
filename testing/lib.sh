#!/bin/bash

die() {
  tput setaf 7
  tput setab 1
  echo "$@" >&2
  tput sgr0
  exit 1
}
