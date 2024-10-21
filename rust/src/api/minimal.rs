use anyhow::Result;
use flutter_rust_bridge::frb;
pub use std::path::Path;
use std::{collections::BTreeMap, path::PathBuf, time::Duration};
#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

pub fn minimal_adder(a: i32, b: i32) -> i32 {
    a + b
}

pub use opcua::client::{Client, ClientBuilder, ClientConfig, ClientEndpoint, ClientUserToken};
pub use opcua::crypto::SecurityPolicy;
pub use opcua::server::prelude::Performance;
pub use opcua::types::DecodingOptions;
pub use opcua::types::DepthGauge;
// #[frb(mirror(Path))]
// pub struct _Path {
//     inner: std::ffi::OsStr,
// }
// #[frb(external)]
// impl Path {
//     pub fn new(inner: String) -> Self {}
// }

#[frb(mirror(ClientUserToken))]
pub struct _ClientUserToken {
    /// Username
    pub user: String,
    /// Password
    pub password: Option<String>,
    pub cert_path: Option<String>,
    pub private_key_path: Option<String>,
}
#[frb(external)]
impl ClientUserToken {
    // pub fn user_pass<S, T>(user: S, password: T) -> Self
    // where
    //     S: Into<String>,
    //     T: Into<String>,
    // {
    // }
    pub fn user_pass(_user: String, _password: String) -> Self {}
    // pub fn x509<S>(user: S, cert_path: &Path, private_key_path: &Path) -> Self
    // where
    //     S: Into<String>,
    // {
    // }
    // pub fn x509(_user: String, _cert_path: &Path, _private_key_path: &Path) -> Self {}
    pub fn is_valid(&self) -> bool {}
}

#[frb(mirror(SecurityPolicy))]
pub enum _SecurityPolicy {
    Unknown,
    None,
    Aes128Sha256RsaOaep,
    Basic256Sha256,
    Aes256Sha256RsaPss,
    Basic128Rsa15,
    Basic256,
}

#[frb(mirror(ClientEndpoint))]
pub struct _ClientEndpoint {
    /// Endpoint path
    pub url: String,
    /// Security policy
    pub security_policy: String,
    /// Security mode
    pub security_mode: String,
    /// User id to use with the endpoint
    pub user_token_id: String,
}
#[frb(external)]
impl ClientEndpoint {
    pub fn new(_url: String) -> Self {}
    #[frb(name = "getSecurityPolicy")]
    pub fn security_policy(&self) -> SecurityPolicy {}
}

#[frb(mirror(ClientConfig))]
pub struct _ClientConfig {
    /// Name of the application that the client presents itself as to the server
    pub application_name: String,
    /// The application uri
    pub application_uri: String,
    /// Product uri
    pub product_uri: String,
    /// Autocreates public / private keypair if they don't exist. For testing/samples only
    /// since you do not have control of the values
    pub create_sample_keypair: bool,
    /// Custom certificate path, to be used instead of the default .der certificate path
    pub certificate_path: Option<PathBuf>,
    /// Custom private key path, to be used instead of the default private key path
    pub private_key_path: Option<PathBuf>,
    /// Auto trusts server certificates. For testing/samples only unless you're sure what you're
    /// doing.
    pub trust_server_certs: bool,
    /// Verify server certificates. For testing/samples only unless you're sure what you're
    /// doing.
    pub verify_server_certs: bool,
    /// PKI folder, either absolute or relative to executable
    pub pki_dir: PathBuf,
    /// Preferred locales
    pub preferred_locales: Vec<String>,
    /// Identifier of the default endpoint
    pub default_endpoint: String,
    /// User tokens
    pub user_tokens: BTreeMap<String, ClientUserToken>,
    /// List of end points
    pub endpoints: BTreeMap<String, ClientEndpoint>,
    /// Decoding options used for serialization / deserialization
    pub decoding_options: DecodingOptions,
    /// Maximum number of times to attempt to reconnect to the server before giving up.
    /// -1 retries forever
    pub session_retry_limit: i32,

    /// Initial delay for exponential backoff when reconnecting to the server.
    pub session_retry_initial: Duration,
    /// Max delay between retry attempts.
    pub session_retry_max: Duration,
    /// Interval between each keep-alive request sent to the server.
    pub keep_alive_interval: Duration,

    /// Timeout for each request sent to the server.
    pub request_timeout: Duration,
    /// Timeout for publish requests, separate from normal timeout since
    /// subscriptions are often more time sensitive.
    pub publish_timeout: Duration,
    /// Minimum publish interval. Setting this higher will make sure that subscriptions
    /// publish together, which may reduce the number of publish requests if you have a lot of subscriptions.
    pub min_publish_interval: Duration,
    /// Maximum number of inflight publish requests before further requests are skipped.
    pub max_inflight_publish: usize,

    /// Requested session timeout in milliseconds
    pub session_timeout: u32,

    /// Client performance settings
    pub performance: Performance,
    /// Session name
    pub session_name: String,
}

pub fn testme(config: ClientConfig) {
    println!("testme: {:?}", config);
}
