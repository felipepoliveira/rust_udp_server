mod net;

use std::{io};

use net::udp::one_to_many_server::IncomingMessage;


#[tokio::main]
async fn main() -> io::Result<()> {

    let mut udp_server = net::udp::one_to_many_server::OneToManyUdpServer::bind("0.0.0.0:8080").await?;
    
    // Define the callback closure with 'static and Send bounds
    let callback = |msg: IncomingMessage| {
        println!("Received {} bytes", msg.length)
    };

    // Set the callback function for handling incoming messages
    udp_server.on_incoming_message(Box::new(callback));

    // Start listening for incoming messages
    udp_server.listen().await?;

    Ok(())

}
