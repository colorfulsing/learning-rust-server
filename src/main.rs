use server::Server;

mod http;
mod server;

fn main() {
    let mut server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}