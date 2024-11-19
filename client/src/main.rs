#![forbid(unsafe_code)]

use zbus::{proxy, Connection, Result};

#[proxy(
    interface = "org.bootc1",
    default_service = "org.bootc",
    default_path = "/org/bootc1"
)]

trait Bootc1 {
    async fn status(&self) -> Result<String>;
}

// Although we use `tokio` here, you can use any async runtime of choice.
#[tokio::main]
async fn main() -> Result<()> {
    let connection = Connection::system().await?;

    // `proxy` macro creates `MyGreaterProxy` based on `Notifications` trait.
    let proxy = Bootc1Proxy::new(&connection).await?;
    let reply = proxy.status().await?;
    println!("{reply}");

    Ok(())
}
