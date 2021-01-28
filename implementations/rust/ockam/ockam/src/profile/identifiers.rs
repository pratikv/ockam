use ockam_vault_core::KeyId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ProfileIdentifier(KeyId);

impl ProfileIdentifier {
    pub fn from_key_id(key_id: KeyId) -> Self {
        Self { 0: key_id }
    }

    pub fn to_string_representation(&self) -> String {
        format!("P_ID.{}", &self.0)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct EventIdentifier([u8; 32]);

impl AsRef<[u8]> for EventIdentifier {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl EventIdentifier {
    pub fn from_hash(hash: [u8; 32]) -> Self {
        Self { 0: hash }
    }

    pub fn to_string_representation(&self) -> String {
        format!("E_ID.{}", hex::encode(&self.0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let _identifier = ProfileIdentifier::from_key_id("test".to_string());
    }
}
