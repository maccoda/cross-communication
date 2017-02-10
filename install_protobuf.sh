#!/bin/bash
set -ex

# Get protobuf
FILE_NAME="protoc-$PROTOBUF_VERS-linux-x86_64"

wget https://github.com/google/protobuf/releases/download/v3.2.0/$FILE_NAME.zip
unzip $FILE_NAME

# Get Rust protobuf stuff
cargo install protobuf
cargo install grpc-compiler
