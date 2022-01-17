#!/bin/bash
set -xe

# source $HOME/.cargo/env
# for docker env
# source ~/.bashrc

(
    cd example/hello_cargo
    cargo build
    cargo run
)