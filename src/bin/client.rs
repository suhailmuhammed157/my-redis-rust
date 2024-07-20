use bytes::BytesMut;
use clap::{Parser, Subcommand};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Get { key: String },
    Set { key: String, value: String },
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let args = Cli::parse();
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    match args.command {
        Command::Set { key, value } => {
            let cmd = format!("set {} {}", key, value);
            stream.write_all(cmd.as_bytes()).await?;

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
        }
        Command::Get { key } => {
            let cmd = format!("get {}", key);

            stream.write_all(cmd.as_bytes()).await?;

            let mut buf = BytesMut::with_capacity(1024);
            stream.read_buf(&mut buf).await?;

            match std::str::from_utf8(&mut buf) {
                Ok(val) => {
                    println!("{}", val);
                }
                Err(err) => {
                    println!("error: {}", err);
                }
            }
        }
    }

    Ok(())
}
