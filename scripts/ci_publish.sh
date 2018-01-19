#!/usr/bin/env bash
cargo doc || { echo 'cargo doc failed' ; exit 1; }

cargo package || { echo 'cargo package failed' ; exit 1; }

cargo publish --token "$CARGO_LOGIN_TOKEN" || { echo 'cargo publish failed' ; exit 1; }

exit 0