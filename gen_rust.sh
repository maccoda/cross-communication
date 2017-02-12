#/bin/bash
set -ex
# Generate the rust files associated with the protocol buffers located
# in the directory *proto*
# First parameter is the destination directory of the generated files
if [[ -z $1 ]]; then
  dest_dir="."
fi
proto_dir="comms-spec"

# Update the submodule (will go to master)
git submodule sync --recursive
git submodule update --init --recursive

protoc --rust_out=$dest_dir $proto_dir/message.proto
protoc --rust-grpc_out=$dest_dir $proto_dir/message.proto

# Hard coded to ensure only the ones we want are used. Once the spec has
# stabilized will look into some more dynamic stuff
cp -vt chat-client/src/ message.rs message_grpc.rs
cp -vt chat-server/src/ message.rs message_grpc.rs

rm message.rs message_grpc.rs
