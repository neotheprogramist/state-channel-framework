use server::start;

mod prove;
mod server;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    start().await?;
    Ok(())
}
