use message::{self, InitiateRequest, TerminateRequest, MessageRequest};
const ROOM: &str = "cross_comm";
fn make_address() -> message::Address {
    let mut addr = message::Address::new();
    addr.set_address("local".to_owned());
    addr
}

fn make_room() -> message::Room {
    let mut room = message::Room::new();
    room.set_name(ROOM.to_owned());
    room
}

pub fn default_initiate() -> InitiateRequest {
    let mut req = InitiateRequest::new();
    req.set_clientAddress(make_address());
    req.set_room(make_room());

    req
}
