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

impl WrapClientBuilder {
    #[frb(sync)]
    /// Creates a `ClientBuilder`
    pub fn new() -> Self {
        Self(client::ClientBuilder::new())
    }
    #[frb(sync)]
    /// Creates a `ClientBuilder` using a configuration file as the initial state.
    pub fn from_config(path: String) -> Result<Self> {
        client::ClientBuilder::from_config(PathBuf::from(path))
            .map(|builder| Self(builder))
            .map_err(|_| anyhow::anyhow!("Failed to create ClientBuilder"))
    }
    #[frb(sync)]
    /// Yields a [`Client`] from the values set by the builder. If the builder is not in a valid state
    /// it will return `None`.
    ///
    /// [`Client`]: client/struct.Client.html
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
    /// Sets the application name.
    pub fn application_name(self, application_name: String) -> Self {
        Self(self.0.application_name(application_name))
    }
    #[frb(sync)]
    /// Sets the application uri.
    pub fn application_uri(self, application_uri: String) -> Self {
        Self(self.0.application_uri(application_uri))
    }
    #[frb(sync)]
    /// Sets the product uri.
    pub fn product_uri(self, product_uri: String) -> Self {
        Self(self.0.product_uri(product_uri))
    }
    #[frb(sync)]
    /// Sets whether the client should generate its own key pair if there is none found in the pki
    /// directory.
    pub fn create_sample_keypair(self, create_sample_keypair: bool) -> Self {
        Self(self.0.create_sample_keypair(create_sample_keypair))
    }
    #[frb(sync)]
    /// Sets a custom client certificate path. The path is required to be provided as a partial
    /// path relative to the PKI directory. If set, this path will be used to read the client
    /// certificate from disk. The certificate can be in either the .der or .pem format.
    pub fn certificate_path(self, certificate_path: String) -> Self {
        Self(self.0.certificate_path(PathBuf::from(certificate_path)))
    }
    #[frb(sync)]
    /// Sets a custom private key path. The path is required to be provided as a partial path
    /// relative to the PKI directory. If set, this path will be used to read the private key
    /// from disk.
    pub fn private_key_path(self, private_key_path: String) -> Self {
        Self(self.0.private_key_path(PathBuf::from(private_key_path)))
    }
    #[frb(sync)]
    /// Sets whether the client should automatically trust servers. If this is not set then
    /// the client will reject the server upon first connect and the server's certificate
    /// must be manually moved from pki's `/rejected` folder to the `/trusted` folder. If it is
    /// set, then the server cert will automatically be stored in the `/trusted` folder.
    pub fn trust_server_certs(self, trust_server_certs: bool) -> Self {
        Self(self.0.trust_server_certs(trust_server_certs))
    }
    #[frb(sync)]
    /// Sets whether the client should verify server certificates. Regardless of this setting,
    /// server certificates are always checked to see if they are trusted and have a valid key
    /// length. In addition (if `verify_server_certs` is unset or is set to `true`) it will
    /// verify the hostname, application uri and the not before / after values to ensure validity.
    pub fn verify_server_certs(self, verify_server_certs: bool) -> Self {
        Self(self.0.verify_server_certs(verify_server_certs))
    }
    #[frb(sync)]
    /// Sets the pki directory where client's own key pair is stored and where `/trusted` and
    /// `/rejected` server certificates are stored.
    pub fn pki_dir(self, pki_dir: String) -> Self {
        Self(self.0.pki_dir(PathBuf::from(pki_dir)))
    }
    #[frb(sync)]
    /// Sets the preferred locales of the client. These are passed to the server during session
    /// creation to ensure localized strings are in the preferred language.
    pub fn preferred_locales(self, preferred_locales: Vec<String>) -> Self {
        Self(self.0.preferred_locales(preferred_locales))
    }
    #[frb(sync)]
    /// Sets the id of the default endpoint to connect to.
    pub fn default_endpoint(self, default_endpoint: String) -> Self {
        Self(self.0.default_endpoint(default_endpoint))
    }
    #[frb(sync)]
    /// Adds an endpoint to the list of endpoints the client knows of.
    pub fn endpoint(self, endpoint_id: String, endpoint: WrapClientEndpoint) -> Self {
        Self(self.0.endpoint(endpoint_id, endpoint.0))
    }
    #[frb(sync)]
    /// Adds multiple endpoints to the list of endpoints the client knows of.
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
    /// Adds a user token to the list supported by the client.
    pub fn user_token(self, user_token_id: String, user_token: WrapClientUserToken) -> Self {
        Self(self.0.user_token(user_token_id, user_token.0))
    }
    #[frb(sync)]
    /// Sets the maximum outgoing message size in bytes. 0 means no limit.
    pub fn max_message_size(self, max_message_size: usize) -> Self {
        Self(self.0.max_message_size(max_message_size))
    }
    #[frb(sync)]
    /// Sets the maximum number of chunks in an outgoing message. 0 means no limit.
    pub fn max_chunk_count(self, max_chunk_count: usize) -> Self {
        Self(self.0.max_chunk_count(max_chunk_count))
    }
    #[frb(sync)]
    /// Maximum size of each individual outgoing message chunk.
    pub fn max_chunk_size(self, max_chunk_size: usize) -> Self {
        Self(self.0.max_chunk_size(max_chunk_size))
    }
    #[frb(sync)]
    /// Maximum size of each incoming chunk.
    pub fn max_incoming_chunk_size(self, max_incoming_chunk_size: usize) -> Self {
        Self(self.0.max_incoming_chunk_size(max_incoming_chunk_size))
    }
    #[frb(sync)]
    /// Maximum length in bytes of a string. 0 actually means 0, i.e. no string permitted.
    pub fn max_string_length(self, max_string_length: usize) -> Self {
        Self(self.0.max_string_length(max_string_length))
    }
    #[frb(sync)]
    /// Maximum length in bytes of a byte string. 0 actually means 0, i.e. no byte strings permitted.
    pub fn max_byte_string_length(self, max_byte_string_length: usize) -> Self {
        Self(self.0.max_byte_string_length(max_byte_string_length))
    }
    #[frb(sync)]
    /// Maximum number of array elements. 0 actually means 0, i.e. no array permitted
    pub fn max_array_length(self, max_array_length: usize) -> Self {
        Self(self.0.max_array_length(max_array_length))
    }
    #[frb(sync)]
    /// Sets the session retry limit.
    ///
    /// # Panics
    ///
    /// Panics if `session_retry_limit` is less -1.
    pub fn session_retry_limit(self, session_retry_limit: i32) -> Self {
        Self(self.0.session_retry_limit(session_retry_limit))
    }
    #[frb(sync)]
    /// Initial time between retries when backing off on session reconnects.
    pub fn session_retry_initial(self, session_retry_initial: Duration) -> Self {
        Self(
            self.0.session_retry_initial(
                chrono::TimeDelta::to_std(&session_retry_initial)
                    .expect("Failed to convert Duration to std::time::Duration"),
            ),
        )
    }
    #[frb(sync)]
    /// Maximum time between retries when backing off on session reconnects.
    pub fn session_retry_max(self, session_retry_max: Duration) -> Self {
        Self(
            self.0.session_retry_max(
                chrono::TimeDelta::to_std(&session_retry_max)
                    .expect("Failed to convert Duration to std::time::Duration"),
            ),
        )
    }
    #[frb(sync)]
    /// Time between making simple Read requests to the server to check for liveness
    /// and avoid session timeouts.
    pub fn keep_alive_interval(self, keep_alive_interval: Duration) -> Self {
        Self(
            self.0.keep_alive_interval(
                chrono::TimeDelta::to_std(&keep_alive_interval)
                    .expect("Failed to convert Duration to std::time::Duration"),
            ),
        )
    }
    #[frb(sync)]
    /// Set the timeout on requests sent to the server.
    pub fn request_timeout(self, request_timeout: Duration) -> Self {
        Self(
            self.0.request_timeout(
                chrono::TimeDelta::to_std(&request_timeout)
                    .expect("Failed to convert Duration to std::time::Duration"),
            ),
        )
    }
    #[frb(sync)]
    /// Set the timeout on publish requests sent to the server.
    pub fn publish_timeout(self, publish_timeout: Duration) -> Self {
        Self(
            self.0.publish_timeout(
                chrono::TimeDelta::to_std(&publish_timeout)
                    .expect("Failed to convert Duration to std::time::Duration"),
            ),
        )
    }
    #[frb(sync)]
    /// Set the lowest allowed publishing interval by the client.
    /// The server may also enforce its own minimum.
    pub fn min_publish_interval(self, min_publish_interval: Duration) -> Self {
        Self(
            self.0.min_publish_interval(
                chrono::TimeDelta::to_std(&min_publish_interval)
                    .expect("Failed to convert Duration to std::time::Duration"),
            ),
        )
    }
    #[frb(sync)]
    /// Maximum number of pending publish requests.
    pub fn max_inflight_publish(self, max_inflight_publish: usize) -> Self {
        Self(self.0.max_inflight_publish(max_inflight_publish))
    }
    #[frb(sync)]
    /// Sets the session timeout period, in milliseconds.
    pub fn session_timeout(self, session_timeout: u32) -> Self {
        Self(self.0.session_timeout(session_timeout))
    }
    #[frb(sync)]
    /// Sets whether the client should ignore clock skew so the client can make a successful
    /// connection to the server, even when the client and server clocks are out of sync.
    pub fn ignore_clock_skew(self) -> Self {
        Self(self.0.ignore_clock_skew())
    }
    #[frb(sync)]
    /// When a session is recreated on the server, the client will attempt to
    /// transfer monitored subscriptions from the old session to the new.
    /// This is the maximum number of monitored items to create per request.
    pub fn recreate_monitored_items_chunk(self, recreate_monitored_items_chunk: usize) -> Self {
        Self(
            self.0
                .recreate_monitored_items_chunk(recreate_monitored_items_chunk),
        )
    }
    #[frb(sync)]
    /// Maximum number of inflight messages.
    pub fn max_inflight_messages(self, max_inflight_messages: usize) -> Self {
        Self(self.0.max_inflight_messages(max_inflight_messages))
    }
    #[frb(sync)]
    /// Session name - the default name to use for a new session
    pub fn session_name(self, session_name: String) -> Self {
        Self(self.0.session_name(session_name))
    }
}
