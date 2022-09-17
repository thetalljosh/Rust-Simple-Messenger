use bytes::BytesMut;
use futures::SinkExt;
use futures::StreamExt;
use tokio::{io::AsyncWriteExt, net::TcpStream};
use std::io;
use tokio_util::codec::{BytesCodec, FramedRead, FramedWrite};
use serde::{Deserialize, Serialize};

const SERVER: &str = "134.126.141.221:5051";

#[derive(Deserialize)]
pub struct LoginModel {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterModel {
    pub username: String,
    pub password: String,
    pub confirm_password: String,
    pub email: String,
}

struct User {
    uuid: String,
    u_name: String,
}

struct Message {
    u_name: String,
    text: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = SERVER.to_string();

    let stdin = FramedRead::new(tokio::io::stdin(), BytesCodec::new());
    let mut stdin = stdin.map(|i| i.map(BytesMut::freeze));
    let mut stdout = FramedWrite::new(tokio::io::stdout(), BytesCodec::new());

    let mut stream = TcpStream::connect(address).await?;
    let (reader, writer) = stream.split();
    let mut sink = FramedWrite::new(writer, BytesCodec::new());
    let mut stream = FramedRead::new(reader, BytesCodec::new());
/*
    println!("Welcome to Rust Messenger");
    println!("Please enter your username");
    let mut u_name = String::new();
    io::stdin().read_line(&mut u_name).expect("failed to readline");
*/
    loop {
        tokio::select! {
            msg = stream.next() => {
                if let Some(Ok(msg)) = msg {
                    
                    stdout.send(msg).await.unwrap();
                } else {
                    break
                }
            },
            input = stdin.next() => {
                if let Some(Ok(input)) = input {
                    sink.send(input).await.unwrap();
                } else {
                    break;
                }
            }
        }
    }
    println!("Done");
    tokio::io::stdout().flush().await.unwrap();
    Ok(())
}