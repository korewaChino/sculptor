pub const LOGGER_ENV: &str = "RUST_LOG";
pub const CONFIG_ENV: &str = "RUST_CONFIG";
pub const LOGS_ENV: &str = "LOGS_FOLDER";

pub const SCULPTOR_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const REPOSITORY: &str = "shiroyashik/sculptor";

pub const USER_AGENT: &str = "reqwest";
pub const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

pub const FIGURA_RELEASES_URL: &str = "https://api.github.com/repos/figuramc/figura/releases";
pub const FIGURA_DEFAULT_VERSION: &str = "0.1.4";