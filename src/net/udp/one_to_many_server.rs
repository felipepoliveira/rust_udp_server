// This file contains logic for One to Many paradigm based UDP server

use std::{io::Result, net::SocketAddr, sync::Arc, thread, time::Duration};
use tokio::{net::{ToSocketAddrs, UdpSocket}, task::{self, JoinHandle}};

/// Function associated with OneToManyUdpServer.send_datagrams_join_handler that manage
/// a async thread that will send datagrams stored in...
/// TODO
async fn send_datagrams_task(_: Arc<UdpSocket>) -> Result<()> { 
    loop {
        thread::sleep(Duration::from_millis(3000));
        println!("Sending datagram");   
    }
    Ok(())
}

/// Function associated with OneToManyUdpServer.read_datagrams_join_handler that manage
/// a async thread that will read datagrams
async fn read_datagrams_task(server_sock: Arc<UdpSocket>, on_incoming_datagram_cb: Option<ArcOfIncomingMessageCallback>) -> Result<()> {
        
    let mut buf = [0; 1024];
    loop {
        let read_result = server_sock.recv_from(&mut buf).await;
        match read_result {
            Ok((len, addr)) => {
                match on_incoming_datagram_cb {
                    Some(ref cb) => {
                        cb.clone()(IncomingMessage {
                            address: addr,
                            datagram: Vec::from(buf),
                            length: len,
                        })
                    },
                    None => (),
                }
            },
            Err(err) => {
                println!("Error {:?}", err.to_string())
            }
        }
    }

    Ok(())
}


type IncomingMessageCallback = Box<dyn Fn(IncomingMessage) + Send + Sync + 'static>;
type ArcOfIncomingMessageCallback = Arc<IncomingMessageCallback>;

pub struct IncomingMessage {
    pub address: SocketAddr,
    pub datagram: Vec<u8>,
    pub length: usize,
}


/// Struct that represents a One to Many paradigm based UDP server
pub struct OneToManyUdpServer {
    /// This callback will be called when a datagram is received by the server
    on_incoming_message: Option<ArcOfIncomingMessageCallback>,

    /// Atomic reference to the server socket. This is a Arc because it is shared between the read/write
    /// threads
    udp_server_socket: Arc<UdpSocket>,
}

/**
 * Default implementation for OneToManyUdpServer
 */
impl OneToManyUdpServer {

    /// Create and bind a UDP server to the given socket address
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<OneToManyUdpServer> {

        let udp_server_socket = Arc::new(UdpSocket::bind(addr).await?);

        Ok(
            OneToManyUdpServer {
                on_incoming_message: None,
                udp_server_socket: udp_server_socket
            }
        )
        
    }

    /// Define the action of the callback when a datagram is received from a client 
    pub fn on_incoming_message(&mut self, cb: IncomingMessageCallback) {
        self.on_incoming_message = Some(Arc::new(cb));
    }
    
    /// Start the server read/write threads to read and send data to the sockets
    /// This function should be used like `OneToManyUdpServer::listen().await` so it can wait for all threads to finish
    pub async fn listen(&self) -> Result<()> {

        // spawn async read/write
        let send_datagrams_task_join_handler = tokio::spawn(send_datagrams_task(self.udp_server_socket.clone()));
        let read_datagrams_task_join_handler = tokio::spawn(read_datagrams_task(self.udp_server_socket.clone(), self.on_incoming_message.clone()));

        // wait for all of them to finish
        send_datagrams_task_join_handler.await?;
        read_datagrams_task_join_handler.await?;

        Ok(())
    }
    
}