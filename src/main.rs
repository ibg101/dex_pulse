mod bot;
mod observations;
mod utils;
mod types;
mod constants;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bot::config::init_env()?;
    pretty_env_logger::init();
    
    let config: bot::config::Config = bot::config::Config::init()?;

    bot::core::run(config).await;

    Ok(())
}

#[tokio::test]
async fn testing() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}