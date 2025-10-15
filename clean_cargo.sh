#!/usr/bin/env bash

for dir in $(find . -type f -name Cargo.toml -exec dirname {} \;); do
  (cd "$dir" && cargo clean)
done
