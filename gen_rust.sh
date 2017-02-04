#/bin/bash

# Generate the rust files associated with the protocol buffers located
# in the directory *proto*

dest_dir="."
proto_dir="proto"
protoc --rust_out=$dest_dir $proto_dir/message.proto
protoc --rust-grpc_out=$dest_dir $proto_dir/message.proto

cp -vt chat-client/src/ message.rs message_grpc.rs
cp -vt chat-server/src/ message.rs message_grpc.rs

rm message.rs message_grpc.rs