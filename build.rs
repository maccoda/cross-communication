//! Build file to generate the protocol

use std::process::Command;
fn main() {
    let workspaces = vec!["chat-client", "chat-server"];
    Command::new("git")
        .arg("submodule")
        .arg("sync")
        .arg("--recursive")
        .spawn()
        .expect("Failed to sync submodules");

    Command::new("git")
        .arg("submodule")
        .arg("update")
        .arg("--init")
        .arg("--recursive")
        .spawn()
        .expect("Failed to update submodules");

    // Now to generate code
    for work in &workspaces {
        Command::new("protoc")
            .arg(format!("--rust_out={}/src", work))
            .arg("comms-spec/message.proto")
            .spawn()
            .expect("Failed to generate protobuf code");

        Command::new("protoc")
            .arg(format!("--rust-grpc_out={}/src", work))
            .arg("comms-spec/message.proto")
            .spawn()
            .expect("Failed to generate grpc code");
    }
}
