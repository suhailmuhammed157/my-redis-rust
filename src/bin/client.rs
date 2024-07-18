use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"set doo bar").await?;

    Ok(())
}
