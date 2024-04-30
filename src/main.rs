mod request;

use request::request;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    request("400040").await?;
    Ok(())
}
