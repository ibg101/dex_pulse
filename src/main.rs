mod bot;
mod rpc;
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
    bot::core::run(config).await?;

    Ok(())
}

#[cfg(test)]
mod test {
    #![allow(unused_imports, dead_code)]
    use super::*;
    use rpc::client::RpcClient;
    use types::rpc::CommitmentLevel;
    use utils::parser::token_instruction::TokenInstruction;
    use futures_util::Future;


    #[tokio::test]
    async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {        
        // bot::config::init_env()?;
        // let config: bot::config::Config = bot::config::Config::init()?;
        // let rpc_client: RpcClient = RpcClient::new_with_commitment(
        //     config.http_url_mainnet.clone(), 
        //     CommitmentLevel::Processed    
        // )?;
        // // let raydium_signature: &'static str = "2wxim9jT7PWM454qybxGRwV5n8NJm3aVPvjpL7qLQg1g1QPddggmh5fjC4HKcWS3iqWHEbQ8sW1hjckRJj5upRRd"; 
        // let meteora_signature: &'static str = "2kfiaSqdxvijrp9dvdHrCAgxKhvHxTeKHnZqa1y2hiPVjQGEEYZxnSHeWfWtuiavhm94yxYwMKBDXHBCaaU3CmiN";
        // let transaction = rpc_client.get_transaction(meteora_signature, CommitmentLevel::Confirmed).await?;
        // println!("{:#?}", transaction);

        Ok(())
    }

    #[test]
    fn parser() -> Result<(), Box<dyn std::error::Error>> {
        // let encoded_data: &str = "g6wxXSFr4ddxH";
        let encoded_data: &str = "jJUxks2v9kQAk";
        let decoded_bytes: Vec<u8> = bs58::decode(encoded_data).into_vec().unwrap();
        println!("{:x?}", decoded_bytes);  // validating hex
        let token_instruction: TokenInstruction = TokenInstruction::unpack(&decoded_bytes[..])?;
        println!("{:#?}", token_instruction);
        let instruction_accounts: Vec<u8> = vec![7, 15, 8, 0];
        let accounts_keys: Vec<String> = vec![
            "37TeTPuWZavLJ7Bt6FGm3aP3LcWVHZ2xtsEnZ8dobrPw",
            "BZZaMHRiwJQ1gSPJibE3ghEwwxjQ1Q87SQYBxS8Xurue",
            "2pG2qDR2YgCBwGseeuMyg6K5W6pJHukWcnzMYRCd6a7U",
            "9imDBw8CK9fRMbCw6wkEn4JSrbrHtyWwZHjDbSdjFAQW",
            "B6Ym1pwZmvSceS9SzJquHBmJBgDypwSn7YZkCbGSU7sz",
            "BxpvsNNCDypp92SX76Lu29oDQE7z2b8EPSfsFuWbbyXw",
            "ByY9M6WtEKeMTc5sTcNy8Bv1uRHtJMMc8QXexXsBV6nq",
            "Cm3z8BTxVQoE94Qt8Q4AYbounhbTEG1mvV5inzGgcaXc",
            "E3nYUcQ7NXJyRWLBhuzzTELptmkJE3Stvc6WEdww8hwx",
            "11111111111111111111111111111111",
            "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
            "ComputeBudget111111111111111111111111111111",
            "D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6",
            "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
            "nXaHAMdSdsL9KtpwPJejkCHxcXJCDwkkdDuhFBbpump",
            "So11111111111111111111111111111111111111112",
            "SysvarRent111111111111111111111111111111111",
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        ].into_iter().map(|s| s.to_owned()).collect();
        let parsed_i = token_instruction.parse(accounts_keys, instruction_accounts)?;
        println!("{:#?}", parsed_i);
        
        Ok(())
    }
}