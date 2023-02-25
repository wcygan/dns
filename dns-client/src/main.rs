use anyhow::Result;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8081").await?;
    let _ = socket.send_to(b"wcygan.io", "0.0.0.0:8080").await?;
    Ok(())
}
