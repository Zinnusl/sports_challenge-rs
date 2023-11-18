pub mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

use prost::Message;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = zmq::Context::new();

    let pull_sock = ctx.socket(zmq::SocketType::PUSH)?;
    pull_sock.bind("tcp://127.0.0.1:36601")?;
    let data = proto::Position {
        sensor_id: 1,
        timestamp_usec: 500,
        position: proto::Data3d {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
    };
    let msg = data.encode_to_vec();

    println!("Es wird jetzt geloopet, digger");

    loop {
        let err = pull_sock.send(&msg, 0);

        if err.is_err() {
            println!("Error!");
        } else {
            println!("sent {} bytes", msg.len());
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
