#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod kawenang {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::storage::Mapping;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Role {
        None,
        KelianBanjar,
        Moderator,
        SubakPekaseh,
        WakilAdat,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct KawenangRecord {
        pub holder: AccountId,
        pub banjar_id: String,
        pub base_weight: u128,
        pub seniority_weight: u128,
        pub role: Role,
        pub granted_at: Timestamp,
        pub active_from: Timestamp,
    }

    #[ink(storage)]
    pub struct Kawenang {
        owner: AccountId,
        kawenang_records: Mapping<AccountId, KawenangRecord>,
        banjar_members: Mapping<String, Vec<AccountId>>,
        total_holders: u32,
    }

    #[ink(event)]
    pub struct KawenangGranted {
        #[ink(topic)]
        holder: AccountId,
        banjar_id: String,
        base_weight: u128,
        granted_at: Timestamp,
    }

    #[ink(event)]
    pub struct KawenangRevoked {
        #[ink(topic)]
        holder: AccountId,
        revoked_at: Timestamp,
    }

    #[ink(event)]
    pub struct RoleModified {
        #[ink(topic)]
        holder: AccountId,
        new_role: Role,
        modified_at: Timestamp,
    }

    impl Kawenang {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                kawenang_records: Mapping::new(),
                banjar_members: Mapping::new(),
                total_holders: 0,
            }
        }

        #[ink(message)]
        pub fn grant_kawenang(
            &mut self,
            holder: AccountId,
            banjar_id: String,
            base_weight: u128,
        ) -> Result<(), String> {
            if Self::env().caller() != self.owner {
                return Err("Only owner can grant Kawenang".to_string());
            }

            let now = Self::env().block_timestamp();
            let active_from = now + 35 * 24 * 60 * 60 * 1000;

            let record = KawenangRecord {
                holder,
                banjar_id: banjar_id.clone(),
                base_weight,
                seniority_weight: 1000,
                role: Role::None,
                granted_at: now,
                active_from,
            };

            self.kawenang_records.insert(holder, &record);
            self.total_holders += 1;

            self.env().emit_event(KawenangGranted {
                holder,
                banjar_id,
                base_weight,
                granted_at: now,
            });

            Ok(())
        }

        #[ink(message)]
        pub fn get_kawenang(&self, holder: AccountId) -> Option<KawenangRecord> {
            self.kawenang_records.get(holder)
        }

        #[ink(message)]
        pub fn is_active(&self, holder: AccountId) -> bool {
            if let Some(record) = self.kawenang_records.get(holder) {
                Self::env().block_timestamp() >= record.active_from
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn get_effective_weight(&self, holder: AccountId) -> Option<u128> {
            if let Some(record) = self.kawenang_records.get(holder) {
                if !self.is_active(holder) {
                    return Some(0);
                }

                let role_modifier = match record.role {
                    Role::None => 0,
                    Role::KelianBanjar => 500,
                    Role::Moderator => 300,
                    Role::SubakPekaseh => 300,
                    Role::WakilAdat => 500,
                };

                let effective = (record.base_weight * record.seniority_weight / 1000) + role_modifier;
                Some(effective)
            } else {
                None
            }
        }

        #[ink(message)]
        pub fn set_role(&mut self, holder: AccountId, role: Role) -> Result<(), String> {
            if Self::env().caller() != self.owner {
                return Err("Only owner can set roles".to_string());
            }

            if let Some(mut record) = self.kawenang_records.get(holder) {
                record.role = role;
                self.kawenang_records.insert(holder, &record);

                self.env().emit_event(RoleModified {
                    holder,
                    new_role: role,
                    modified_at: Self::env().block_timestamp(),
                });

                Ok(())
            } else {
                Err("Kawenang holder not found".to_string())
            }
        }

        #[ink(message)]
        pub fn total_holders(&self) -> u32 {
            self.total_holders
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn test_grant_kawenang() {
            let mut contract = Kawenang::new();
            let holder = AccountId::from([0x01; 32]);
            let banjar_id = "banjar_test".to_string();

            let result = contract.grant_kawenang(holder, banjar_id, 1000);
            assert!(result.is_ok());
            assert_eq!(contract.total_holders(), 1);
        }

        #[ink::test]
        fn test_kawenang_hold_period() {
            let mut contract = Kawenang::new();
            let holder = AccountId::from([0x02; 32]);
            let banjar_id = "banjar_test".to_string();

            contract.grant_kawenang(holder, banjar_id, 1000).unwrap();
            assert!(!contract.is_active(holder));
        }
    }
}
