// services/symphony-engine/src/spatial_audio.rs
use nalgebra::{Vector3, Point3};
use std::collections::HashMap;

pub struct SpatialAudioEngine {
    listener_position: Point3<f32>,
    listener_orientation: Vector3<f32>,
    sound_sources: HashMap<uuid::Uuid, SpatialSoundSource>,
}

impl SpatialAudioEngine {
    pub fn new() -> Self {
        Self {
            listener_position: Point3::origin(),
            listener_orientation: Vector3::z(), // Looking forward
            sound_sources: HashMap::new(),
        }
    }

    pub fn update_listener(
        &mut self,
        position: Point3<f32>,
        orientation: Vector3<f32>,
    ) {
        self.listener_position = position;
        self.listener_orientation = orientation.normalize();
    }

    pub fn add_sound_source(&mut self, source: SpatialSoundSource) {
        self.sound_sources.insert(source.id, source);
    }

    pub fn process_3d_audio(
        &self,
        source_id: uuid::Uuid,
        audio: Vec<f32>,
    ) -> StereoAudio {
        if let Some(source) = self.sound_sources.get(&source_id) {
            self.apply_3d_processing(audio, source)
        } else {
            // Return mono audio as stereo if source not found
            StereoAudio {
                left: audio.clone(),
                right: audio,
            }
        }
    }

    fn apply_3d_processing(
        &self,
        audio: Vec<f32>,
        source: &SpatialSoundSource,
    ) -> StereoAudio {
        // Calculate distance and direction
        let direction = source.position - self.listener_position;
        let distance = direction.magnitude();
        let normalized_dir = direction.normalize();

        // Calculate attenuation based on distance
        let attenuation = self.calculate_attenuation(distance, &source.attenuation);

        // Calculate panning based on direction
        let (left_gain, right_gain) = self.calculate_stereo_panning(normalized_dir);

        // Apply Doppler effect if source is moving
        let doppler_shifted = if source.velocity.magnitude() > 0.01 {
            self.apply_doppler_effect(audio.clone(), source, distance)
        } else {
            audio.clone()
        };

        // Apply environmental effects
        let processed = self.apply_environmental_effects(
            doppler_shifted,
            source,
            distance,
        );

        // Create stereo output with panning and attenuation
        StereoAudio {
            left: processed.iter()
                .map(|&s| s * left_gain * attenuation)
                .collect(),
            right: processed.iter()
                .map(|&s| s * right_gain * attenuation)
                .collect(),
        }
    }

    fn calculate_attenuation(
        &self,
        distance: f32,
        model: &AttenuationModel,
    ) -> f32 {
        match model {
            AttenuationModel::Linear { min_distance, max_distance } => {
                if distance <= *min_distance {
                    1.0
                } else if distance >= *max_distance {
                    0.0
                } else {
                    1.0 - (distance - min_distance) / (max_distance - min_distance)
                }
            },
            AttenuationModel::Exponential { rolloff_factor } => {
                (1.0 / (1.0 + rolloff_factor * distance)).min(1.0)
            },
            AttenuationModel::Logarithmic { reference_distance, rolloff_factor } => {
                (reference_distance / (reference_distance + rolloff_factor * (distance - reference_distance))).min(1.0)
            },
        }
    }

    fn calculate_stereo_panning(&self, direction: Vector3<f32>) -> (f32, f32) {
        // Calculate angle between listener orientation and sound direction
        let right = self.listener_orientation.cross(&Vector3::y()).normalize();
        let forward = self.listener_orientation;

        // Project direction onto listener's plane
        let forward_component = direction.dot(&forward);
        let right_component = direction.dot(&right);

        // Calculate panning (-1 = full left, 0 = center, 1 = full right)
        let pan = right_component.atan2(forward_component) / std::f32::consts::PI;

        // Convert to left/right gains using constant power panning
        let left_gain = ((1.0 - pan) * std::f32::consts::PI / 4.0).cos();
        let right_gain = ((1.0 + pan) * std::f32::consts::PI / 4.0).cos();

        (left_gain, right_gain)
    }

    fn apply_doppler_effect(
        &self,
        audio: Vec<f32>,
        source: &SpatialSoundSource,
        distance: f32,
    ) -> Vec<f32> {
        // Speed of sound in units per second
        const SPEED_OF_SOUND: f32 = 343.0;

        // Calculate relative velocity along the line between listener and source
        let direction = (source.position - self.listener_position).normalize();
        let relative_velocity = source.velocity.dot(&direction);

        // Calculate Doppler shift factor
        let doppler_factor = SPEED_OF_SOUND / (SPEED_OF_SOUND - relative_velocity);

        // Resample audio based on Doppler factor
        self.resample_audio(audio, doppler_factor)
    }

    fn resample_audio(&self, audio: Vec<f32>, factor: f32) -> Vec<f32> {
        // Simple linear interpolation resampling
        let output_len = (audio.len() as f32 / factor) as usize;
        let mut output = Vec::with_capacity(output_len);

        for i in 0..output_len {
            let source_index = i as f32 * factor;
            let index_floor = source_index.floor() as usize;
            let fraction = source_index - index_floor as f32;

            if index_floor + 1 < audio.len() {
                let interpolated = audio[index_floor] * (1.0 - fraction)
                    + audio[index_floor + 1] * fraction;
                output.push(interpolated);
            } else if index_floor < audio.len() {
                output.push(audio[index_floor]);
            }
        }

        output
    }

    fn apply_environmental_effects(
        &self,
        audio: Vec<f32>,
        source: &SpatialSoundSource,
        distance: f32,
    ) -> Vec<f32> {
        let mut processed = audio;

        // Apply low-pass filter for distance
        if distance > 10.0 {
            processed = self.apply_lowpass_filter(processed, 2000.0 / (distance / 10.0));
        }

        // Apply reverb based on environment
        if source.environment.reverb > 0.0 {
            processed = self.apply_environmental_reverb(
                processed,
                &source.environment,
            );
        }

        // Apply occlusion
        if source.occlusion > 0.0 {
            processed = self.apply_occlusion(processed, source.occlusion);
        }

        processed
    }

    fn apply_lowpass_filter(&self, audio: Vec<f32>, cutoff: f32) -> Vec<f32> {
        // Simple one-pole lowpass filter
        let mut output = vec![0.0; audio.len()];
        let rc = 1.0 / (2.0 * std::f32::consts::PI * cutoff);
        let dt = 1.0 / 44100.0;
        let alpha = dt / (rc + dt);

        output[0] = audio[0];
        for i in 1..audio.len() {
            output[i] = output[i - 1] + alpha * (audio[i] - output[i - 1]);
        }

        output
    }

    fn apply_environmental_reverb(
        &self,
        audio: Vec<f32>,
        environment: &EnvironmentAcoustics,
    ) -> Vec<f32> {
        let mut output = audio.clone();

        // Apply multiple delay taps for reverb
        let delays = [
            (0.043, 0.5),  // Early reflection 1
            (0.067, 0.4),  // Early reflection 2
            (0.087, 0.3),  // Early reflection 3
            (0.120, 0.25), // Late reflection 1
            (0.190, 0.2),  // Late reflection 2
        ];

        for (delay_time, gain) in delays.iter() {
            let delay_samples = (delay_time * 44100.0) as usize;
            for i in delay_samples..output.len() {
                output[i] += output[i - delay_samples] * gain * environment.reverb;
            }
        }

        output
    }

    fn apply_occlusion(&self, audio: Vec<f32>, occlusion: f32) -> Vec<f32> {
        // Apply more aggressive low-pass filter for occlusion
        self.apply_lowpass_filter(audio, 1000.0 * (1.0 - occlusion))
    }
}

pub struct SpatialSoundSource {
    pub id: uuid::Uuid,
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub attenuation: AttenuationModel,
    pub environment: EnvironmentAcoustics,
    pub occlusion: f32, // 0.0 = no occlusion, 1.0 = fully occluded
}

pub enum AttenuationModel {
    Linear {
        min_distance: f32,
        max_distance: f32,
    },
    Exponential {
        rolloff_factor: f32,
    },
    Logarithmic {
        reference_distance: f32,
        rolloff_factor: f32,
    },
}

pub struct EnvironmentAcoustics {
    pub reverb: f32,          // 0.0 - 1.0
    pub echo_delay: f32,      // Seconds
    pub echo_decay: f32,      // 0.0 - 1.0
    pub absorption: f32,      // High frequency absorption
}

pub struct StereoAudio {
    pub left: Vec<f32>,
    pub right: Vec<f32>,
}