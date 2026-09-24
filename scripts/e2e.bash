#!/usr/bin/env bash

set -euxo pipefail

action="$1"
test_name="$2"

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
PLUGIN_PATH="${PLUGIN_PATH:-"${repo_root}/target/wasm32-unknown-unknown/debug/dprint_plugin_sh.wasm"}"

cd "${repo_root}/tests/${test_name}"

shopt -s nullglob

target_file=""
for f in expected.* .envrc; do
  target_file="$f"
  break
done

raw_file="../raw.sh"
for f in raw.*; do
  raw_file="$f"
  break
done

case "${action}" in
  check)
    dprint check --plugins="${PLUGIN_PATH}" "${target_file}"
    diff <(cat "${raw_file}" | dprint fmt --stdin "${target_file}" --plugins="${PLUGIN_PATH}") "${target_file}"
    ;;
  bump)
    cat "${raw_file}" | dprint fmt --stdin "${target_file}" --plugins="${PLUGIN_PATH}" > "${target_file}"
    ;;
  *)
    echo "Unknown action: ${action}" >&2
    exit 1
    ;;
esac
