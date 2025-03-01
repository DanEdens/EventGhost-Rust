use std::time::Duration;
use eventghost::core::{GlobalsStore, GlobalsConfig, GlobalsBackendType, GlobalValue};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // Choose which backend to use
    let args: Vec<String> = std::env::args().collect();
    let backend_type = if args.len() > 1 {
        match args[1].as_str() {
            "mqtt" => {
                #[cfg(feature = "globals_mqtt")]
                {
                    println!("Using MQTT backend");
                    let broker = args.get(2).map(|s| s.as_str()).unwrap_or("localhost");
                    let mut config = GlobalsConfig {
                        backend_type: GlobalsBackendType::Mqtt,
                        mqtt_broker: Some(broker.to_string()),
                        ..Default::default()
                    };
                    config
                }
                #[cfg(not(feature = "globals_mqtt"))]
                {
                    println!("MQTT support not enabled, using local backend");
                    GlobalsConfig::default()
                }
            },
            "redis" => {
                #[cfg(feature = "globals_redis")]
                {
                    println!("Using Redis backend");
                    let url = args.get(2).map(|s| s.as_str()).unwrap_or("redis://localhost");
                    let mut config = GlobalsConfig {
                        backend_type: GlobalsBackendType::Redis,
                        redis_url: Some(url.to_string()),
                        ..Default::default()
                    };
                    config
                }
                #[cfg(not(feature = "globals_redis"))]
                {
                    println!("Redis support not enabled, using local backend");
                    GlobalsConfig::default()
                }
            },
            _ => {
                println!("Using local backend");
                GlobalsConfig::default()
            }
        }
    } else {
        println!("Using local backend");
        GlobalsConfig::default()
    };
    
    // Create a globals store
    let globals = GlobalsStore::new(backend_type).await?;
    
    // String variable
    globals.set_string("greeting", "Hello, EventGhost!".to_string()).await?;
    let greeting = globals.get_string("greeting").await?;
    println!("Greeting: {}", greeting);
    
    // Integer variable
    globals.set_integer("count", 42).await?;
    let count = globals.get_integer("count").await?;
    println!("Count: {}", count);
    
    // Float variable
    globals.set_float("pi", 3.14159).await?;
    let pi = globals.get_float("pi").await?;
    println!("Pi: {}", pi);
    
    // Boolean variable
    globals.set_boolean("enabled", true).await?;
    let enabled = globals.get_boolean("enabled").await?;
    println!("Enabled: {}", enabled);
    
    // JSON variable
    #[derive(serde::Serialize, serde::Deserialize, Debug)]
    struct User {
        name: String,
        age: u32,
    }
    
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };
    
    globals.set_json("user", &user).await?;
    let retrieved_user: User = globals.get_json("user").await?;
    println!("User: {:?}", retrieved_user);
    
    // Subscribe to changes
    println!("Setting up subscription for changes to 'counter'...");
    globals.subscribe("counter", |key, value| {
        if let Some(i) = value.as_integer() {
            println!("Received update for '{}': {}", key, i);
        }
    }).await?;
    
    // Create a counter and increment it
    for i in 1..=5 {
        println!("Setting counter to {}", i);
        globals.set_integer("counter", i).await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    
    // Check if a key exists
    let exists = globals.exists("counter").await?;
    println!("Counter exists: {}", exists);
    
    // Delete a key
    globals.delete("counter").await?;
    let exists = globals.exists("counter").await?;
    println!("Counter exists after deletion: {}", exists);
    
    // Clean up
    globals.shutdown().await?;
    
    println!("Example completed successfully");
    Ok(())
} 