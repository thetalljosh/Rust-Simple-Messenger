use uuid::Uuid;
use std::collections::HashMap;
use tokio::{
    io::{AsyncWriteExt, AsyncBufReadExt, BufReader},
    net::TcpListener,
    sync::broadcast,
    };

#[derive(Clone)]
pub struct Server<String>{
    svr_addr: String,
}

impl Server<String>{
    pub fn new(svr_addr: String) -> Self {
        Server{
            svr_addr,
        }
        }
    


    pub async fn run(&self) {
    let bind_addr = self.svr_addr.clone();
    let listener = TcpListener::bind(bind_addr).await.unwrap(); // set up TCP listener

    let (tx, _rx) = broadcast::channel(10); //puts a sender and receiver on each client task
    
    loop{
        let (mut socket, addr) = listener.accept().await.unwrap(); // set up socket
        let tx = tx.clone(); //get tx inside the loop 
        let mut rx = tx.subscribe(); // creates new receivers - the API for Tokio requires creating new receivers from transmitter subscribe

        tokio::spawn(async move  {                      

            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader); // create bufreader - wraps the reader and maintains its own buffer
            let mut line = String::new();
            
        loop {

            tokio::select!{                               // tokio select macro - allows running multiple async operations concurrently and act on results FIFO - useful for when a fixed number of things need to run concurrently within the scope of a task and with a shared state
                result = reader.read_line(&mut line) => { // identifier = future { runs block of code}
                    if result.unwrap() == 0 {
                        break;
                    }

                    tx.send((line.clone(), addr)).unwrap();
                    line.clear();
                    let userid = Uuid::new_v4();
                }
                result = rx.recv() => {
                    let (msg, other_addr) = result.unwrap();

                    if addr != other_addr {                            // don't send msg back to the address that sent it, only send to other addr
                    
                    writer.write_all(& msg.as_bytes()).await.unwrap(); // write all input buffer content to output buffer
                    }
                }
        }   
    
    }});
 }

}
}