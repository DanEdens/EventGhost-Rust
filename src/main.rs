use eg::EventGhost;

mod core;
mod eg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut eg = EventGhost::new();
    eg.initialize()?;
    eg.start()?;
    
    // Main event loop
    loop {
        if eg.globals.read().stop_execution_flag {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    
    eg.stop()?;
    Ok(())
} 