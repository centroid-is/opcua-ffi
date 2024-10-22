use anyhow::Result;
use chrono::Duration;
use flutter_rust_bridge::frb;
use std::path::PathBuf;
#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

use opcua::client;

#[frb]
pub struct WrapClient(client::Client);

impl WrapClient {
    #[frb(ignore)]
    pub fn new(config: client::ClientConfig) -> Self {
        Self(client::Client::new(config))
    }
}

impl From<client::Client> for WrapClient {
    #[frb(ignore)]
    fn from(client: client::Client) -> Self {
        Self(client)
    }
}

pub struct WrapClientEndpoint(client::ClientEndpoint);

#[frb(sync)]
impl WrapClientEndpoint {
    #[frb(sync)]
    pub fn new(url: String) -> Self {
        Self(client::ClientEndpoint::new(url))
    }
}

impl From<WrapClientEndpoint> for client::ClientEndpoint {
    #[frb(ignore)]
    fn from(endpoint: WrapClientEndpoint) -> Self {
        endpoint.0
    }
}

pub struct WrapClientUserToken(client::ClientUserToken);

#[frb(sync)]
impl WrapClientUserToken {
    #[frb(sync)]
    pub fn user_pass(user: String, password: String) -> Self {
        Self(client::ClientUserToken::user_pass(user, password))
    }
    #[frb(sync)]
    pub fn x509(user: String, cert_path: String, private_key_path: String) -> Self {
        Self(client::ClientUserToken::x509(
            user,
            &PathBuf::from(cert_path),
            &PathBuf::from(private_key_path),
        ))
    }
    #[frb(sync)]
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }
}

#[frb(sync)]
pub struct WrapClientBuilder(client::ClientBuilder);

// #[delegate(self.0)]
impl WrapClientBuilder {
    #[frb(sync)]
    pub fn new() -> Self {
        Self(client::ClientBuilder::new())
    }
    #[frb(sync)]
    pub fn from_config(path: String) -> Result<Self> {
        client::ClientBuilder::from_config(PathBuf::from(path))
            .map(|builder| Self(builder))
            .map_err(|_| anyhow::anyhow!("Failed to create ClientBuilder"))
    }
    #[frb(sync)]
    pub fn client(self) -> Result<WrapClient> {
        self.0
            .client()
            .ok_or_else(|| anyhow::anyhow!("Failed to create Client"))
            .map(WrapClient::from)
    }
    #[frb(sync)]
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }
    #[frb(sync)]
    pub fn application_name(self, application_name: String) -> Self {
        Self(self.0.application_name(application_name))
    }
    #[frb(sync)]
    pub fn application_uri(self, application_uri: String) -> Self {
        Self(self.0.application_uri(application_uri))
    }
    #[frb(sync)]
    pub fn product_uri(self, product_uri: String) -> Self {
        Self(self.0.product_uri(product_uri))
    }
    #[frb(sync)]
    pub fn create_sample_keypair(self, create_sample_keypair: bool) -> Self {
        Self(self.0.create_sample_keypair(create_sample_keypair))
    }
    #[frb(sync)]
    pub fn certificate_path(self, certificate_path: String) -> Self {
        Self(self.0.certificate_path(PathBuf::from(certificate_path)))
    }
    #[frb(sync)]
    pub fn private_key_path(self, private_key_path: String) -> Self {
        Self(self.0.private_key_path(PathBuf::from(private_key_path)))
    }
    #[frb(sync)]
    pub fn trust_server_certs(self, trust_server_certs: bool) -> Self {
        Self(self.0.trust_server_certs(trust_server_certs))
    }
    #[frb(sync)]
    pub fn verify_server_certs(self, verify_server_certs: bool) -> Self {
        Self(self.0.verify_server_certs(verify_server_certs))
    }
    #[frb(sync)]
    pub fn pki_dir(self, pki_dir: String) -> Self {
        Self(self.0.pki_dir(PathBuf::from(pki_dir)))
    }
    #[frb(sync)]
    pub fn preferred_locales(self, preferred_locales: Vec<String>) -> Self {
        Self(self.0.preferred_locales(preferred_locales))
    }
    #[frb(sync)]
    pub fn default_endpoint(self, default_endpoint: String) -> Self {
        Self(self.0.default_endpoint(default_endpoint))
    }
    #[frb(sync)]
    pub fn endpoint(self, endpoint_id: String, endpoint: WrapClientEndpoint) -> Self {
        Self(self.0.endpoint(endpoint_id, endpoint.0))
    }
    #[frb(sync)]
    pub fn endpoints(self, endpoints: Vec<(String, WrapClientEndpoint)>) -> Self {
        Self(
            self.0.endpoints(
                endpoints
                    .into_iter()
                    .map(|(id, endpoint)| (id, endpoint.0))
                    .collect(),
            ),
        )
    }
    #[frb(sync)]
    pub fn user_token(self, user_token_id: String, user_token: WrapClientUserToken) -> Self {
        Self(self.0.user_token(user_token_id, user_token.0))
    }
    #[frb(sync)]
    pub fn max_message_size(self, max_message_size: usize) -> Self {
        Self(self.0.max_message_size(max_message_size))
    }
    #[frb(sync)]
    pub fn max_chunk_count(self, max_chunk_count: usize) -> Self {
        Self(self.0.max_chunk_count(max_chunk_count))
    }
    #[frb(sync)]
    pub fn max_chunk_size(self, max_chunk_size: usize) -> Self {
        Self(self.0.max_chunk_size(max_chunk_size))
    }
    #[frb(sync)]
    pub fn max_incoming_chunk_size(self, max_incoming_chunk_size: usize) -> Self {
        Self(self.0.max_incoming_chunk_size(max_incoming_chunk_size))
    }
    #[frb(sync)]
    pub fn max_string_length(self, max_string_length: usize) -> Self {
        Self(self.0.max_string_length(max_string_length))
    }
    #[frb(sync)]
    pub fn max_byte_string_length(self, max_byte_string_length: usize) -> Self {
        Self(self.0.max_byte_string_length(max_byte_string_length))
    }
    #[frb(sync)]
    pub fn max_array_length(self, max_array_length: usize) -> Self {
        Self(self.0.max_array_length(max_array_length))
    }
    #[frb(sync)]
    pub fn session_retry_limit(self, session_retry_limit: i32) -> Self {
        Self(self.0.session_retry_limit(session_retry_limit))
    }
    #[frb(sync)]
    pub fn session_retry_initial(self, session_retry_initial: Duration) -> Self {
        Self(
            self.0
                .session_retry_initial(chrono::TimeDelta::to_std(&session_retry_initial)),
        )
    }
}
