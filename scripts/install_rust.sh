#!/bin/bash
set -xe



curl https://sh.rustup.rs -sSf > scripts/rust_init.sh

sh ./scripts/rust_init.sh -y



source $HOME/.cargo/env

rustc --version

cargo --version
