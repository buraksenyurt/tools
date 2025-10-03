/// A simple port scanner implementation in Rust.
/// This module defines a `Scanner` struct that can scan a given IP address for open ports within a specified timeout duration.

/// Scanner
/// A struct representing a port scanner.
/// It contains the target IP address and the timeout duration for each port scan.
pub struct Scanner{
    pub target: IpAddr,
    pub timeout: Duration,
}

impl Scanner{
    /// Creates a new `Scanner` instance with the specified target IP address and timeout duration.
    pub fn new(target: IpAddr, timeout: Duration) -> Self {
        Scanner { target, timeout }
    }

    /// Scans a specific port on the target IP address.
    pub fn scan_port(&self, port: u16) -> Option<PortInfo> {
        let socket = SocketAddr::new(self.target, port);
        let start = Instant::now();

        match TcpStream::connect_timeout(&socket, self.timeout) {
            Ok(_) => None,
            Err(_) => None,
        }
    }
}