use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub mandala_id_did: String,
    pub identity_tier: IdentityTier,
    pub is_diaspora: bool,
    pub is_krama_mipil: bool,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentityTier {
    Guest,
    Tier1,
    Tier1D,
    Tier2,
    Tier3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub thread_id: Uuid,
    pub content: String,
    pub language: Language,
    pub location_banjar: Option<String>,
    pub location_subak: Option<String>,
    pub location_desa: Option<String>,
    pub location_kabupaten: Option<String>,
    pub created_at: DateTime<Utc>,
    pub topic_category: TopicCategory,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Language {
    Indonesian,
    Balinese,
    Mixed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TopicCategory {
    LandAndPlanning,
    CultureAndTradition,
    WaterAndEnvironment,
    Tourism,
    Education,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kawenang {
    pub holder_id: Uuid,
    pub banjar_id: String,
    pub base_weight: f64,
    pub seniority_weight: f64,
    pub role_modifier: f64,
    pub granted_at: DateTime<Utc>,
    pub active_from: DateTime<Utc>,
    pub chain_anchor_hash: Option<String>,
}

impl Kawenang {
    pub fn effective_weight(&self) -> f64 {
        self.base_weight * self.seniority_weight + self.role_modifier
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusyawarahMatter {
    pub id: Uuid,
    pub track: MusyawarahTrack,
    pub title: String,
    pub raised_by: Uuid,
    pub banjar_id: String,
    pub deliberation_opened_at: DateTime<Utc>,
    pub deliberation_closes_at: DateTime<Utc>,
    pub outcome: Option<MusyawarahOutcome>,
    pub chain_anchor_hash: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MusyawarahTrack {
    PasangkepanRutin,
    ParumanAgung,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MusyawarahOutcome {
    Mufakat,
    Ditunda,
    PemungutanSuara,
}
