use salvo::prelude::*;

#[handler]
async fn hello_world() -> &'static str {
    "Hello from Salvo"
}

#[tokio::main]
async fn main() {
    let server_addr = "127.0.0.1:7878";
    println!("Starting Salvo Server at https://{}", server_addr);
    let router = Router::new().get(hello_world);
    Server::new(TcpListener::bind(server_addr)).serve(router).await;
}