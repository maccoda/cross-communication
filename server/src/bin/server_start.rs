extern crate chat_server;

use std::thread;

#[allow(unused_variables)]
fn main() {
    // Create the server, need unused variable so doesn't get disposed of
    let server = chat_server::Server::new().start();
    println!("Server started");
    loop {
        thread::park();
    }
}
