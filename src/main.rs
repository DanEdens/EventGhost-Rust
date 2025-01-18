mod core;
mod eg;

use crate::core::Error;
use crate::eg::EventGhost;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut eg = EventGhost::new().await?;
    
    // Initialize and start
    eg.start().await?;
    
    // Main event loop
    loop {
        if eg.should_stop().await {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    
    // Cleanup
    eg.stop().await?;
    Ok(())
} 