#!/usr/bin/env bash
set -ex

cd "$(dirname "$0")"

docker build -t nexislabs/rust .

read -r rustc version _ < <(docker run nexislabs/rust rustc --version)
[[ $rustc = rustc ]]
docker tag nexislabs/rust:latest nexislabs/rust:"$version"
docker push nexislabs/rust:"$version"
docker push nexislabs/rust:latest
