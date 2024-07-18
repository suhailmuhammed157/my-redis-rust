use std::{borrow::BorrowMut, io::Error};

use bytes::BytesMut;
use my_redis::{helpers::buffer_to_array, storage::Storage, Command};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    println!("Server file");

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let mut storage = Storage::new();

    loop {
        let (mut socket, _) = listener.accept().await?;
        println!("socket connection accepted {:?}", socket);

        let mut buffer_data = BytesMut::with_capacity(1024);
        socket.read_buf(&mut buffer_data).await?;
        let data = buffer_data.to_ascii_lowercase();
        let commands = buffer_to_array(data);

        let requested_command = Command::get_command(&commands[0]);

        println!("buffer {:?}", requested_command);
        process_query(
            &requested_command,
            &mut storage,
            &commands[1],
            Some(&commands[2]),
            &mut socket,
        )
        .await?;
    }

    Ok(())
}

async fn process_query(
    command: &Command,
    storage: &mut Storage,
    key: &String,
    value: Option<&String>,
    socket: &mut TcpStream,
) -> Result<(), Error> {
    match command {
        Command::Get => Ok(()),
        Command::Set => {
            if let Some(val) = value {
                let response: Result<&str, Error> = storage.add_new_entry(key, val);
                match response {
                    Ok(result) => {
                        socket.write_all(result.as_bytes()).await?;
                        println!("Data inserted:{:?}", storage);
                    }
                    Err(_err) => {
                        socket.write_all("".as_bytes()).await?;
                        println!("Data insertion error");
                    }
                }
                Ok(())
            } else {
                Ok(())
            }
        }
        _ => Ok(()),
    }
}
