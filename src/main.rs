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
    use types::{custom::{self, Dex, AccountKeys}, rpc::{CommitmentLevel, LoadedAddresses}};
    use utils::parser::token_instruction::TokenInstruction;
    use futures_util::Future;


    // #[tokio::test]
    // async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {        
    //     bot::config::init_env()?;
    //     let config: bot::config::Config = bot::config::Config::init()?;
    //     let rpc_client: RpcClient = RpcClient::new_with_commitment(
    //         config.http_url_mainnet.clone(), 
    //         CommitmentLevel::Processed    
    //     )?;
    //     let raydium_signature: &'static str = "2pAnC9xE4nhQJHFggsY8CtxU9wmgiDqxai25J5u4WtxJfSG5giEeX3x7tNAmPAufFmWBmKxB6hQ5zUKwHFunXWGF"; 
    //     // let meteora_signature: &'static str = "2NTyTuYu5zZMnsKAqY7gJfmoQEyj1uKKJEjZcwQPc3DvvFH6XNsXoGnQrVJ6udyjnrYCTTuypvxqTMvnod8XkSVt";
    //     let transaction = rpc_client.get_transaction(raydium_signature, CommitmentLevel::Confirmed).await?;
    //     println!("{:#?}", transaction);
    //     // let processed_tx_raw = Dex::Meteora.meteora_process_transaction(transaction).await?;
    //     // println!("{:#?}", processed_tx_raw);

    //     Ok(())
    // }

    #[test]
    fn parser() -> Result<(), Box<dyn std::error::Error>> {
        // let encoded_data: &str = "6ekZrwzFbXXm";  // mintto
        // let instruction_accounts: Vec<usize> = vec![4, 10, 16];
        // let encoded_data: &str = "3DWrJp21szUo";  // transfer
        // let instruction_accounts: Vec<usize> = vec![1, 6, 0];
        let encoded_data: &str = "2";  //initializeaccount
        let instruction_accounts: Vec<usize> = vec![6, 14, 16, 20];
        let decoded_bytes: Vec<u8> = bs58::decode(encoded_data).into_vec()?;
        let token_instruction: TokenInstruction = TokenInstruction::unpack(&decoded_bytes[..])?;
        println!("{:#?}", token_instruction);
        let static_keys: Vec<String> = vec![
            "2jkxNJmCksepw53LYDrg9XvdSFby6bhWik2nYDQvUJjg",
            "DUPJhy6qfVa6JhTjZqjN6mDUcBXftx6QbAV8wZEMAPxk",
            "6osmJdkun8E5YuVY8ugGE8d9HN15WAxyTAfJxx1G9Bn7",
            "BqUSk6jmk57DjTu3dAx6iSRPoPQEV92JG8uEfFX94j9w",
            "7tYTdVdc8aFBu1P8EJR9y1WfsyvK2vaPJdcLJtCcsAUs",
            "2gKXTbKZmN9UKPX3xc1d7a3pkzyoNaPfy2QnenM1SjDH",
            "AzgFyZvtaqKCwtLXBgreantHGJ2NsDqE7b6kH2Srdubq",
            "7AD4vBn3uYRqyzdn3Qmk8uA1qJpfcDqkiM14EK6gRwcm",
            "7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5",
            "Epnk2R6KR3w98kdkjDSh1kSvAR4cDQwERqDHpZW7vZSQ",
            "2idje6jFYUKiMsM8KVzKuMvFJpPNzyAxTz1F6Rg8grTG",
            "ComputeBudget111111111111111111111111111111",
            "11111111111111111111111111111111",
            "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
            "So11111111111111111111111111111111111111112",
            "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
            "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
            "BziXodZY5qYTV3ftpGR6ftFSJHRbFgCZwZpZSppCpump",
            "9DCxsMizn3H1hprZ7xWe6LDzeUeZBksYFpBWBtSf1PQX",
            "9rcwjfrynwQnCDgJYJEkQHGkHqMfdCS4iCV6cTh3BFsw",
        ].into_iter().map(|i| i.to_owned()).collect();
        let loaded_addresses: LoadedAddresses = LoadedAddresses {
            writable: vec![],
            readonly: vec![
                "SysvarRent111111111111111111111111111111111",
                "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
                "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
            ].into_iter().map(|i| i.to_owned()).collect()
        };
        let account_keys: AccountKeys = AccountKeys::new(&static_keys, Some(&loaded_addresses)); 
        let parsed_i = token_instruction.parse(&account_keys, &instruction_accounts)?;
        println!("{:#?}", parsed_i);

        Ok(())
    }

    // #[test]
    // fn testing_account_keys() -> () {
    //     let static_keys: Vec<String> = vec!["s0".to_owned(), "s1".to_owned()];
    //     let loaded_addresses: LoadedAddresses = LoadedAddresses {
    //         writable: vec!["s2".to_owned()],
    //         readonly: vec!["s3".to_owned(), "s4".to_owned()]
    //     };
    //     let account_keys = AccountKeys::new(
    //         &static_keys,
    //         Some(&loaded_addresses)
    //     );

    //     for i in 0..=4usize {
    //         println!("{}", account_keys[i]);
    //     }
    // }
}