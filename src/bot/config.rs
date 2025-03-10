pub fn init_env() -> std::io::Result<()> {
    let env_file: String = std::fs::read_to_string(".env")?;

    for line in env_file.lines() {
        if let Some((key, val)) = line.split_once('=') {
            std::env::set_var(key, val);
        }
    }

    Ok(())
}

#[derive(Clone)]
pub struct Config {
    pub ws_url_mainnet: String,
    pub http_url_mainnet: String,
    pub channel_username: String
}

impl Config {
    pub fn init() -> Result<Self, std::env::VarError> {
        Ok(Self {
            ws_url_mainnet: std::env::var("WS_URL_MAINNET")?,
            http_url_mainnet: std::env::var("HTTP_URL_MAINNET")?,
            channel_username: std::env::var("CHANNEL_USERNAME")?
        })
    }
}