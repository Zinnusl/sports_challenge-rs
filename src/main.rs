pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

use prost::Message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = zmq::Context::new();

    let pull_sock = ctx.socket(zmq::SocketType::PULL).unwrap();
    pull_sock.connect("tcp://127.0.0.1:36601")?;
    let mut msg = zmq::Message::new();

    loop {
        let recv_result = pull_sock.recv(&mut msg, 0);
        if recv_result.is_err() {
            println!("recv error: {:?}", recv_result);
            continue;
        }

        let pos = proto::Position::decode(msg.as_ref()).unwrap();

        println!("{:?}", pos);
    }
}
