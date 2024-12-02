#!/bin/env bash

function violates() {
  local increasing=$1
  local diff=$2
  local abs=$(echo ${diff#-})
  local violates=$(($increasing == 1 && $diff < 0 || \
    $increasing == 0 && $diff > 0 || \
    $abs != 1 && $abs != 2 && $abs != 3))
  echo $violates
}

function isSafe() {
  local array=("$@")
  local increasing=$(("${array[1]}" - "${array[0]}" > 0))
  local len="${#array[@]}"
  for ((i = 0; i < $(($len - 1)); i++)); do
    diff=$((${array[i + 1]} - ${array[i]}))
    if [[ $(violates $increasing $diff) -eq 1 ]]; then
      echo 'unsafe'
      return
    fi
  done
  echo 'safe'
}

function part1() {
  local count=0
  local safe
  while IFS="" read -r p || [ -n "$p" ]; do
    if [[ $(isSafe $p) == 'safe' ]]; then
      count=$(($count + 1))
    fi
  done <02.input
  echo "safe $count"
}

part1
