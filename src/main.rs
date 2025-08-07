use std::io;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream, UdpSocket};
use tracing::{Level, error, info, span};

const BIND_PORT: u16 = 2000;
const BIND_IP_ADDRESS: &str = "0.0.0.0";

struct DnsUdpMessage {
    length: usize,
    data: [u8; 512], // RFC 1035 specifies a maximum UDP datagram size of 512 bytes
    remote_address: SocketAddr,
}

#[tracing::instrument(skip_all, fields(remote_address = message.remote_address.to_string()))]
async fn handle_udp(socket: &UdpSocket, message: DnsUdpMessage) -> io::Result<()> {
    Ok(())
}

#[tracing::instrument(skip(stream))]
async fn handle_tcp(stream: &mut TcpStream, remote_address: SocketAddr) -> io::Result<()> {
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();

    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    let bind_address = format!("{BIND_IP_ADDRESS}:{BIND_PORT}")
        .parse::<SocketAddr>()
        .unwrap();

    let udp_listener = tokio::spawn({
        let _span = span!(Level::INFO, "udp_listener").entered();

        let udp_socket = UdpSocket::bind(bind_address).await?;
        info!("Listening on {} (UDP)", bind_address);

        async move {
            loop {
                let mut data = [0u8; 512];
                let Ok((length, remote_address)) = udp_socket.recv_from(&mut data).await else {
                    continue;
                };
                let dns_udp_message = DnsUdpMessage { length, data, remote_address, };
                match handle_udp(&udp_socket, dns_udp_message).await {
                    Ok(_) => info!("Handled UDP request from {}", remote_address),
                    Err(error) => error!("Failed to handle UDP request: {}", error),
                }
            }
        }
    });

    let tcp_listener = tokio::spawn({
        let _span = span!(Level::INFO, "tcp_listener").entered();

        let tcp_socket = TcpListener::bind(bind_address).await?;
        info!("Listening on {} (TCP)", bind_address);

        async move {
            loop {
                let Ok((mut stream, remote_address)) = tcp_socket.accept().await else {
                    continue;
                };
                match handle_tcp(&mut stream, remote_address).await {
                    Ok(_) => info!("Handled TCP request from {}", remote_address),
                    Err(error) => error!("Failed to handle TCP request: {}", error),
                }
            }
        }
    });

    udp_listener.await?;
    tcp_listener.await?;

    Ok(())
}
