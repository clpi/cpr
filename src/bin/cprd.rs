#![allow(clippy::unused_async)]
#![allow(clippy::unimplemented)]

use cpr::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = run().await;
    Ok(())
}
