// client/mock-client/src/main.rs - Updated version

mod enhanced_client;

use enhanced_client::EnhancedClient;
use finalverse_common::*;
use finalverse_protocol::*;
use std::io::{self, Write};

fn print_main_menu() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║        🌟 FINALVERSE CLIENT 🌟         ║");
    println!("╠════════════════════════════════════════╣");
    println!("║ BASIC ACTIONS                          ║");
    println!("║ 1. Check service status                ║");
    println!("║ 2. Perform melody                      ║");
    println!("║ 3. View world state                    ║");
    println!("║ 4. Interact with Echo                  ║");
    println!("║                                        ║");
    println!("║ ADVANCED FEATURES                      ║");
    println!("║ 5. View progression & stats            ║");
    println!("║ 6. View chronicle                      ║");
    println!("║ 7. Request personal quest              ║");
    println!("║ 8. View ecosystem                      ║");
    println!("║ 9. Interact with AI NPC                ║");
    println!("║ 10. Perform advanced melody            ║");
    println!("║ 11. Initiate symphony (group event)    ║");
    println!("║                                        ║");
    println!("║ 0. Exit                                ║");
    println!("╚════════════════════════════════════════╝");
    print!("Choose action: ");
    io::stdout().flush().unwrap();
}

async fn select_region(client: &mut EnhancedClient) -> anyhow::Result<()> {
    let response = client.client
        .get(&format!("{}/regions", client.service_urls["world"]))
        .send()
        .await?;
    
    if response.status().is_success() {
        let data: serde_json::Value = response.json().await?;
        
        println!("\n🌍 Available Regions:");
        if let Some(regions) = data["regions"].as_array() {
            for (i, region) in regions.iter().enumerate() {
                println!("{}. {} (Harmony: {:.1}%, Weather: {})",
                    i + 1,
                    region["name"],
                    region["harmony_level"],
                    region["weather"]
                );
            }
            
            print!("\nSelect region (number): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            
            if let Ok(index) = input.trim().parse::<usize>() {
                if index > 0 && index <= regions.len() {
                    let region_id = regions[index - 1]["id"].as_str().unwrap();
                    client.current_region = Some(RegionId(uuid::Uuid::parse_str(region_id)?));
                    println!("✅ Selected region: {}", regions[index - 1]["name"]);
                }
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    println!("╔════════════════════════════════════════╗");
    println!("║     🌟 Welcome to Finalverse! 🌟       ║");
    println!("║  Where Stories Meet Infinite Worlds    ║");
    println!("╚════════════════════════════════════════╝");
    
    print!("\nEnter your name, Songweaver: ");
    io::stdout().flush().unwrap();
    
    let mut player_name = String::new();
    io::stdin().read_line(&mut player_name)?;
    let player_name = player_name.trim().to_string();
    
    let mut client = EnhancedClient::new(player_name.clone());
    println!("\n✨ Welcome, {}!", player_name);
    println!("Your unique ID: {}", client.player_id.0);
    
    // Select initial region
    println!("\nFirst, let's choose your starting region...");
    select_region(&mut client).await?;
    
    loop {
        print_main_menu();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        match input.trim() {
            "1" => {
                client.check_services().await;
            }
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
                
                // Update bond level
                if let Ok(bond_level) = client.update_echo_bond(echo.trim()).await {
                    let echo_type = match echo.trim().to_lowercase().as_str() {
                        "lumi" => EchoType::Lumi,
                        "kai" => EchoType::KAI,
                        "terra" => EchoType::Terra,
                        "ignis" => EchoType::Ignis,
                        _ => EchoType::Lumi,
                    };
                    client.echo_bonds.insert(echo_type, bond_level);
                }
            }
            "5" => {
                let _ = client.view_progression().await;
                let _ = client.view_detailed_stats().await;
            }
            "6" => {
                let _ = client.view_chronicle().await;
            }
            "7" => {
                let _ = client.request_quest().await;
            }
            "8" => {
                let _ = client.view_ecosystem().await;
            }
            "9" => {
                print!("Enter NPC name: ");
                io::stdout().flush().unwrap();
                let mut npc_name = String::new();
                io::stdin().read_line(&mut npc_name)?;
                
                print!("Enter emotion (happy/worried/excited/neutral): ");
                io::stdout().flush().unwrap();
                let mut emotion = String::new();
                io::stdin().read_line(&mut emotion)?;
                
                let _ = client.interact_with_ai_npc(npc_name.trim(), emotion.trim()).await;
            }
            "10" => {
                println!("\nAvailable advanced melodies:");
                println!("  - healing_touch");
                println!("  - light_of_hope (requires Lumi bond 20+)");
                println!("  - forge_of_will (requires Ignis bond 30+)");
                
                print!("Enter melody ID: ");
                io::stdout().flush().unwrap();
                let mut melody_id = String::new();
                io::stdin().read_line(&mut melody_id)?;
                
                let _ = client.perform_advanced_melody(melody_id.trim()).await;
            }
            "11" => {
                println!("\nAvailable symphonies:");
                println!("  - harmony_of_balance");
                println!("  - song_of_restoration");
                
                print!("Enter symphony type: ");
                io::stdout().flush().unwrap();
                let mut symphony = String::new();
                io::stdin().read_line(&mut symphony)?;
                
                let _ = client.perform_symphony(symphony.trim()).await;
            }
            "0" => {
                println!("\n✨ May the Song guide your path, {}!", player_name);
                println!("Until we meet again in the Verse...");
                break;
            }
            _ => println!("Invalid option"),
        }
        
        // Auto-save progress
        if let Some(region_id) = &client.current_region {
            // Grant some resonance for actions
            let _ = client.client
                .post(&format!("{}/grant", client.service_urls["harmony"]))
                .json(&serde_json::json!({
                    "player_id": client.player_id.0.to_string(),
                    "creative": 1,
                    "exploration": 1,
                    "restoration": 1,
                    "echo_bonds": client.echo_bonds.iter().map(|(k, v)| {
                        (format!("{:?}", k).to_lowercase(), v)
                    }).collect::<std::collections::HashMap<_, _>>(),
                }))
                .send()
                .await;
        }
    }
    
    Ok(())
}

// Import the basic client functions

impl EnhancedClient {
    pub async fn check_services(&self) {
        println!("\n🔍 Checking service status...");
        
        let services = vec![
            ("Song Engine", "song"),
            ("World Engine", "world"),
            ("Echo Engine", "echo"),
            ("AI Orchestra", "ai"),
            ("Story Engine", "story"),
            ("Harmony Service", "harmony"),
        ];
        
        for (name, key) in services {
            match self.client.get(&format!("{}/info", self.service_urls[key])).send().await {
                Ok(resp) => {
                    if let Ok(info) = resp.json::<ServiceInfo>().await {
                        println!("✅ {}: {:?} (uptime: {}s)", name, info.status, info.uptime_seconds);
                    }
                }
                Err(_) => println!("❌ {}: Offline", name),
            }
        }
    }
    
    pub async fn perform_melody(&self, melody_type: &str) -> anyhow::Result<()> {
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
            .post(&format!("{}/melody", self.service_urls["song"]))
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
    
    pub async fn view_world_state(&self) -> anyhow::Result<()> {
        let response = self.client
            .get(&format!("{}/regions", self.service_urls["world"]))
            .send()
            .await?;
        
        if response.status().is_success() {
            let data: serde_json::Value = response.json().await?;
            println!("\n🌍 World State:");
            println!("   Cosmic Time: {}", data["cosmic_time"]);
            
            if let Some(regions) = data["regions"].as_array() {
                for region in regions {
                    println!("\n   Region: {}", region["name"]);
                    println!("   - Harmony: {:.1}%", region["harmony_level"]);
                    println!("   - Weather: {:?}", region["weather"]);
                    println!("   - Active Players: {}", region["active_players"]);
                }
            }
        }
        
        Ok(())
    }
    
    pub async fn interact_with_echo(&self, echo_name: &str) -> anyhow::Result<()> {
        let request = serde_json::json!({
            "player_id": self.player_id.0.to_string(),
            "echo_id": echo_name.to_lowercase(),
        });
        
        let response = self.client
            .post(&format!("{}/interact", self.service_urls["echo"]))
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
}