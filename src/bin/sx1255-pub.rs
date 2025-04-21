use std::error::Error;
use zeromq::prelude::*;

#[async_helpers::main]
fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting server...");
    let mut socket = zeromq::PubSocket::new();
    socket.bind("tcp://*:17017")?;
    println!("Start sending loop");
    loop {
        // read 5000 bytes from the audio source
        let buf: [u8; 5000] = [0; 5000];  
        let mut m: ZmqMessage = ZmqMessage::from(*stock);
        m.push_back(bytes);
        println!("Sending: {:?}", m);
        socket.send(m)?;
    }
}
