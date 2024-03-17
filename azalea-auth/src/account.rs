use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    certs::{Certificates, FetchCertificatesError},
    sessionserver::ClientSessionServerError,
};

#[async_trait]
pub trait Account: std::fmt::Debug + Send + Sync + 'static {
    async fn join(
        &self,
        public_key: &[u8],
        private_key: &[u8],
        server_id: &str,
    ) -> Result<(), ClientSessionServerError> {
        let server_hash = azalea_crypto::hex_digest(&azalea_crypto::digest_data(
            server_id.as_bytes(),
            public_key,
            private_key,
        ));
        let uuid = self.get_uuid();

        self.join_with_server_id_hash(uuid, server_hash).await
    }
    async fn join_with_server_id_hash(
        &self,
        uuid: Uuid,
        server_hash: String,
    ) -> Result<(), ClientSessionServerError>;

    async fn fetch_certificates(&self) -> Result<Option<Certificates>, FetchCertificatesError>;

    fn get_username(&self) -> String;
    fn get_uuid(&self) -> Uuid;
}