#!/usr/bin/env bash

array=( "one" "two" "three" )

if [[ -n "$1" ]] && [[ $1 == "test" ]]; then
for ((i=0; i<3; i++)); do
echo "${array[i]}"
done
fi
