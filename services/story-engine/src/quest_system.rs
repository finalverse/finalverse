// services/story-engine/src/quest_system.rs
// Dynamic quest generation and management system

use fv_common::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicQuest {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub quest_type: QuestType,
    pub objectives: Vec<DynamicObjective>,
    pub prerequisites: QuestPrerequisites,
    pub rewards: QuestRewards,
    pub context: QuestContext,
    pub state: QuestState,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestType {
    // Solo quests
    Personal { narrative_weight: f32 },
    Discovery { exploration_focus: bool },
    Restoration { harmony_requirement: f32 },
    
    // Group quests
    Community { min_participants: u32, max_participants: u32 },
    Symphony { required_echoes: Vec<EchoType> },
    WorldEvent { affected_regions: Vec<RegionId> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicObjective {
    pub id: Uuid,
    pub description: String,
    pub objective_type: ObjectiveType,
    pub progress: ObjectiveProgress,
    pub hidden: bool,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveType {
    // Location-based
    ReachLocation { coordinates: Coordinates, radius: f32 },
    ExploreArea { region_id: RegionId, coverage_percent: f32 },
    
    // Interaction-based
    InteractWithEcho { echo_type: EchoType, min_bond_level: u32 },
    TalkToNPC { npc_id: String, dialogue_branch: Option<String> },
    
    // Action-based
    PerformMelody { melody_type: Option<String>, location: Option<Coordinates> },
    RestoreHarmony { region_id: RegionId, target_level: f32 },
    CollectItems { item_type: String, quantity: u32 },
    
    // Social
    GatherPlayers { count: u32, location: Option<Coordinates> },
    TeachMelody { student_count: u32 },
    
    // Time-based
    SurviveTime { duration_seconds: u64, conditions: Vec<String> },
    CompleteBeforeTime { deadline: chrono::DateTime<chrono::Utc> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectiveProgress {
    NotStarted,
    InProgress { current: f32, target: f32 },
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestPrerequisites {
    pub min_resonance: Option<Resonance>,
    pub required_quests: Vec<Uuid>,
    pub required_echo_bonds: HashMap<EchoType, u32>,
    pub region_harmony: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRewards {
    pub resonance: Resonance,
    pub items: Vec<String>,
    pub unlocks: Vec<QuestUnlock>,
    pub narrative_impact: NarrativeImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestUnlock {
    NewMelody { melody_id: String },
    NewRegion { region_id: RegionId },
    EchoAbility { echo_type: EchoType, ability: String },
    StoryChapter { chapter_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeImpact {
    pub world_state_changes: HashMap<String, serde_json::Value>,
    pub relationship_changes: HashMap<String, i32>,
    pub legend_entry: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestContext {
    pub generated_by: QuestGenerator,
    pub narrative_tags: Vec<String>,
    pub difficulty_rating: f32,
    pub estimated_duration: u64, // in minutes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestGenerator {
    System { template_id: String },
    AI { prompt_hash: String, model: String },
    Player { creator_id: PlayerId },
    Echo { echo_type: EchoType },
    WorldEvent { event_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestState {
    Available,
    Active { started_at: chrono::DateTime<chrono::Utc>, participants: Vec<PlayerId> },
    Completed { completed_at: chrono::DateTime<chrono::Utc>, completion_style: CompletionStyle },
    Failed { failed_at: chrono::DateTime<chrono::Utc>, reason: String },
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompletionStyle {
    Perfect, // All objectives including optional
    Standard, // All required objectives
    Creative, // Found alternative solution
    Collaborative, // Completed with help
}

// Quest generation system
pub struct QuestGenerationEngine {
    templates: HashMap<String, QuestTemplate>,
    ai_service_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuestTemplate {
    id: String,
    name: String,
    base_description: String,
    objective_templates: Vec<ObjectiveTemplate>,
    variable_elements: Vec<VariableElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ObjectiveTemplate {
    description_template: String,
    objective_type: String,
    variables: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VariableElement {
    name: String,
    options: Vec<String>,
    affects: Vec<String>,
}

impl QuestGenerationEngine {
    pub fn new(ai_service_url: String) -> Self {
        let mut templates = HashMap::new();
        
        // Add some basic templates
        templates.insert("restoration_basic".to_string(), QuestTemplate {
            id: "restoration_basic".to_string(),
            name: "Basic Restoration Quest".to_string(),
            base_description: "Help restore harmony to a troubled area".to_string(),
            objective_templates: vec![
                ObjectiveTemplate {
                    description_template: "Travel to the affected area".to_string(),
                    objective_type: "reach_location".to_string(),
                    variables: HashMap::new(),
                },
                ObjectiveTemplate {
                    description_template: "Perform healing melodies to restore harmony".to_string(),
                    objective_type: "perform_melody".to_string(),
                    variables: HashMap::new(),
                },
            ],
            variable_elements: vec![
                VariableElement {
                    name: "location".to_string(),
                    options: vec!["grove".to_string(), "lake".to_string(), "ruins".to_string()],
                    affects: vec!["description".to_string(), "objectives".to_string()],
                },
            ],
        });
        
        Self {
            templates,
            ai_service_url,
        }
    }
    
    pub async fn generate_quest(
        &self,
        player_profile: &PlayerProfile,
        context: &GenerationContext,
    ) -> Result<DynamicQuest, String> {
        match context.generation_type {
            GenerationType::Template { template_id } => {
                self.generate_from_template(template_id, player_profile, context).await
            }
            GenerationType::AI { parameters } => {
                self.generate_with_ai(player_profile, context, parameters).await
            }
            GenerationType::Emergent { world_state } => {
                self.generate_emergent_quest(player_profile, world_state).await
            }
        }
    }
    
    async fn generate_from_template(
        &self,
        template_id: &str,
        player_profile: &PlayerProfile,
        context: &GenerationContext,
    ) -> Result<DynamicQuest, String> {
        let template = self.templates.get(template_id)
            .ok_or_else(|| format!("Template {} not found", template_id))?;
        
        // Apply variations based on player profile
        let difficulty_modifier = calculate_difficulty_modifier(player_profile);
        
        let quest = DynamicQuest {
            id: Uuid::new_v4(),
            title: self.generate_title(&template.name, context),
            description: self.customize_description(&template.base_description, context),
            quest_type: QuestType::Personal { narrative_weight: 0.7 },
            objectives: self.generate_objectives(&template.objective_templates, context),
            prerequisites: self.calculate_prerequisites(player_profile),
            rewards: self.calculate_rewards(player_profile, difficulty_modifier),
            context: QuestContext {
                generated_by: QuestGenerator::System { template_id: template_id.to_string() },
                narrative_tags: vec!["restoration".to_string(), "harmony".to_string()],
                difficulty_rating: difficulty_modifier,
                estimated_duration: 30,
            },
            state: QuestState::Available,
            created_at: chrono::Utc::now(),
            expires_at: None,
        };
        
        Ok(quest)
    }
    
    async fn generate_with_ai(
        &self,
        player_profile: &PlayerProfile,
        context: &GenerationContext,
        parameters: &HashMap<String, serde_json::Value>,
    ) -> Result<DynamicQuest, String> {
        // Call AI service to generate quest
        let client = reqwest::Client::new();
        
        let ai_request = serde_json::json!({
            "context": {
                "player_level": player_profile.total_resonance(),
                "region": context.region_id,
                "recent_quests": player_profile.recent_quest_types(),
                "preferred_play_style": player_profile.play_style,
            },
            "parameters": parameters,
        });
        
        let response = client
            .post(&format!("{}/quest/generate", self.ai_service_url))
            .json(&ai_request)
            .send()
            .await
            .map_err(|e| format!("AI service error: {}", e))?;
        
        if !response.status().is_success() {
            return Err("Failed to generate quest with AI".to_string());
        }
        
        let ai_response: serde_json::Value = response.json().await
            .map_err(|e| format!("Failed to parse AI response: {}", e))?;
        
        // Convert AI response to quest
        self.parse_ai_quest(ai_response, player_profile)
    }
    
    async fn generate_emergent_quest(
        &self,
        player_profile: &PlayerProfile,
        world_state: &WorldState,
    ) -> Result<DynamicQuest, String> {
        // Analyze world state for emergent opportunities
        let opportunities = self.analyze_world_state(world_state);
        
        if let Some(opportunity) = opportunities.first() {
            // Create quest based on world conditions
            let quest = match opportunity {
                EmergentOpportunity::HarmonyCrisis { region_id, severity } => {
                    self.create_crisis_quest(region_id, *severity, player_profile)
                }
                EmergentOpportunity::PlayerGathering { location, count } => {
                    self.create_social_quest(location, *count)
                }
                EmergentOpportunity::EchoNeed { echo_type, need } => {
                    self.create_echo_quest(echo_type, need)
                }
            };
            
            Ok(quest)
        } else {
            // Fallback to template generation
            self.generate_from_template("restoration_basic", player_profile, &GenerationContext::default()).await
        }
    }
    
    // Helper methods
    fn generate_title(&self, base_title: &str, context: &GenerationContext) -> String {
        format!("{} - {}", base_title, context.region_name.as_deref().unwrap_or("Unknown Lands"))
    }
    
    fn customize_description(&self, base_desc: &str, context: &GenerationContext) -> String {
        // Add contextual details to description
        format!("{} The {} calls for your aid.", base_desc, context.region_name.as_deref().unwrap_or("region"))
    }
    
    fn generate_objectives(
        &self,
        templates: &[ObjectiveTemplate],
        context: &GenerationContext,
    ) -> Vec<DynamicObjective> {
        templates.iter().map(|template| {
            DynamicObjective {
                id: Uuid::new_v4(),
                description: template.description_template.clone(),
                objective_type: self.parse_objective_type(&template.objective_type, &template.variables),
                progress: ObjectiveProgress::NotStarted,
                hidden: false,
                optional: false,
            }
        }).collect()
    }
    
    fn parse_objective_type(
        &self,
        type_str: &str,
        variables: &HashMap<String, serde_json::Value>,
    ) -> ObjectiveType {
        match type_str {
            "reach_location" => ObjectiveType::ReachLocation {
                coordinates: Coordinates { x: 100.0, y: 50.0, z: 200.0 },
                radius: 50.0,
            },
            "perform_melody" => ObjectiveType::PerformMelody {
                melody_type: variables.get("melody_type").and_then(|v| v.as_str()).map(String::from),
                location: None,
            },
            _ => ObjectiveType::RestoreHarmony {
                region_id: RegionId(Uuid::new_v4()),
                target_level: 75.0,
            },
        }
    }
    
    fn calculate_prerequisites(&self, player_profile: &PlayerProfile) -> QuestPrerequisites {
        QuestPrerequisites {
            min_resonance: Some(Resonance {
                creative: 10,
                exploration: 10,
                restoration: 10,
            }),
            required_quests: vec![],
            required_echo_bonds: HashMap::new(),
            region_harmony: None,
        }
    }
    
    fn calculate_rewards(
        &self,
        player_profile: &PlayerProfile,
        difficulty: f32,
    ) -> QuestRewards {
        let base_reward = 10.0 * difficulty;
        
        QuestRewards {
            resonance: Resonance {
                creative: (base_reward * 1.5) as u64,
                exploration: (base_reward * 1.0) as u64,
                restoration: (base_reward * 2.0) as u64,
            },
            items: vec![],
            unlocks: vec![],
            narrative_impact: NarrativeImpact {
                world_state_changes: HashMap::new(),
                relationship_changes: HashMap::new(),
                legend_entry: Some("Restored harmony to a troubled land".to_string()),
            },
        }
    }
    
    fn parse_ai_quest(
        &self,
        ai_response: serde_json::Value,
        player_profile: &PlayerProfile,
    ) -> Result<DynamicQuest, String> {
        // Parse AI-generated quest data
        let quest_data = ai_response.get("quest")
            .ok_or("No quest data in AI response")?;
        
        Ok(DynamicQuest {
            id: Uuid::new_v4(),
            title: quest_data.get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("AI Generated Quest")
                .to_string(),
            description: quest_data.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("A mysterious quest awaits")
                .to_string(),
            quest_type: QuestType::Personal { narrative_weight: 0.8 },
            objectives: vec![], // TODO: Parse objectives from AI
            prerequisites: self.calculate_prerequisites(player_profile),
            rewards: self.calculate_rewards(player_profile, 1.0),
            context: QuestContext {
                generated_by: QuestGenerator::AI {
                    prompt_hash: "ai_generated".to_string(),
                    model: ai_response.get("model_used")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                },
                narrative_tags: vec!["ai_generated".to_string()],
                difficulty_rating: 1.0,
                estimated_duration: 45,
            },
            state: QuestState::Available,
            created_at: chrono::Utc::now(),
            expires_at: None,
        })
    }
    
    fn analyze_world_state(&self, world_state: &WorldState) -> Vec<EmergentOpportunity> {
        let mut opportunities = Vec::new();
        
        // Check for harmony crises
        for (region_id, harmony) in &world_state.region_harmonies {
            if *harmony < 30.0 {
                opportunities.push(EmergentOpportunity::HarmonyCrisis {
                    region_id: region_id.clone(),
                    severity: calculate_crisis_severity(*harmony),
                });
            }
        }
        
        // Check for player gatherings
        for (location, players) in &world_state.player_concentrations {
            if players.len() >= 5 {
                opportunities.push(EmergentOpportunity::PlayerGathering {
                    location: location.clone(),
                    count: players.len() as u32,
                });
            }
        }
        
        // Check for Echo needs
        for (echo_type, state) in &world_state.echo_states {
            if let Some(need) = self.analyze_echo_state(state) {
                opportunities.push(EmergentOpportunity::EchoNeed {
                    echo_type: echo_type.clone(),
                    need,
                });
            }
        }
        
        opportunities
    }
    
    fn analyze_echo_state(&self, state: &EchoState) -> Option<String> {
        if state.energy < 20.0 {
            Some("low_energy".to_string())
        } else if state.loneliness > 80.0 {
            Some("needs_companionship".to_string())
        } else if state.unfulfilled_requests > 3 {
            Some("has_urgent_request".to_string())
        } else {
            None
        }
    }
    
    fn create_crisis_quest(
        &self,
        region_id: &RegionId,
        severity: f32,
        player_profile: &PlayerProfile,
    ) -> DynamicQuest {
        let urgency = if severity > 0.8 { "Critical" } else if severity > 0.5 { "Urgent" } else { "Important" };
        
        DynamicQuest {
            id: Uuid::new_v4(),
            title: format!("{} Harmony Crisis", urgency),
            description: format!(
                "The harmony in this region has fallen dangerously low. \
                The Silence threatens to consume everything if action isn't taken immediately."
            ),
            quest_type: QuestType::Restoration {
                harmony_requirement: 60.0,
            },
            objectives: vec![
                DynamicObjective {
                    id: Uuid::new_v4(),
                    description: "Rush to the affected region".to_string(),
                    objective_type: ObjectiveType::ReachLocation {
                        coordinates: Coordinates { x: 0.0, y: 0.0, z: 0.0 }, // Would be set from region
                        radius: 100.0,
                    },
                    progress: ObjectiveProgress::NotStarted,
                    hidden: false,
                    optional: false,
                },
                DynamicObjective {
                    id: Uuid::new_v4(),
                    description: "Perform restoration melodies to push back the Silence".to_string(),
                    objective_type: ObjectiveType::RestoreHarmony {
                        region_id: region_id.clone(),
                        target_level: 60.0,
                    },
                    progress: ObjectiveProgress::NotStarted,
                    hidden: false,
                    optional: false,
                },
                DynamicObjective {
                    id: Uuid::new_v4(),
                    description: "Rally other Songweavers to help (Optional)".to_string(),
                    objective_type: ObjectiveType::GatherPlayers {
                        count: 3,
                        location: None,
                    },
                    progress: ObjectiveProgress::NotStarted,
                    hidden: false,
                    optional: true,
                },
            ],
            prerequisites: QuestPrerequisites {
                min_resonance: None, // Emergency - no prerequisites
                required_quests: vec![],
                required_echo_bonds: HashMap::new(),
                region_harmony: None,
            },
            rewards: QuestRewards {
                resonance: Resonance {
                    creative: (30.0 * severity) as u64,
                    exploration: (20.0 * severity) as u64,
                    restoration: (50.0 * severity) as u64,
                },
                items: vec!["Harmony Shard".to_string()],
                unlocks: vec![],
                narrative_impact: NarrativeImpact {
                    world_state_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("saved_region".to_string(), serde_json::json!(region_id));
                        changes
                    },
                    relationship_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("region_inhabitants".to_string(), 50);
                        changes
                    },
                    legend_entry: Some(format!("Saved a region from the {} crisis", urgency.to_lowercase())),
                },
            },
            context: QuestContext {
                generated_by: QuestGenerator::WorldEvent {
                    event_id: format!("harmony_crisis_{}", region_id.0),
                },
                narrative_tags: vec!["emergency".to_string(), "restoration".to_string(), "crisis".to_string()],
                difficulty_rating: severity,
                estimated_duration: (20.0 * severity) as u64,
            },
            state: QuestState::Available,
            created_at: chrono::Utc::now(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(2)), // Urgent!
        }
    }
    
    fn create_social_quest(
        &self,
        location: &Coordinates,
        player_count: u32,
    ) -> DynamicQuest {
        DynamicQuest {
            id: Uuid::new_v4(),
            title: "Convergence of Songweavers".to_string(),
            description: format!(
                "A gathering of {} Songweavers has formed. \
                This is a rare opportunity to perform a group harmony that could strengthen the entire region.",
                player_count
            ),
            quest_type: QuestType::Community {
                min_participants: 3,
                max_participants: player_count + 5,
            },
            objectives: vec![
                DynamicObjective {
                    id: Uuid::new_v4(),
                    description: "Join the gathering of Songweavers".to_string(),
                    objective_type: ObjectiveType::ReachLocation {
                        coordinates: location.clone(),
                        radius: 50.0,
                    },
                    progress: ObjectiveProgress::NotStarted,
                    hidden: false,
                    optional: false,
                },
                DynamicObjective {
                    id: Uuid::new_v4(),
                    description: "Synchronize your melody with other players".to_string(),
                    objective_type: ObjectiveType::PerformMelody {
                        melody_type: Some("harmony".to_string()),
                        location: Some(location.clone()),
                    },
                    progress: ObjectiveProgress::NotStarted,
                    hidden: false,
                    optional: false,
                },
            ],
            prerequisites: QuestPrerequisites {
                min_resonance: Some(Resonance {
                    creative: 20,
                    exploration: 20,
                    restoration: 20,
                }),
                required_quests: vec![],
                required_echo_bonds: HashMap::new(),
                region_harmony: None,
            },
            rewards: QuestRewards {
                resonance: Resonance {
                    creative: 40,
                    exploration: 40,
                    restoration: 40,
                },
                items: vec!["Harmony Amplifier".to_string()],
                unlocks: vec![QuestUnlock::NewMelody {
                    melody_id: "group_harmony".to_string(),
                }],
                narrative_impact: NarrativeImpact {
                    world_state_changes: HashMap::new(),
                    relationship_changes: {
                        let mut changes = HashMap::new();
                        changes.insert("songweaver_community".to_string(), 25);
                        changes
                    },
                    legend_entry: Some("Participated in a grand convergence of Songweavers".to_string()),
                },
            },
            context: QuestContext {
                generated_by: QuestGenerator::WorldEvent {
                    event_id: "player_convergence".to_string(),
                },
                narrative_tags: vec!["social".to_string(), "harmony".to_string(), "community".to_string()],
                difficulty_rating: 0.5,
                estimated_duration: 30,
            },
            state: QuestState::Available,
            created_at: chrono::Utc::now(),
            expires_at: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
        }
    }
    
    fn create_echo_quest(
        &self,
        echo_type: &EchoType,
        need: &str,
    ) -> DynamicQuest {
        let (title, description, objective) = match (echo_type, need) {
            (EchoType::Lumi, "low_energy") => (
                "Lumi's Fading Light".to_string(),
                "Lumi's hopeful glow is dimming. She needs the energy of discovery to reignite her spark.".to_string(),
                ObjectiveType::PerformMelody {
                    melody_type: Some("discovery".to_string()),
                    location: None,
                }
            ),
            (EchoType::KAI, "needs_companionship") => (
                "KAI's Logical Loneliness".to_string(),
                "KAI has been processing alone for too long. Engage in meaningful interaction to ease their isolation.".to_string(),
                ObjectiveType::InteractWithEcho {
                    echo_type: EchoType::KAI,
                    min_bond_level: 30,
                }
            ),
            (EchoType::Terra, "has_urgent_request") => (
                "Terra's Call of the Wild".to_string(),
                "Terra senses a disturbance in the natural order and needs help investigating.".to_string(),
                ObjectiveType::ExploreArea {
                    region_id: RegionId(Uuid::new_v4()),
                    coverage_percent: 0.7,
                }
            ),
            (EchoType::Ignis, _) => (
                "Ignis's Challenge".to_string(),
                "Ignis seeks a worthy Songweaver to tests in the fires of courage.".to_string(),
                ObjectiveType::SurviveTime {
                    duration_seconds: 300,
                    conditions: vec!["combat_trial".to_string()],
                }
            ),
            _ => (
                format!("{:?}'s Request", echo_type),
                "An Echo needs your assistance.".to_string(),
                ObjectiveType::InteractWithEcho {
                    echo_type: echo_type.clone(),
                    min_bond_level: 20,
                }
            ),
        };
        
        DynamicQuest {
            id: Uuid::new_v4(),
            title,
            description,
            quest_type: QuestType::Personal {
                narrative_weight: 0.9,
            },
            objectives: vec![
                DynamicObjective {
                    id: Uuid::new_v4(),
                    description: "Respond to the Echo's call".to_string(),
                    objective_type: objective,
                    progress: ObjectiveProgress::NotStarted,
                    hidden: false,
                    optional: false,
                },
            ],
            prerequisites: QuestPrerequisites {
                min_resonance: None,
                required_quests: vec![],
                required_echo_bonds: {
                    let mut bonds = HashMap::new();
                    bonds.insert(echo_type.clone(), 10);
                    bonds
                },
                region_harmony: None,
            },
            rewards: QuestRewards {
                resonance: Resonance {
                    creative: 30,
                    exploration: 30,
                    restoration: 30,
                },
                items: vec![],
                unlocks: vec![QuestUnlock::EchoAbility {
                    echo_type: echo_type.clone(),
                    ability: "special_bond".to_string(),
                }],
                narrative_impact: NarrativeImpact {
                    world_state_changes: HashMap::new(),
                    relationship_changes: {
                        let mut changes = HashMap::new();
                        changes.insert(format!("{:?}", echo_type), 30);
                        changes
                    },
                    legend_entry: Some(format!("Answered {:?}'s call in their time of need", echo_type)),
                },
            },
            context: QuestContext {
                generated_by: QuestGenerator::Echo {
                    echo_type: echo_type.clone(),
                },
                narrative_tags: vec!["echo".to_string(), "personal".to_string(), "bond".to_string()],
                difficulty_rating: 0.7,
                estimated_duration: 25,
            },
            state: QuestState::Available,
            created_at: chrono::Utc::now(),
            expires_at: None,
        }
    }
}

// Supporting types and structures
#[derive(Debug, Clone)]
pub struct PlayerProfile {
    pub player_id: PlayerId,
    pub total_resonance: Resonance,
    pub completed_quests: Vec<Uuid>,
    pub play_style: PlayStyle,
    pub preferred_content: Vec<String>,
}

impl PlayerProfile {
    pub fn total_resonance(&self) -> u64 {
        self.total_resonance.creative + self.total_resonance.exploration + self.total_resonance.restoration
    }
    
    pub fn recent_quest_types(&self) -> Vec<String> {
        // Analyze recent quests to avoid repetition
        vec!["restoration".to_string(), "exploration".to_string()]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayStyle {
    Explorer,
    Socializer,
    Achiever,
    Creator,
    Mixed,
}

#[derive(Debug, Clone)]
pub struct GenerationContext {
    pub generation_type: GenerationType,
    pub region_id: Option<RegionId>,
    pub region_name: Option<String>,
    pub world_events: Vec<String>,
}

impl Default for GenerationContext {
    fn default() -> Self {
        Self {
            generation_type: GenerationType::Template {
                template_id: "restoration_basic".to_string(),
            },
            region_id: None,
            region_name: None,
            world_events: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum GenerationType {
    Template { template_id: String },
    AI { parameters: HashMap<String, serde_json::Value> },
    Emergent { world_state: WorldState },
}

#[derive(Debug, Clone)]
pub struct WorldState {
    pub region_harmonies: HashMap<RegionId, f32>,
    pub player_concentrations: HashMap<Coordinates, Vec<PlayerId>>,
    pub echo_states: HashMap<EchoType, EchoState>,
    pub active_events: Vec<WorldEvent>,
}

#[derive(Debug, Clone)]
pub struct EchoState {
    pub energy: f32,
    pub loneliness: f32,
    pub unfulfilled_requests: u32,
    pub last_interaction: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub struct WorldEvent {
    pub event_id: String,
    pub event_type: String,
    pub affected_regions: Vec<RegionId>,
    pub severity: f32,
}

#[derive(Debug, Clone)]
enum EmergentOpportunity {
    HarmonyCrisis { region_id: RegionId, severity: f32 },
    PlayerGathering { location: Coordinates, count: u32 },
    EchoNeed { echo_type: EchoType, need: String },
}

fn calculate_difficulty_modifier(player_profile: &PlayerProfile) -> f32 {
    let base_difficulty = 1.0;
    let level_modifier = (player_profile.total_resonance() as f32 / 100.0).min(2.0);
    base_difficulty * level_modifier
}

fn calculate_crisis_severity(harmony: f32) -> f32 {
    if harmony < 10.0 {
        1.0
    } else if harmony < 20.0 {
        0.8
    } else if harmony < 30.0 {
        0.6
    } else {
        0.4
    }
}