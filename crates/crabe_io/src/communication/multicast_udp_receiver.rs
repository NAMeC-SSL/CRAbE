use crate::constant::BUFFER_SIZE;
use log::error;
use std::io::Cursor;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};
use std::str::FromStr;

/// A struct that handles a Multicast UDP Receiver.
pub struct MulticastUDPReceiver {
    /// The UDP socket that joins the multicast group.
    socket: UdpSocket,
    /// A buffer that is used to receive data from the socket without allocating new heap memory.
    buffer: [u8; BUFFER_SIZE],
}

impl MulticastUDPReceiver {
    /// Creates a new `MulticastUDPReceiver` that joins an IPv4 multicast group.
    ///
    /// # Arguments
    ///
    /// * `ip`: The IP address of the multicast group as a string slice.
    /// * `port`: The port number of the multicast group.
    ///
    /// # Returns
    ///
    /// A new `MulticastUDPReceiver` that is ready to receive data in a non-blocking mode.
    ///
    /// # Errors
    ///
    /// This function will return an `Box<dyn std::error::Error>` if the IP address string cannot be parsed into an IPv4 address, if there is an error while binding the socket, joining the multicast group or setting the socket to non-blocking mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::net::Ipv4Addr;
    /// use crabe_io::communication::MulticastUDPReceiver;
    ///
    /// let receiver = MulticastUDPReceiver::new(Ipv4Addr::new(224,5,23,2), 10020).expect("Failed to create MulticastUDPReceiver");
    /// ```
    ///
    /// This example creates a new `MulticastUDPReceiver` that listens on IP address 224.5.23.2 and port 10020, which is the default grSim vision address and port.
    pub fn new(ip: Ipv4Addr, port: u16) -> Result<Self, Box<dyn std::error::Error>> {
        let socket = UdpSocket::bind(SocketAddrV4::new(ip, port))?;

        socket.join_multicast_v4(&ip, &Ipv4Addr::UNSPECIFIED)?;
        socket.set_nonblocking(true)?;

        Ok(Self {
            socket,
            buffer: [0u8; BUFFER_SIZE],
        })
    }

    /// Attempts to receive a packet of type `T` from the socket and decode it using `prost`.    
    ///
    /// # Returns
    ///
    /// An `Option` that contains the decoded packet if the receive operation is successful, or `None` otherwise.
    ///
    /// # Type Parameters
    ///
    /// * `T`: The type of the packet to decode. It must implement the `prost::Message` and `Default` traits, and should be a struct generated by protobuf files using `prost`.
    pub fn receive<T: prost::Message + Default>(&mut self) -> Option<T> {
        if let Ok(p_size) = self.socket.recv(&mut self.buffer) {
            return match T::decode(Cursor::new(&self.buffer[0..p_size])) {
                Ok(packet) => Some(packet),
                Err(e) => {
                    error!("Decoding of the received packet failed: {}", e);
                    None
                }
            };
        } else {
            None
        }
    }
}
