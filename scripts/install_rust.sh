#!/bin/bash
set -xe



curl https://sh.rustup.rs -sSf | sh


source $HOME/.cargo/env

rustc --version

cargo --version
