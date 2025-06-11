// client/txtViewer/src/main.rs - Final text-based viewer

//mod enhanced_client;
pub mod enhanced_client;

use enhanced_client::EnhancedClient;
use fv_common::*;
use serde::{Serialize, Deserialize};
use finalverse_protocol::*;
use std::io::{self, Write};
use tracing::info;
use crossterm::{execute, cursor::MoveTo, terminal::{Clear, ClearType}};

#[derive(Serialize)]
struct NoteRequest {
    frequency: f32,
    duration: f32,
    intensity: f32,
}

#[derive(Serialize)]
struct MelodyRequest {
    notes: Vec<NoteRequest>,
    tempo: f32,
    harmony_type: String,
}

#[derive(Serialize)]
struct CoordinatesRequest {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize)]
struct PerformMelodyRequest {
    player_id: String,
    melody: MelodyRequest,
    target_location: CoordinatesRequest,
}

#[derive(Deserialize)]
struct PerformMelodyResponse {
    success: bool,
    resonance_gained: f32,
    harmony_impact: f32,
    message: String,
    effects: Vec<String>,
}

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
    println!("║ 12. Select/Change region               ║");
    println!("║ 13. Move to coordinates                ║");
    println!("║                                        ║");
    println!("║ 0. Exit                                ║");
    println!("╚════════════════════════════════════════╝");
    print!("Choose action: ");
    io::stdout().flush().unwrap();
}

fn print_status(client: &EnhancedClient) {
    println!("╔════════ Player Status ═════════╗");
    println!("Name: {}", client.player_name);
    println!("Location: ({:.1}, {:.1}, {:.1})", client.position.x, client.position.y, client.position.z);
    if let Some(region) = &client.current_region {
        println!("Region: {}", region.0);
    } else {
        println!("Region: Unknown");
    }
    println!("Echo Bonds: L:{} K:{} T:{} I:{}", 
        client.echo_bonds.get(&EchoType::Lumi).unwrap_or(&0),
        client.echo_bonds.get(&EchoType::KAI).unwrap_or(&0),
        client.echo_bonds.get(&EchoType::Terra).unwrap_or(&0),
        client.echo_bonds.get(&EchoType::Ignis).unwrap_or(&0));
    println!("╚════════════════════════════════╝\n");
}

async fn select_region(client: &mut EnhancedClient) -> anyhow::Result<()> {
    let response = match client.client
        .get(&format!("{}/regions", client.service_urls["world"]))
        .send()
        .await {
        Ok(resp) => resp,
        Err(e) => {
            println!("❌ Failed to connect to World Engine: {}", e);
            println!("   Using default region: Terra Nova");
            client.current_region = Some(RegionId(uuid::Uuid::new_v4()));
            return Ok(());
        }
    };
    
    if response.status().is_success() {
        let data: serde_json::Value = response.json().await?;
        
        println!("\n🌍 Available Regions:");
        if let Some(regions) = data["regions"].as_array() {
            if regions.is_empty() {
                println!("   No regions available. Creating default region...");
                client.current_region = Some(RegionId(uuid::Uuid::new_v4()));
                return Ok(());
            }
            
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
                    return Ok(());
                }
            }
            
            // Default to first region if invalid selection
            let region_id = regions[0]["id"].as_str().unwrap();
            client.current_region = Some(RegionId(uuid::Uuid::parse_str(region_id)?));
            println!("✅ Selected default region: {}", regions[0]["name"]);
        }
    } else {
        println!("❌ Failed to get regions. Using default.");
        client.current_region = Some(RegionId(uuid::Uuid::new_v4()));
    }
    
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Simple logging without complex formatting
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(false)
        .init();
    
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
    
    // Check if services are running
    println!("\nChecking services...");
    let services_online = client.check_services_silent().await;
    if !services_online {
        println!("⚠️  Some services are offline. Some features may not work.");
    }
    
    // Try to select initial region
    println!("\nConnecting to the world...");
    if let Err(e) = select_region(&mut client).await {
        println!("⚠️  Could not connect to world: {}", e);
        println!("   Some features will be limited.");
    }
    
    loop {
        execute!(io::stdout(), Clear(ClearType::All), MoveTo(0,0)).unwrap();
        print_status(&client);
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
                if let Err(e) = client.perform_melody(melody.trim()).await {
                    println!("❌ Failed to perform melody: {}", e);
                }
            }
            "3" => {
                if let Err(e) = client.view_world_state().await {
                    println!("❌ Failed to view world state: {}", e);
                }
            }
            "4" => {
                print!("Enter Echo name (lumi/kai/terra/ignis): ");
                io::stdout().flush().unwrap();
                let mut echo = String::new();
                io::stdin().read_line(&mut echo)?;
                if let Err(e) = client.interact_with_echo(echo.trim()).await {
                    println!("❌ Failed to interact with Echo: {}", e);
                }
                
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
                if let Err(e) = client.view_progression().await {
                    println!("❌ Failed to view progression: {}", e);
                }
                if let Err(e) = client.view_detailed_stats().await {
                    println!("❌ Failed to view stats: {}", e);
                }
            }
            "6" => {
                if let Err(e) = client.view_chronicle().await {
                    println!("❌ Failed to view chronicle: {}", e);
                }
            }
            "7" => {
                if let Err(e) = client.request_quest().await {
                    println!("❌ Failed to request quest: {}", e);
                }
            }
            "8" => {
                if let Err(e) = client.view_ecosystem().await {
                    println!("❌ Failed to view ecosystem: {}", e);
                }
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
                
                if let Err(e) = client.interact_with_ai_npc(npc_name.trim(), emotion.trim()).await {
                    println!("❌ Failed to interact with NPC: {}", e);
                }
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
                
                if let Err(e) = client.perform_advanced_melody(melody_id.trim()).await {
                    println!("❌ Failed to perform advanced melody: {}", e);
                }
            }
            "11" => {
                println!("\nAvailable symphonies:");
                println!("  - harmony_of_balance");
                println!("  - song_of_restoration");
                
                print!("Enter symphony type: ");
                io::stdout().flush().unwrap();
                let mut symphony = String::new();
                io::stdin().read_line(&mut symphony)?;
                
                if let Err(e) = client.perform_symphony(symphony.trim()).await {
                    println!("❌ Failed to perform symphony: {}", e);
                }
            }
            "12" => {
                if let Err(e) = select_region(&mut client).await {
                    println!("❌ Failed to change region: {}", e);
                }
            }
            "13" => {
                print!("Enter X Y Z: ");
                io::stdout().flush().unwrap();
                let mut coords = String::new();
                io::stdin().read_line(&mut coords)?;
                let parts: Vec<f64> = coords
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                if parts.len() == 3 {
                    client.move_to(parts[0], parts[1], parts[2]);
                } else {
                    println!("Invalid coordinates");
                }
            }
            "0" => {
                println!("\n✨ May the Song guide your path, {}!", player_name);
                println!("Until we meet again in the Verse...");
                break;
            }
            _ => println!("Invalid option"),
        }
        
        // Auto-save progress (only if harmony service is available)
        if client.current_region.is_some() {
            let _ = client.client
                .post(&format!("{}/grant", client.service_urls.get("harmony").unwrap_or(&"http://localhost:3006".to_string())))
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
use reqwest;
use uuid::Uuid;

impl EnhancedClient {
    pub async fn check_services(&self) {
        println!("\n🔍 Checking service status...");
        
        let services = vec![
            ("Song Engine", "song", "3001"),
            ("World Engine", "world", "3002"),
            ("Echo Engine", "echo", "3003"),
            ("AI Orchestra", "ai", "3004"),
            ("Story Engine", "story", "3005"),
            ("Harmony Service", "harmony", "3006"),
            ("Asset Service", "asset", "3007"),
            ("Community", "community", "3008"),
            ("Silence Service", "silence", "3009"),
            ("Procedural Gen", "procedural", "3010"),
            ("Behavior AI", "behavior", "3011"),
        ];
        
        for (name, key, port) in services {
            let url = self.service_urls.get(key)
                .cloned()
                .unwrap_or_else(|| format!("http://localhost:{}", port));
            
            match self.client.get(&format!("{}/info", url)).send().await {
                Ok(resp) => {
                    if let Ok(info) = resp.json::<ServiceInfo>().await {
                        println!("✅ {}: {:?} (uptime: {}s)", name, info.status, info.uptime_seconds);
                    } else {
                        println!("⚠️  {}: Running but info unavailable", name);
                    }
                }
                Err(_) => println!("❌ {}: Offline", name),
            }
        }
    }
    
    pub async fn check_services_silent(&self) -> bool {
        let mut all_online = true;
        let services = vec![
            ("song", "3001"),
            ("world", "3002"),
            ("echo", "3003"),
            ("ai", "3004"),
            ("story", "3005"),
            ("harmony", "3006"),
            ("asset", "3007"),
            ("community", "3008"),
            ("silence", "3009"),
            ("procedural", "3010"),
            ("behavior", "3011"),
        ];
        
        for (key, port) in services {
            let url = self.service_urls.get(key)
                .cloned()
                .unwrap_or_else(|| format!("http://localhost:{}", port));
            
            match self.client.get(&format!("{}/health", url)).send().await {
                Ok(resp) => {
                    if !resp.status().is_success() {
                        all_online = false;
                    }
                }
                Err(_) => all_online = false,
            }
        }
        
        all_online
    }
    
    pub async fn perform_melody(&self, melody_type: &str) -> anyhow::Result<()> {
        let (harmony_type, power) = match melody_type {
            "healing" => ("restoration", 10.0),
            "creation" => ("creative", 20.0),
            "discovery" => ("exploration", 15.0),
            "courage" => ("courage", 12.0),
            _ => return Err(anyhow::anyhow!("Unknown melody type")),
        };

        let notes = vec![NoteRequest {
            frequency: 440.0,
            duration: power / 10.0,
            intensity: 1.0,
        }];

        let request = PerformMelodyRequest {
            player_id: self.player_id.0.to_string(),
            melody: MelodyRequest {
                notes,
                tempo: 120.0,
                harmony_type: harmony_type.to_string(),
            },
            target_location: CoordinatesRequest { x: 100.0, y: 50.0, z: 200.0 },
        };
        
        let response = self.client
            .post(&format!("{}/melody", self.service_urls["song"]))
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let _result: PerformMelodyResponse = response.json().await?;
            println!("\n🎵 Melody performed!");
        } else {
            return Err(anyhow::anyhow!("Server returned error: {}", response.status()));
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
                    
                    // Check if this is our current region
                    if let Some(current) = &self.current_region {
                        if region["id"].as_str() == Some(&current.0.to_string()) {
                            println!("   📍 You are here!");
                        }
                    }
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
            return Err(anyhow::anyhow!("Server returned error: {}", response.status()));
        }
        
        Ok(())
    }
}