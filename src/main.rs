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
    use types::{custom::{self, Dex}, rpc::{CommitmentLevel, LoadedAddresses}};
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

    // #[test]
    // fn parser() -> Result<(), Box<dyn std::error::Error>> {
    //     // let encoded_data: &str = "g6wxXSFr4ddxH";
    //     // let encoded_data: &str = "jJUxks2v9kQAk";  // transferchecked
    //     // let decoded_bytes: Vec<u8> = bs58::decode(encoded_data).into_vec().unwrap();
    //     // println!("{:x?}", decoded_bytes);  // validating hex
    //     // let token_instruction: TokenInstruction = TokenInstruction::unpack(&decoded_bytes[..])?;
    //     // println!("{:#?}", token_instruction);
    //     // let instruction_accounts: Vec<usize> = vec![7, 15, 8, 0];
    //     // let accounts_keys: Vec<String> = vec![
    //     //     "37TeTPuWZavLJ7Bt6FGm3aP3LcWVHZ2xtsEnZ8dobrPw",
    //     //     "BZZaMHRiwJQ1gSPJibE3ghEwwxjQ1Q87SQYBxS8Xurue",
    //     //     "2pG2qDR2YgCBwGseeuMyg6K5W6pJHukWcnzMYRCd6a7U",
    //     //     "9imDBw8CK9fRMbCw6wkEn4JSrbrHtyWwZHjDbSdjFAQW",
    //     //     "B6Ym1pwZmvSceS9SzJquHBmJBgDypwSn7YZkCbGSU7sz",
    //     //     "BxpvsNNCDypp92SX76Lu29oDQE7z2b8EPSfsFuWbbyXw",
    //     //     "ByY9M6WtEKeMTc5sTcNy8Bv1uRHtJMMc8QXexXsBV6nq",
    //     //     "Cm3z8BTxVQoE94Qt8Q4AYbounhbTEG1mvV5inzGgcaXc",
    //     //     "E3nYUcQ7NXJyRWLBhuzzTELptmkJE3Stvc6WEdww8hwx",
    //     //     "11111111111111111111111111111111",
    //     //     "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
    //     //     "ComputeBudget111111111111111111111111111111",
    //     //     "D1ZN9Wj1fRSUQfCjhvnu1hqDMT7hzjzBBpi12nVniYD6",
    //     //     "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo",
    //     //     "nXaHAMdSdsL9KtpwPJejkCHxcXJCDwkkdDuhFBbpump",
    //     //     "So11111111111111111111111111111111111111112",
    //     //     "SysvarRent111111111111111111111111111111111",
    //     //     "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    //     // ].into_iter().map(|s| s.to_owned()).collect();
    //     // let parsed_i = token_instruction.parse(&accounts_keys, &instruction_accounts)?;
    //     // println!("{:#?}", parsed_i);
        

    //     // let encoded_data: &str = "6ekZrwzFbXXm";  // mintto
    //     // let instruction_accounts: Vec<usize> = vec![4, 10, 16];
    //     // let encoded_data: &str = "3DWrJp21szUo";  // transfer
    //     // let instruction_accounts: Vec<usize> = vec![1, 6, 0];
    //     let encoded_data: &str = "2";  //initializeaccount
    //     let instruction_accounts: Vec<usize> = vec![6, 14, 16, 20];
    //     let decoded_bytes: Vec<u8> = bs58::decode(encoded_data).into_vec()?;
    //     let token_instruction: TokenInstruction = TokenInstruction::unpack(&decoded_bytes[..])?;
    //     println!("{:#?}", token_instruction);
    //     let account_keys: Vec<String> = vec![
    //         "2jkxNJmCksepw53LYDrg9XvdSFby6bhWik2nYDQvUJjg",
    //         "DUPJhy6qfVa6JhTjZqjN6mDUcBXftx6QbAV8wZEMAPxk",
    //         "6osmJdkun8E5YuVY8ugGE8d9HN15WAxyTAfJxx1G9Bn7",
    //         "BqUSk6jmk57DjTu3dAx6iSRPoPQEV92JG8uEfFX94j9w",
    //         "7tYTdVdc8aFBu1P8EJR9y1WfsyvK2vaPJdcLJtCcsAUs",
    //         "2gKXTbKZmN9UKPX3xc1d7a3pkzyoNaPfy2QnenM1SjDH",
    //         "AzgFyZvtaqKCwtLXBgreantHGJ2NsDqE7b6kH2Srdubq",
    //         "7AD4vBn3uYRqyzdn3Qmk8uA1qJpfcDqkiM14EK6gRwcm",
    //         "7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5",
    //         "Epnk2R6KR3w98kdkjDSh1kSvAR4cDQwERqDHpZW7vZSQ",
    //         "2idje6jFYUKiMsM8KVzKuMvFJpPNzyAxTz1F6Rg8grTG",
    //         "ComputeBudget111111111111111111111111111111",
    //         "11111111111111111111111111111111",
    //         "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    //         "So11111111111111111111111111111111111111112",
    //         "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
    //         "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
    //         "BziXodZY5qYTV3ftpGR6ftFSJHRbFgCZwZpZSppCpump",
    //         "9DCxsMizn3H1hprZ7xWe6LDzeUeZBksYFpBWBtSf1PQX",
    //         "9rcwjfrynwQnCDgJYJEkQHGkHqMfdCS4iCV6cTh3BFsw",
    //     ].into_iter().map(|i| i.to_owned()).collect();
    //     let parsed_i = token_instruction.parse(&account_keys, &instruction_accounts)?;
    //     println!("{:#?}", parsed_i);

    //     Ok(())
    // }

    #[test]
    fn testing_account_keys() -> () {
        // struct AccountKeys<'a> {
        //     static_keys: &'a [String],
        //     dynamic_keys: Option<&'a LoadedAddresses>
        // }

        #[derive(Debug)]
        struct Test {
            vec1: Vec<String>,
            vec2: Vec<String>
        }
        
        impl std::ops::Index<usize> for Test {
            type Output = String;

            fn index(&self, index: usize) -> &Self::Output {
                self.get(index).expect("index is invalid")
            }
        }

        impl Test {
            fn iter_segments(&self) -> impl Iterator<Item = &Vec<String>> {
                [&self.vec1, &self.vec2].into_iter()
            }

            fn iter(&self) -> impl Iterator<Item = &String> {
                self.iter_segments().flatten()
            }

            fn get(&self, mut index: usize) -> Option<&String> {
                for segment in self.iter_segments() {
                    let segments_len: usize = segment.len(); 
                    if index < segments_len {
                        return Some(&segment[index]);
                    }
                    index = index.saturating_sub(segments_len);
                }

                None
            }
        }

        let test = Test {
            vec1: vec!["1", "2", "3", "4"].into_iter().map(|i| i.to_owned()).collect(),
            vec2: vec!["5", "6"].into_iter().map(|i| i.to_owned()).collect()
        };
        println!("Item: {:?}", test[4]);
    }
}