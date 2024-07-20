use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    stream.write_all(b"set name suhail").await?;

    let mut buf = BytesMut::with_capacity(1024);
    stream.read_buf(&mut buf).await?;

    match std::str::from_utf8(&buf) {
        Ok(val) => {
            if val == "r OK" {
                println!("Key updated with new value");
            } else {
                println!("New key added");
            }
        }
        Err(err) => {
            println!("error: {}", err);
        }
    }

    Ok(())
}
