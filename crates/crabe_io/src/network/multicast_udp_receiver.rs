use crate::constants::BUFFER_SIZE;
use std::io::Cursor;
use std::net::{Ipv4Addr, UdpSocket};
use std::str::FromStr;

/// A struct that handles a Multicast UDP Receiver.
pub struct MulticastUDPReceiver {
    /// The UDP socket that joins the multicast group.
    socket: UdpSocket,
    /// A buffer that is used to receive data from the socket without allocating new heap memory.
    buffer: [u8; BUFFER_SIZE],
}

impl MulticastUDPReceiver {
    /// Creates a new Multicast UDP Receiver that joins the IPv4 multicast group.
    ///
    /// # Arguments
    ///
    /// * `ip`: The IP address of the Multicast UDP Receiver, in the form of a str slice.
    /// * `port`: The port number of the Multicast UDP Receiver.
    ///
    /// # Returns
    ///
    /// A new Multicast UDP Receiver that is ready to receive data.
    ///
    /// # Example
    ///
    /// ```
    /// use crabe_io::MulticastReceiver;
    ///
    /// let receiver = MulticastReceiver::new("224.5.23.2", 10020);
    /// ```
    ///
    /// This example creates a new Multicast UDP Receiver that listens on IP address 224.5.23.2 and port 10020.
    pub fn new(ip: &str, port: u32) -> Self {
        let ipv4 = Ipv4Addr::from_str(ip).expect("TODO: Failed to parse vision server ip");
        let socket =
            UdpSocket::bind(format!("{}:{}", ip, port)).expect("Failed to bind the UDP Socket");
        socket
            .join_multicast_v4(&ipv4, &Ipv4Addr::UNSPECIFIED)
            .expect("Error to join multicast group");
        socket
            .set_nonblocking(true)
            .expect("Failed to set non blocking");

        Self {
            socket,
            buffer: [0u8; BUFFER_SIZE],
        }
    }

    /// Creates a new Multicast UDP Receiver that joins the IPv4 multicast group.
    ///
    /// # Arguments
    ///
    /// * `ip`: The IP address of the Multicast UDP Receiver, in the form of a str slice.
    /// * `port`: The port number of the Multicast UDP Receiver.
    ///
    /// # Returns
    ///
    /// A new Multicast UDP Receiver that is ready to receive data.
    pub fn receive<T: prost::Message + Default>(&mut self) -> Option<T> {
        if let Ok(p_size) = self.socket.recv(&mut self.buffer) {
            let packet = T::decode(Cursor::new(&self.buffer[0..p_size]))
                .expect("Error - Decoding the packet");
            Some(packet)
        } else {
            None
        }
    }
}
