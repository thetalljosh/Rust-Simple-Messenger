mod server;
use server::Server;

const SERVER: &str = "134.126.141.221:5051";

#[tokio::main]
async fn main() {
    env_logger::init();
    let init_addr = SERVER.to_string();
    let server = Server::new(init_addr);
    server.run().await;
}