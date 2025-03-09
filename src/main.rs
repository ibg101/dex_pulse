mod bot;
mod observations;
mod processing;
mod utils;
mod types;
mod constants;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    bot::config::init_env()?;
    pretty_env_logger::init();
    
    let config: bot::config::Config = bot::config::Config::init()?;
    
    log::info!("Starting bot!");
    bot::core::run(config).await;

    Ok(())
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;

    use futures_util::Future;


    #[tokio::test]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {        
        caller(cally).await;
        Ok(())
    }

    async fn caller<F, Fut>(closure: F) -> () 
    where
        F: Fn() -> Fut,
        Fut: Future<Output = ()>
    {
        closure().await;
    }

    async fn cally() -> () {
        println!("im called!");
    }
}