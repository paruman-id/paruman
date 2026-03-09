use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MandalaID {
    pub did: String,
    pub chain_anchor_hash: Option<String>,
    pub verified_at: Option<DateTime<Utc>>,
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
pub struct IdentityVerification {
    pub user_id: Uuid,
    pub mandala_id: MandalaID,
    pub identity_tier: IdentityTier,
    pub is_diaspora: bool,
    pub is_krama_mipil: bool,
    pub verified_at: DateTime<Utc>,
    pub kawenang_active_from: Option<DateTime<Utc>>,
}

impl IdentityVerification {
    pub fn new(user_id: Uuid, mandala_id: MandalaID) -> Self {
        let verified_at = Utc::now();
        let kawenang_active_from = verified_at + Duration::days(35);

        Self {
            user_id,
            mandala_id,
            identity_tier: IdentityTier::Tier1,
            is_diaspora: false,
            is_krama_mipil: false,
            verified_at,
            kawenang_active_from: Some(kawenang_active_from),
        }
    }

    pub fn is_kawenang_active(&self) -> bool {
        if let Some(active_from) = self.kawenang_active_from {
            Utc::now() >= active_from
        } else {
            false
        }
    }

    pub fn days_until_kawenang_active(&self) -> Option<i64> {
        if let Some(active_from) = self.kawenang_active_from {
            let now = Utc::now();
            if now < active_from {
                Some((active_from - now).num_days())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationRequest {
    pub mandala_id_did: String,
    pub identity_tier: IdentityTier,
    pub is_diaspora: bool,
    pub is_krama_mipil: bool,
    pub banjar_id: Option<String>,
    pub kk_proof_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityVerificationResponse {
    pub user_id: Uuid,
    pub identity_tier: IdentityTier,
    pub is_kawenang_active: bool,
    pub days_until_active: Option<i64>,
    pub verified_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_verification_creation() {
        let user_id = Uuid::new_v4();
        let mandala_id = MandalaID {
            did: "did:example:123456".to_string(),
            chain_anchor_hash: None,
            verified_at: None,
        };

        let verification = IdentityVerification::new(user_id, mandala_id);
        assert_eq!(verification.identity_tier, IdentityTier::Tier1);
        assert!(!verification.is_kawenang_active());
        assert!(verification.days_until_kawenang_active().is_some());
    }

    #[test]
    fn test_kawenang_hold_period() {
        let user_id = Uuid::new_v4();
        let mandala_id = MandalaID {
            did: "did:example:123456".to_string(),
            chain_anchor_hash: None,
            verified_at: None,
        };

        let verification = IdentityVerification::new(user_id, mandala_id);
        let days_until = verification.days_until_kawenang_active().unwrap();
        assert!(days_until > 0 && days_until <= 35);
    }
}
