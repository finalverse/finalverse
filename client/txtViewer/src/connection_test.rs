// client/txtViewer/src/connection_test.rs
// A simple test to verify direct connections work

use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing direct connections to services...\n");
    
    let services = vec![
        ("Song Engine", "http://localhost:3001/info"),
        ("World Engine", "http://localhost:3002/info"),
        ("Echo Engine", "http://localhost:3003/info"),
        ("AI Orchestra", "http://localhost:3004/info"),
        ("Story Engine", "http://localhost:3005/info"),
        ("Harmony Service", "http://localhost:3006/info"),
    ];
    
    let client = reqwest::Client::new();
    
    for (name, url) in services {
        match client.get(url).send().await {
            Ok(response) => {
                println!("✅ {} - Status: {}", name, response.status());
                if let Ok(body) = response.text().await {
                    println!("   Response: {}", body);
                }
            }
            Err(e) => {
                println!("❌ {} - Error: {}", name, e);
            }
        }
        println!();
    }
    
    // Test a specific endpoint
    println!("Testing World Engine regions endpoint:");
    match client.get("http://localhost:3002/regions").send().await {
        Ok(response) => {
            println!("✅ Status: {}", response.status());
            if let Ok(body) = response.text().await {
                println!("   Regions: {}", body);
            }
        }
        Err(e) => {
            println!("❌ Error: {}", e);
        }
    }
    
    Ok(())
}