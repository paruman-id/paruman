#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod musyawarah {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MusyawarahTrack {
        PasangkepanRutin,
        ParumanAgung,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MusyawarahOutcome {
        Mufakat,
        Ditunda,
        PemungutanSuara,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct MusyawarahMatter {
        pub id: u32,
        pub track: MusyawarahTrack,
        pub title: String,
        pub raised_by: AccountId,
        pub banjar_id: String,
        pub deliberation_opened_at: Timestamp,
        pub deliberation_closes_at: Timestamp,
        pub outcome: Option<MusyawarahOutcome>,
        pub chain_anchor_hash: Option<String>,
    }

    #[ink(storage)]
    pub struct Musyawarah {
        owner: AccountId,
        matters: Mapping<u32, MusyawarahMatter>,
        matter_count: u32,
        banjar_matters: Mapping<String, Vec<u32>>,
    }

    #[ink(event)]
    pub struct MatterRaised {
        #[ink(topic)]
        matter_id: u32,
        track: MusyawarahTrack,
        raised_by: AccountId,
        banjar_id: String,
        opened_at: Timestamp,
    }

    #[ink(event)]
    pub struct MatterOutcomeRecorded {
        #[ink(topic)]
        matter_id: u32,
        outcome: MusyawarahOutcome,
        recorded_at: Timestamp,
    }

    #[ink(event)]
    pub struct MatterAnchored {
        #[ink(topic)]
        matter_id: u32,
        chain_anchor_hash: String,
        anchored_at: Timestamp,
    }

    impl Musyawarah {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                matters: Mapping::new(),
                matter_count: 0,
                banjar_matters: Mapping::new(),
            }
        }

        #[ink(message)]
        pub fn raise_matter(
            &mut self,
            track: MusyawarahTrack,
            title: String,
            banjar_id: String,
        ) -> Result<u32, String> {
            let now = Self::env().block_timestamp();
            let deliberation_period = match track {
                MusyawarahTrack::PasangkepanRutin => 35 * 24 * 60 * 60 * 1000,
                MusyawarahTrack::ParumanAgung => 105 * 24 * 60 * 60 * 1000,
            };

            let matter_id = self.matter_count;
            let matter = MusyawarahMatter {
                id: matter_id,
                track,
                title,
                raised_by: Self::env().caller(),
                banjar_id: banjar_id.clone(),
                deliberation_opened_at: now,
                deliberation_closes_at: now + deliberation_period,
                outcome: None,
                chain_anchor_hash: None,
            };

            self.matters.insert(matter_id, &matter);
            self.matter_count += 1;

            self.env().emit_event(MatterRaised {
                matter_id,
                track,
                raised_by: Self::env().caller(),
                banjar_id,
                opened_at: now,
            });

            Ok(matter_id)
        }

        #[ink(message)]
        pub fn record_outcome(
            &mut self,
            matter_id: u32,
            outcome: MusyawarahOutcome,
        ) -> Result<(), String> {
            if Self::env().caller() != self.owner {
                return Err("Only owner can record outcomes".to_string());
            }

            if let Some(mut matter) = self.matters.get(matter_id) {
                matter.outcome = Some(outcome);
                self.matters.insert(matter_id, &matter);

                self.env().emit_event(MatterOutcomeRecorded {
                    matter_id,
                    outcome,
                    recorded_at: Self::env().block_timestamp(),
                });

                Ok(())
            } else {
                Err("Matter not found".to_string())
            }
        }

        #[ink(message)]
        pub fn anchor_matter(
            &mut self,
            matter_id: u32,
            chain_anchor_hash: String,
        ) -> Result<(), String> {
            if Self::env().caller() != self.owner {
                return Err("Only owner can anchor matters".to_string());
            }

            if let Some(mut matter) = self.matters.get(matter_id) {
                matter.chain_anchor_hash = Some(chain_anchor_hash.clone());
                self.matters.insert(matter_id, &matter);

                self.env().emit_event(MatterAnchored {
                    matter_id,
                    chain_anchor_hash,
                    anchored_at: Self::env().block_timestamp(),
                });

                Ok(())
            } else {
                Err("Matter not found".to_string())
            }
        }

        #[ink(message)]
        pub fn get_matter(&self, matter_id: u32) -> Option<MusyawarahMatter> {
            self.matters.get(matter_id)
        }

        #[ink(message)]
        pub fn is_deliberation_active(&self, matter_id: u32) -> bool {
            if let Some(matter) = self.matters.get(matter_id) {
                let now = Self::env().block_timestamp();
                now >= matter.deliberation_opened_at && now <= matter.deliberation_closes_at
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn total_matters(&self) -> u32 {
            self.matter_count
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_raise_matter() {
            let mut contract = Musyawarah::new();
            let title = "Test Matter".to_string();
            let banjar_id = "banjar_test".to_string();

            let result = contract.raise_matter(
                MusyawarahTrack::PasangkepanRutin,
                title,
                banjar_id,
            );
            assert!(result.is_ok());
            assert_eq!(contract.total_matters(), 1);
        }

        #[ink::test]
        fn test_record_outcome() {
            let mut contract = Musyawarah::new();
            let title = "Test Matter".to_string();
            let banjar_id = "banjar_test".to_string();

            let matter_id = contract
                .raise_matter(MusyawarahTrack::PasangkepanRutin, title, banjar_id)
                .unwrap();

            let result = contract.record_outcome(matter_id, MusyawarahOutcome::Mufakat);
            assert!(result.is_ok());

            if let Some(matter) = contract.get_matter(matter_id) {
                assert_eq!(matter.outcome, Some(MusyawarahOutcome::Mufakat));
            }
        }
    }
}
