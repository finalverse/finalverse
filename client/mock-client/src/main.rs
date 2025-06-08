// client/mock-client/src/main.rs

use finalverse_common::*;
use finalverse_protocol::*;
use reqwest;
use std::io::{self, Write};
use tracing::info;
use uuid::Uuid;

struct MockClient {
    player_id: PlayerId,
    player_name: String,
    song_engine_url: String,
    world_engine_url: String,
    echo_engine_url: String,
    client: reqwest::Client,
}

impl MockClient {
    fn new(player_name: String) -> Self {
        Self {
            player_id: PlayerId(Uuid::new_v4()),
            player_name,
            song_engine_url: "http://localhost:3001".to_string(),
            world_engine_url: "http://localhost:3002".to_string(),
            echo_engine_url: "http://localhost:3003".to_string(),
            client: reqwest::Client::new(),
        }
    }
    
    async fn check_services(&self) {
        println!("\n🔍 Checking service status...");
        
        // Check Song Engine
        match self.client.get(&format!("{}/info", self.song_engine_url)).send().await {
            Ok(resp) => {
                if let Ok(info) = resp.json::<ServiceInfo>().await {
                    println!("✅ Song Engine: {:?} (uptime: {}s)", info.status, info.uptime_seconds);
                }
            }
            Err(_) => println!("❌ Song Engine: Offline"),
        }
        
        // Check World Engine
        match self.client.get(&format!("{}/info", self.world_engine_url)).send().await {
            Ok(resp) => {
                if let Ok(info) = resp.json::<ServiceInfo>().await {
                    println!("✅ World Engine: {:?} (uptime: {}s)", info.status, info.uptime_seconds);
                }
            }
            Err(_) => println!("❌ World Engine: Offline"),
        }
        
        // Check Echo Engine
        match self.client.get(&format!("{}/info", self.echo_engine_url)).send().await {
            Ok(resp) => {
                if let Ok(info) = resp.json::<ServiceInfo>().await {
                    println!("✅ Echo Engine: {:?} (uptime: {}s)", info.status, info.uptime_seconds);
                }
            }
            Err(_) => println!("❌ Echo Engine: Offline"),
        }
    }
    
    async fn perform_melody(&self, melody_type: &str) -> anyhow::Result<()> {
        let melody = match melody_type {
            "healing" => Melody::Healing { power: 10.0 },
            "creation" => Melody::Creation { pattern: "star".to_string() },
            "discovery" => Melody::Discovery { range: 50.0 },
            "courage" => Melody::Courage { intensity: 15.0 },
            _ => return Err(anyhow::anyhow!("Unknown melody type")),
        };
        
        let request = grpc::PerformMelodyRequest {
            player_id: self.player_id.0.to_string(),
            melody,
            target: Coordinates { x: 100.0, y: 50.0, z: 200.0 },
        };
        
        let response = self.client
            .post(&format!("{}/melody", self.song_engine_url))
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let result: grpc::PerformMelodyResponse = response.json().await?;
            println!("\n🎵 Melody performed!");
            println!("   Harmony changed by: {:.1}", result.harmony_change);
            println!("   Resonance gained - Creative: {}, Exploration: {}, Restoration: {}", 
                result.resonance_gained.creative,
                result.resonance_gained.exploration,
                result.resonance_gained.restoration
            );
        } else {
            println!("❌ Failed to perform melody");
        }
        
        Ok(())
    }
    
    async fn view_world_state(&self) -> anyhow::Result<()> {
        let response = self.client
            .get(&format!("{}/regions", self.world_engine_url))
            .send()
            .await?;
        
        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            println!("\n🌍 World State:");
            println!("   Cosmic Time: {}", data["cosmic_time"]);
            
            if let Some(regions) = data["regions"].as_array() {
                for region in regions {
                    let name = region["name"].as_str().unwrap_or("");
                    let harmony = region["harmony_level"].as_f64().unwrap_or(0.0);
                    let weather = &region["weather"];
                    let active_players = region["active_players"].as_u64().unwrap_or(0);
                    println!("\n   Region: {}", name);
                    println!("   - Harmony: {:.1}%", harmony);
                    println!("   - Weather: {:?}", weather);
                    println!("   - Active Players: {}", active_players);
                }
            }
        }
        
        Ok(())
    }
    
    async fn interact_with_echo(&self, echo_name: &str) -> anyhow::Result<()> {
        let request = serde_json::json!({
            "player_id": self.player_id.0.to_string(),
            "echo_id": echo_name.to_lowercase(),
        });
        
        let response = self.client
            .post(&format!("{}/interact", self.echo_engine_url))
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let result: serde_json::Value = response.json().await?;
            println!("\n✨ Echo Interaction:");
            println!("   {}", result["response"]);
            println!("   Bond Level: {}/100", result["bond_level"]);
        } else {
            println!("❌ Failed to interact with Echo");
        }
        
        Ok(())
    }
    
    async fn view_echo_bonds(&self) -> anyhow::Result<()> {
        let response = self.client
            .get(&format!("{}/bonds/{}", self.echo_engine_url, self.player_id.0))
            .send()
            .await?;
        
        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            println!("\n💫 Your Echo Bonds:");
            
            if let Some(bonds) = data["bonds"].as_array() {
                if bonds.is_empty() {
                    println!("   No bonds formed yet. Try interacting with the Echoes!");
                } else {
                    for bond in bonds {
                        println!("   {} - Level: {}/100", bond["echo_type"], bond["bond_level"]);
                    }
                }
            }
        }
        
        Ok(())
    }
}

fn print_menu() {
    println!("\n========== Finalverse Client ==========");
    println!("1. Check service status");
    println!("2. Perform melody (healing/creation/discovery/courage)");
    println!("3. View world state");
    println!("4. Interact with Echo (lumi/kai/terra/ignis)");
    println!("5. View Echo bonds");
    println!("6. Exit");
    println!("=====================================");
    print!("Choose action: ");
    io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("🌟 Welcome to Finalverse!");
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name)?;
    let player_name = player_name.trim().to_string();
    
    let client = MockClient::new(player_name.clone());
    println!("\nWelcome, {}! Your ID: {}", player_name, client.player_id.0);
    
    loop {
        print_menu();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => client.check_services().await,
            "2" => {
                print!("Enter melody type (healing/creation/discovery/courage): ");
                io::stdout().flush().unwrap();
                let mut melody = String::new();
                io::stdin().read_line(&mut melody)?;
                let _ = client.perform_melody(melody.trim()).await;
            }
            "3" => {
                let _ = client.view_world_state().await;
            }
            "4" => {
                print!("Enter Echo name (lumi/kai/terra/ignis): ");
                io::stdout().flush().unwrap();
                let mut echo = String::new();
                io::stdin().read_line(&mut echo)?;
                let _ = client.interact_with_echo(echo.trim()).await;
            }
            "5" => {
                let _ = client.view_echo_bonds().await;
            }
            "6" => {
                println!("Farewell, Songweaver!");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
    
    Ok(())
}