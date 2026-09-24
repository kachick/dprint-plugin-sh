#!/usr/bin/env bash

foo() {
  echo "formatting test"
  case "$1" in
  abc)
    echo "abc"
    ;;
  *)
    echo "default"
    ;;
  esac
}

if [ "$1" = "hello" ] &&
  [ "$2" = "world" ]; then
  foo "hello" >/dev/null
fi
