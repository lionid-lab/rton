use tokio::net::{TcpSocket, UdpSocket};

enum SocketType {
    Udp(UdpSocket),
    Tcp(TcpSocket),
}

struct Socket {
    socket: SocketType,
}
