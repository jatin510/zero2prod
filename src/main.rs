use std::io::Error;
use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    // let tcp_listener: TcpListener = "127.0.0.1:8080";
    let tcp_listener = TcpListener::bind("127.0.0.1:80")?;
    let _ = run(tcp_listener).await?;

    Ok(())
}