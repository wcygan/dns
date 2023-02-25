use anyhow::Result;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    loop {
        let mut buf = [0; 512];
        let socket = UdpSocket::bind("0.0.0.0:8080").await?;

        let (size, src) = socket.recv_from(&mut buf).await?;
        println!("{}: {}", src, String::from_utf8_lossy(&buf[..size]),);
    }
}
