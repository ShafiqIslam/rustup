#!/bin/sh

cargo test

cargo test expensive_test -- --show-output --ignored

# only run integration_test file
cargo test --test integration_test

# only run integration tests from tests directory
cargo test --test '*'
