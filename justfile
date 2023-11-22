#!/usr/bin/env just --justfile

build:
    cd libsixel && cargo build --release
    sudo cp libsixel/target/release/liblibsixel.so /usr/lib/liblibsixel.so
    cd cmd/icat && CGO_ENABLED=1 go install .

update:
  go get -u
  go mod tidy -v