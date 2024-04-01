// server/handler.rs
use tokio::net::TcpListener;

pub async fn run_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server listening on 127.0.0.1:8080");

    loop {
        let (_socket, _) = listener.accept().await.unwrap();
        // Handle client connections and requests
    }
}