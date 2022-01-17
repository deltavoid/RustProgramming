FROM deltavoid/debian_rust_basic:latest
# FROM debian_rust_basic


ADD ./ /repos/RustProgramming

# RUN cd /repos/RustProgramming && git submodule update --init --recursive

RUN cd /repos/RustProgramming && . ~/.bashrc && ./scripts/build.sh
