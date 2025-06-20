// client/txtViewer/src/enhanced_client.rs

use finalverse_core::*;
use finalverse_protocol::*;
use serde::{Deserialize, Serialize};
use reqwest;
use serde_json;
use std::collections::HashMap;
use tracing::info;
use uuid::Uuid;

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

pub struct EnhancedClient {
    pub player_id: PlayerId,
    pub player_name: String,
    pub service_urls: HashMap<String, String>,
    pub client: reqwest::Client,
    pub current_region: Option<RegionId>,
    pub echo_bonds: HashMap<EchoType, u32>,
    pub position: Coordinates,
}

impl EnhancedClient {
    pub fn new(player_name: String) -> Self {
        let mut service_urls = HashMap::new();
        
        // Check if we're using docker or local development
        let base_url = if std::env::var("USE_DOCKER").is_ok() {
            "http://localhost"
        } else {
            "http://localhost"
        };
        
        service_urls.insert("song".to_string(), format!("{}:3001", base_url));
        service_urls.insert("world".to_string(), format!("{}:3002", base_url));
        service_urls.insert("echo".to_string(), format!("{}:3003", base_url));
        service_urls.insert("ai".to_string(), format!("{}:3004", base_url));
        service_urls.insert("story".to_string(), format!("{}:3005", base_url));
        service_urls.insert("harmony".to_string(), format!("{}:3006", base_url));
        service_urls.insert("asset".to_string(), format!("{}:3007", base_url));
        service_urls.insert("community".to_string(), format!("{}:3008", base_url));
        service_urls.insert("silence".to_string(), format!("{}:3009", base_url));
        service_urls.insert("procedural".to_string(), format!("{}:3010", base_url));
        service_urls.insert("behavior".to_string(), format!("{}:3011", base_url));
        
        let mut echo_bonds = HashMap::new();
        echo_bonds.insert(EchoType::Lumi, 0);
        echo_bonds.insert(EchoType::KAI, 0);
        echo_bonds.insert(EchoType::Terra, 0);
        echo_bonds.insert(EchoType::Ignis, 0);
        
        Self {
            player_id: PlayerId(Uuid::new_v4()),
            player_name,
            service_urls,
            client: reqwest::Client::new(),
            current_region: None,
            echo_bonds,
            position: Coordinates { x: 0.0, y: 0.0, z: 0.0 },
        }
    }
    
    pub async fn view_progression(&self) -> anyhow::Result<()> {
        let response = self.client
            .get(&format!("{}/progression/{}", self.service_urls["harmony"], self.player_id.0))
            .send()
            .await?;
        
        if response.status().is_success() {
            let progression: serde_json::Value = response.json().await?;
            
            println!("\n🌟 Your Progression:");
            println!("   Attunement Tier: {}", progression["attunement_tier"]);
            println!("   Resonance:");
            println!("     - Creative: {}", progression["resonance"]["creative"]);
            println!("     - Exploration: {}", progression["resonance"]["exploration"]);
            println!("     - Restoration: {}", progression["resonance"]["restoration"]);
            println!("   Total Actions: {}", progression["total_actions"]);
            println!("   Unlocked Melodies: {}", progression["unlocked_melodies"].as_array().map(|a| a.len()).unwrap_or(0));
        } else {
            println!("   No progression data yet. Start performing melodies!");
        }
        
        Ok(())
    }
    
    pub async fn view_chronicle(&self) -> anyhow::Result<()> {
        let response = self.client
            .get(&format!("{}/chronicle/{}", self.service_urls["story"], self.player_id.0))
            .send()
            .await?;
        
        if response.status().is_success() {
            let chronicle: serde_json::Value = response.json().await?;
            
            println!("\n📜 Your Chronicle:");
            
            if let Some(legends) = chronicle["legends"].as_array() {
                if legends.is_empty() {
                    println!("   No legends recorded yet. Your story is just beginning!");
                } else {
                    println!("   Legends ({}):", legends.len());
                    for legend in legends.iter().take(5) {
                        println!("   - {} ({})", legend["title"], legend["impact"]);
                    }
                }
            }
            
            if let Some(quest) = chronicle.get("current_quest") {
                if !quest.is_null() {
                    println!("\n   Current Quest: {}", quest["title"]);
                    println!("   {}", quest["description"]);
                }
            }
        } else {
            println!("   Your chronicle has not begun yet.");
        }
        
        Ok(())
    }
    
    pub async fn request_quest(&self) -> anyhow::Result<()> {
        let request = serde_json::json!({
            "player_id": self.player_id.0.to_string(),
            "region": self.current_region.as_ref().map(|r| r.0.to_string()).unwrap_or_else(|| "Terra Nova".to_string()),
        });
        
        let response = self.client
            .post(&format!("{}/quest/generate", self.service_urls["story"]))
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let quest: serde_json::Value = response.json().await?;
            
            println!("\n🎯 New Quest Received!");
            println!("   Title: {}", quest["title"]);
            println!("   {}", quest["description"]);
            println!("   Quest Giver: {}", quest["quest_giver"]);
            println!("   Rewards:");
            println!("     - Creative: +{}", quest["reward_resonance"]["creative"]);
            println!("     - Exploration: +{}", quest["reward_resonance"]["exploration"]);
            println!("     - Restoration: +{}", quest["reward_resonance"]["restoration"]);
        } else {
            println!("❌ Failed to generate quest");
        }
        
        Ok(())
    }
    
    pub async fn view_ecosystem(&self) -> anyhow::Result<()> {
        if let Some(region_id) = &self.current_region {
            let response = self.client
                .get(&format!("{}/regions/{}/ecosystem", self.service_urls["world"], region_id.0))
                .send()
                .await?;
            
            if response.status().is_success() {
                let ecosystem: serde_json::Value = response.json().await?;
                
                println!("\n🌿 Ecosystem Status:");
                println!("   Biodiversity Index: {:.2}", ecosystem["biodiversity_index"].as_f64().unwrap_or(0.0));
                println!("   Creature Population: {}", ecosystem["creature_count"].as_u64().unwrap_or(0));
                println!("   Flora Count: {}", ecosystem["flora_count"].as_u64().unwrap_or(0));
                
                if let Some(creatures) = ecosystem["notable_creatures"].as_array() {
                    println!("\n   Notable Creatures:");
                    for creature in creatures.iter().take(3) {
                        println!("   - {} at ({:.0}, {:.0})", 
                            creature["species"], 
                            creature["x"].as_f64().unwrap_or(0.0),
                            creature["z"].as_f64().unwrap_or(0.0)
                        );
                    }
                }
            }
        } else {
            println!("🌍 Select a region first to view its ecosystem.");
        }
        
        Ok(())
    }
    
    pub async fn perform_advanced_melody(&self, melody_id: &str) -> anyhow::Result<()> {
        // First check if we have this melody unlocked
        let progression_response = self.client
            .get(&format!("{}/melodies/{}", self.service_urls["harmony"], self.player_id.0))
            .send()
            .await?;
        
        if progression_response.status().is_success() {
            let melodies: serde_json::Value = progression_response.json().await?;
            
            let unlocked = melodies["unlocked"].as_array()
                .map(|arr| arr.iter().any(|m| m["id"] == melody_id))
                .unwrap_or(false);
            
            if !unlocked {
                println!("❌ You haven't unlocked the '{}' melody yet!", melody_id);
                println!("   Available melodies to unlock:");
                if let Some(available) = melodies["available_to_unlock"].as_array() {
                    for melody in available {
                        println!("   - {} (requires: C:{} E:{} R:{})", 
                            melody["name"],
                            melody["resonance_requirement"]["creative"],
                            melody["resonance_requirement"]["exploration"],
                            melody["resonance_requirement"]["restoration"]
                        );
                    }
                }
                return Ok(());
            }
        }
        
        // Prepare a simple melody request. The client does not yet construct
        // full melodies, so we send placeholder note data based on the ID.
        let (harmony_type, power) = match melody_id {
            "healing_touch" => ("restoration", 15.0),
            "light_of_hope" => ("exploration", 20.0),
            "forge_of_will" => ("creative", 25.0),
            _ => ("courage", 10.0),
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
            println!("\n🎵 Advanced melody '{}' performed successfully!", melody_id);
        }
        
        Ok(())
    }
    
    pub async fn interact_with_ai_npc(&self, npc_name: &str, emotion: &str) -> anyhow::Result<()> {
        let request = serde_json::json!({
            "context": {
                "npc_name": npc_name,
                "emotion": emotion,
                "player_name": self.player_name,
                "location": "Terra Nova"
            }
        });
        
        let response = self.client
            .post(&format!("{}/npc/dialogue", self.service_urls["ai"]))
            .json(&request)
            .send()
            .await?;
        
        if response.status().is_success() {
            let dialogue: serde_json::Value = response.json().await?;
            
            println!("\n💬 NPC Interaction:");
            println!("   {}", dialogue["dialogue"]);
            println!("   (Emotion: {}, Confidence: {:.2})", 
                dialogue["emotion_detected"],
                dialogue["confidence"].as_f64().unwrap_or(0.0)
            );
        }
        
        Ok(())
    }
    
    pub async fn update_echo_bond(&self, echo_name: &str) -> anyhow::Result<u32> {
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
            let bond_level = result["bond_level"].as_u64().unwrap_or(0) as u32;
            
            // Update local tracking
            let echo_type = match echo_name.to_lowercase().as_str() {
                "lumi" => EchoType::Lumi,
                "kai" => EchoType::KAI,
                "terra" => EchoType::Terra,
                "ignis" => EchoType::Ignis,
                _ => EchoType::Lumi,
            };
            
            Ok(bond_level)
        } else {
            Ok(0)
        }
    }
    
    pub async fn perform_symphony(&self, symphony_type: &str) -> anyhow::Result<()> {
        println!("\n🎼 Attempting to perform {} Symphony...", symphony_type);
        
        // Check if we have the required harmony
        let progression_response = self.client
            .get(&format!("{}/harmonies/{}", self.service_urls["harmony"], self.player_id.0))
            .send()
            .await?;
        
        if progression_response.status().is_success() {
            let harmonies: serde_json::Value = progression_response.json().await?;
            
            if let Some(available) = harmonies["available"].as_array() {
                let has_harmony = available.iter().any(|h| h["id"] == "harmony_of_balance");
                
                if !has_harmony {
                    println!("❌ You need to unlock the Harmony of Balance first!");
                    println!("   Requirements: All Echoes at bond level 50+");
                    return Ok(());
                }
            }
        }
        
        println!("✨ Symphony initiated! This would trigger a server-wide event in the full game.");
        println!("   Players across the world would need to work together to complete it.");
        
        Ok(())
    }
    
    pub async fn view_detailed_stats(&self) -> anyhow::Result<()> {
        println!("\n📊 Detailed Statistics for {}", self.player_name);
        println!("   Player ID: {}", self.player_id.0);
        
        // Get progression
        if let Ok(response) = self.client
            .get(&format!("{}/progression/{}", self.service_urls["harmony"], self.player_id.0))
            .send()
            .await {
            if response.status().is_success() {
                let progression: serde_json::Value = response.json().await?;
                let total_resonance = progression["resonance"]["creative"].as_u64().unwrap_or(0)
                    + progression["resonance"]["exploration"].as_u64().unwrap_or(0)
                    + progression["resonance"]["restoration"].as_u64().unwrap_or(0);
                
                println!("\n   Total Resonance: {}", total_resonance);
                println!("   Actions Performed: {}", progression["total_actions"]);
            }
        }
        
        // Get chronicle stats
        if let Ok(response) = self.client
            .get(&format!("{}/chronicle/{}", self.service_urls["story"], self.player_id.0))
            .send()
            .await {
            if response.status().is_success() {
                let chronicle: serde_json::Value = response.json().await?;
                
                let legend_count = chronicle["legends"].as_array().map(|a| a.len()).unwrap_or(0);
                let quest_count = chronicle["quest_history"].as_array().map(|a| a.len()).unwrap_or(0);
                
                println!("   Legends Recorded: {}", legend_count);
                println!("   Quests Completed: {}", quest_count);
            }
        }
        
        // Get echo bonds
        if let Ok(response) = self.client
            .get(&format!("{}/bonds/{}", self.service_urls["echo"], self.player_id.0))
            .send()
            .await {
            if response.status().is_success() {
                let bonds: serde_json::Value = response.json().await?;
                
                println!("\n   Echo Bonds:");
                if let Some(bond_list) = bonds["bonds"].as_array() {
                    for bond in bond_list {
                        println!("     - {}: {}/100", bond["echo_type"], bond["bond_level"]);
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn move_to(&mut self, x: f64, y: f64, z: f64) {
        self.position = Coordinates { x, y, z };
        println!("\n📍 You moved to ({:.1}, {:.1}, {:.1})", x, y, z);
    }
}
