use async_trait::async_trait;
use md5::{Digest, Md5};
use uuid::Uuid;

use crate::{
    account::Account,
    certs::{Certificates, FetchCertificatesError},
    sessionserver::ClientSessionServerError,
};

#[derive(Clone, Debug)]
pub struct OfflineAccount {
    pub username: String,
    pub uuid: Option<Uuid>,
}

impl OfflineAccount {
    pub fn new(username: impl ToString) -> Self {
        Self {
            username: username.to_string(),
            uuid: None,
        }
    }

    pub fn with_uuid(username: impl ToString, uuid: Uuid) -> Self {
        Self {
            username: username.to_string(),
            uuid: Some(uuid),
        }
    }
}

#[async_trait]
impl Account for OfflineAccount {
    async fn join_with_server_id_hash(
        &self,
        _: Uuid,
        _: String,
    ) -> Result<(), ClientSessionServerError> {
        Ok(())
    }

    async fn fetch_certificates(&self) -> Result<Option<Certificates>, FetchCertificatesError> {
        Ok(None)
    }

    fn get_username(&self) -> String {
        self.username.clone()
    }

    fn get_uuid(&self) -> Uuid {
        self.uuid.unwrap_or_else(|| generate_uuid(&self.username))
    }
}

pub fn generate_uuid(username: &str) -> Uuid {
    uuid::Builder::from_md5_bytes(hash(format!("OfflinePlayer:{username}").as_bytes())).into_uuid()
}

fn hash(data: &[u8]) -> [u8; 16] {
    let mut hasher = Md5::new();

    hasher.update(data);

    let mut bytes = [0; 16];
    bytes.copy_from_slice(&hasher.finalize()[..16]);

    bytes
}
