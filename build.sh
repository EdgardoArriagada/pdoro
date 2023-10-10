#!/bin/bash

set -o errexit
set -o nounset
# set -o xtrace # uncomment for debugging

declare mode=${1:-release}

if [[ "$mode" == "release" ]]; then
  declare targetDir=./target/release
  cargo build --release
else
  declare targetDir=./target/debug
  cargo build
fi

rm -rf ./bin
mkdir ./bin

function isExecutable() {
  [[ -f ${1} && -x ${1} ]]
}

declare fileNames=(`ls ${targetDir}`)

for fileName in "${fileNames[@]}"; do
  declare file=${targetDir}/${fileName}

  if isExecutable ${file}; then
    cp ${file} ./bin
  fi
done

