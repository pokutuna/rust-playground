#!/bin/bash

set -eo pipefail

NAME=$1
ROOT=$(cd $(dirname "$0") && pwd)

err() {
  echo "Error: $*" >&2
}

if [ -z "$NAME" ]; then
  err "Must give a name"
  exit 1
fi

cd $ROOT
cargo new $NAME
sed -i '' -e "s|- {{ NEXT }}|- [$NAME](./$NAME)\n- {{ NEXT }}|" README.md
