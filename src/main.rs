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
    use crate::types::custom::{PairMeta, Parser, SharedTokenMeta, LPTokenMeta, Unpack};

    use super::*;
    use rpc::client::RpcClient;
    use types::{error, custom::{self, Dex, AccountKeys}, rpc::{CommitmentLevel, LoadedAddresses}};
    use utils::parser::{
        token_instruction::TokenInstruction,
        system_instruction::SystemInstruction,
        pumpswap::{
            AMMAnchorCPILog,
            // CreatePoolInstruction
        },
        account::AccountType,
    };
    use futures_util::Future;


    #[tokio::test]
    async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {        
        bot::config::init_env()?;
        let config: bot::config::Config = bot::config::Config::init()?;
        let rpc_client: RpcClient = RpcClient::new_with_commitment(
            config.http_url_mainnet.clone(),  
            CommitmentLevel::Processed    
        )?;
        // let raydium_signature: &'static str = "2R9NKfTTxSSsZ2c59tFcNzZoMPq4rgC364PuruJumG1iLki7pmv7BQLyajT6LGteWP9CUZkgfBAT9iLEkAorYxDo"; 
        // let meteora_signature: &'static str = "487swy1ZX9eNuQPdCasVD1fvWTboQNewowavcat7ejPukcqGkDaw35ApCUpzneQnznGPqAejVtKCKjfpsEvA4WxQ";
        // let meteora_signature: &'static str = "67EUGqosmoQFHPyjaSmQsh8dRUQzqQzVaEfwmhQWZiKsCjxsTKqSojUkUC8Thc2TyyBz4Woq8CvMsAmJwBnneW4F";
        // let pumpswap_signature: &'static str = "2g66jSg8j6Tgyd9Js2wmR5U56hZpL4mkBpRfMdXoTJDrJHyfsodF38pEAv3CUyBNJubfboSxRotUbxzpvM2JBEif";
        let pumpswap_signature: &'static str = "4ejDPRGBYF43zg8Hpu8gApjZN8yurQkcXNwnMXVKrzQZQHexFXFqVd1VLFZpa6eGWz39Vxo6Nhbz99aKbhz3CfAv";
        // let pumpswap_signature: &'static str = "bfVLmwBzDgNrpycKXHXWg1eWb9r2DJuDMWAfGFRzbenQZBzTCmM7f9VfP5sesFyGdTv4eqb7W9f74u1tckaJ7V2";
        // let pumpswap_signature: &'static str = "4DSjpA5wVoYiK8w47KoffJKZi25nuVXJfb85EjKn9Fd7iGDTrat5as7vijjBmhLYg8EtohDhiD3uBy2qaDcb35WV";
        let transaction = rpc_client.get_transaction(pumpswap_signature, CommitmentLevel::Confirmed).await?;
        println!("{:#?}", transaction);
        // let processed_tx_raw = Dex::PumpSwap.process_transaction(transaction).await?;
        // println!("{:#?}", processed_tx_raw);

        // let not_mintable_not_freezable: &str = "y7D9BxVeQ5iwwd7yC8R3VsW1prWpsPkcnq63eSupump";
        // let mintable_freezable: &str = "4CUAn6CgkcirqTQ9nmpcFtYNaDT3vgWTCZjPL7Tp7Eei";
        // let account_info = rpc_client.get_account_info("hdsUgquX4iXf2Faqx9nzExdbVyCu62VZMXNNYrSpump").await?;
        // println!("account info: {:#?}", account_info);

        Ok(())
    }

    // #[test]
    // fn parse_from_raw_bytes_slice() -> Result<(), Box<dyn std::error::Error>> {
    //     let data = "rLaD5MVJGTSekbeMDJ6HPt5nvuHSPtLvwkvagDoeu4iossn61mgzs2ifSeRk5XbhRLaXVM9d1ZUnXdWEq1BHB8BnH49ezrdw6cnaYMyKxQ4L7o2LY1wuVRxVsUpEWJh6y4FsdwCb2dnhKTvYTgUMWTXv5KYnB9aayYDCQYPhwsfirfhTdxijzD9R33rcx5u5Quf9aaFV8gkAaZ3Shs4tTak58AT2VKdNNa6zm8Wba6BwoHhj7LaVuxZU7G1zNkiqMhZYTEYJPQxMvg5ogdGbvofxr7G6xrr14P2ecCKcHcAovMETBRq1WMyN7DuN6c15p1jGPfKWYyAHTiDfBbFAHMX9AwqpmdeVkxrGkHAAktRZDc2ZeGefV6PZvCGUqqTsmDNdJZV7Up9syFf5R95SE8xgkPzBkBWZ6EFfrS";
    //     // let data = "rLaD5MVJGTSekbeMDJ6HPtMevyYBuFn4aoy9n3DJMAggCWiUSxhxynVsazyJ5KjeZ5C2r1RtTBhLXMeEwqaeAJeTJwcmejQHSTzJ22zS4hLjru5BtULuAKe28t3SjscvboVVYjAzpD5tVdEBpG6VpREWeaik6V6WG6bgKAbgNai5wxj9yTcy97i2eNKJ7xZykXURhD3EG66Th3HFo9u7CVM7oftC1nNLAVEpzdXMqTgxfLynHK1PUKpLCdirS4BqKkfnbz4uAUm6AkZ9fApQgNBxdohG1XV1HJMR4XS56vLSPsqjuuU7ZxJs1bka836ubrHtfyG22DWc7dEp9azCUgZCmvQZz2DWgv1SCERUH7CnZhwRv2DrvbKC2CPyinqmhNrgENkYuhRVbu5KezBearBfp5u5Wwb9Z4uD8i";
    //     let bytes: Vec<u8> = bs58::decode(data).into_vec()?;

    //     let dev_key_encoded: &str = "CbVJyp4R9NQwRJKjpyp1QQ2AYZeuMmUjjTP2KHB4Sa7a";
    //     let dev_key_bytes: Vec<u8> = bs58::decode(dev_key_encoded).into_vec()?;

    //     // for (i, el) in bytes.iter().enumerate() {
    //     //     if el == &dev_key_bytes[2] {
    //     //         println!("index: {} - value: {}", i, el);
    //     //     }
    //     // }

    //     println!("is equal: {:?}", &bytes[26..58] == dev_key_bytes);

    //     println!("{}", bs58::encode(&bytes[26..58]).into_string());
    //     Ok(())
    // }

    // #[test]
    // fn tx_instruction_parser() -> Result<(), Box<dyn std::error::Error>> {
    //     // let encoded_data: &str = "6ekZrwzFbXXm";  // mintto
    //     // let instruction_accounts: Vec<usize> = vec![4, 10, 16];
    //     // let encoded_data: &str = "3DWrJp21szUo";  // transfer
    //     // let instruction_accounts: Vec<usize> = vec![1, 6, 0];
    //     let encoded_data: &str = "2";  //initializeaccount
    //     let instruction_accounts: Vec<usize> = vec![6, 14, 16, 20];
    //     let decoded_bytes: Vec<u8> = bs58::decode(encoded_data).into_vec()?;
    //     let token_instruction: TokenInstruction = TokenInstruction::unpack(&decoded_bytes[..])?;
    //     println!("{:#?}", token_instruction);
    //     let static_keys: Vec<String> = vec![
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
    //     let loaded_addresses: LoadedAddresses = LoadedAddresses {
    //         writable: vec![],
    //         readonly: vec![
    //             "SysvarRent111111111111111111111111111111111",
    //             "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
    //             "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
    //         ].into_iter().map(|i| i.to_owned()).collect()
    //     };
    //     let account_keys: AccountKeys = AccountKeys::new(&static_keys, Some(&loaded_addresses)); 
    //     let parsed_i = token_instruction.parse(&account_keys, &instruction_accounts)?;
    //     println!("{:#?}", parsed_i);


        // --pumpswap--
        // ANCHOR CPI LOG
        // let data = "rLaD5MVJGTSekbeMDJ6HPtvCzxUm9GCBf3DDLoPhXkK6UKFhXRcdSpcvuBoUUuZvfYKE9d6agFVnXVP2sWq3q12zu2xkYSSzjj1X1BQX2gwAiAzQsNZwXkxQrX7feS4gjArGaZ9KJCh88cGTrhKwYuVPZhPDRahSJktBQxQSjRbzxVHAVb4CASZ57LxuEfQ2nS1peHtF7Wu5A2QTFvFXQB2c7Y45v3tVvGcSKoAaPEymnfnnP1WUhe4sVvsRk4nVhzb7g2zd7zcqJaNYdCe2TTe2LpdDJH7FXDig8hsmATzgCsc87qkRYZqXETerXjiLcGX6KnLPPumdGtU5EJwDGV3dMP8AusbwnzMx5qZb63Vha1AjpPjK8ZkVG9gJc4T5X9VjoU5xmVACs6bvsHskXmFzYAjqUrWt8AnHSr";
        // let decoded_bytes: Vec<u8> = bs58::decode(data).into_vec()?; 
        // println!("bytes len: {}", decoded_bytes.len());

    //     let data = "7PDgJJxP9Deo";  // burn
    //     // let data = "6VGqp5KosJBrKqsHqEcQpmghJdAt5jzqvZK2c326TVamK";  // initializeaccount3
    //     // let data = "6eQA6kJksZAP";  // token 2022 mintto 
    //     let decoded_bytes: Vec<u8> = bs58::decode(data).into_vec()?;
    //     let token_instruction: TokenInstruction = TokenInstruction::unpack(&decoded_bytes)?;
    //     let static_keys: Vec<String> = vec![
    //         "999998H51LaVaPCK9TDVHiFv8HJGLFaDrPrkYSQeCSLM",
    //         "E1W75quKfhBG7EKnXhTGfEYrsMd7u4KLkaCnVFpM33XE",
    //         "2tJrSwhxyqujGDd16Hyh9APHXhJNE6Qo2uatPmvk2Eob",
    //         "2LupHit5nuc6Vg57oWRzseNDDp4yP3R1Tyy2mPDv6p1D",
    //         "5ENtXSq7N6pGKRS38xcMNFpazhnS3cnbAnnc5TUkh56o",
    //         "4TiKCzUtzvcd1BBzB4k7op43s1P8Sc67JFAG9UwL3ib5",
    //         "BGNvHv9w7RHLhYBtRpw7d4Rv8m4CtfbkbCmEELPrcRHK",
    //         "7FFbg7vH9t3Xto6CdRiWdt8tdYBbBJ5GrdQCXRWRZLEr",
    //         "3n4yMk1jVagqvf5NizF1TjXcAhwCSxaNnR7ssQMVAREv",
    //         "663B2NrEuevDY5TNrtgGggeDoUM7xBZfA2nKacT2eTc3",
    //         "GWMvvvJspDyNfKmyKoc1N1gTCJiKkctuG6v5idRFuscF",
    //         "AkfEzSkGHadJhWyhJcicgD9PYMUuDrPY1SD4Gac2BeCh",
    //         "6T6oxn3AFkQKVQnjzSynYGRmkNobFSf89nV8mAS5v3iH",
    //         "ComputeBudget111111111111111111111111111111",
    //         "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
    //         "hdsUgquX4iXf2Faqx9nzExdbVyCu62VZMXNNYrSpump",
    //         "BBBBBBZy6FZkuzYrCvYh4DdwGZZMXFeKHZXXMq9vUCHN",
    //         "So11111111111111111111111111111111111111112",                
    //     ].into_iter().map(|i| i.to_owned()).collect();
    //     let loaded_addresses: LoadedAddresses = LoadedAddresses {
    //         writable: vec![
    //             "62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV",
    //             "39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg",
    //             "ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw",
    //             "94qWNrtmfn42h3ZjUZwWvK1MEo9uVmmrBPd2hpNjYDjb",
    //         ].into_iter().map(|i| i.to_owned()).collect(),
    //         readonly: vec![
    //             "11111111111111111111111111111111",
    //             "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    //             "4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf",
    //             "SysvarRent111111111111111111111111111111111",
    //             "Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1",
    //             "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    //             "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA",
    //             "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
    //             "GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR",
    //         ].into_iter().map(|i| i.to_owned()).collect()
    //     };
    //     let account_keys = AccountKeys::new(&static_keys, Some(&loaded_addresses));
    //     let instruction_accounts: Vec<usize> = vec![9, 8, 5];  // burn
    //     // let instruction_accounts: Vec<usize> = vec![1, 15];  // initializeaccount3
    //     // let instruction_accounts: Vec<usize> = vec![8, 9, 4];  // token 2022 mintto
    //     let parsed_i = token_instruction.parse(&account_keys, &instruction_accounts)?;
    //     println!("{:#?}", parsed_i);

    //     Ok(())
    // }

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

    // #[test]
    // fn account_parser() -> Result<(), Box<dyn std::error::Error>> {
    //     // not_mintable_not_freezable
    //     // let encoded_data: &str = "11112q78wWJ3FypoJp7jT6jY5cpcDSCskau97g9pweEocSsEvKaQ576oaqnc9K8HBRj5F6Vp4XZaxK83B4QEnNemLpDKmHeZSZhYc4jQ991Wsuh";
    //     // let encoded_data: &str = "1111Dk7tnoddMvATwtoKYbhf9c51kPxy4Siv5Ubb93zssnp2NB4385QmUMWoc6it7sxezXmUX58o5SjkiaMfEp9QenSyKJHLDUuJJBQXz2r7yZ";
    //     // mintable_freezable
    //     // let encoded_data: &str = "DK9MzeHSprngGXWSrcu6oBsYBs9rNTiKy3CjDqNN3uJrUry7MFuy86u316TCbmX2Xr7ZYvmE6rkCrGFxzJVdkahanvjhEHowpjLFog8mBnHGutw";
    //     let encoded_data: &str = "DK9N4DzFoA11J4H9JqhtRCAJbt1Z4CbmR9F5yhoo65zo5wagP5GqqZHLyYCKwFmyib7bQVAGjhCc5byek39dDceTqrg7uQ228N6BzfovVho41M8";
    //     let bytes: Vec<u8> = bs58::decode(&encoded_data).into_vec()?;
    //     println!("bytes len: {}", bytes.len());
    //     println!("{:#?}", AccountType::unpack(&bytes)?);

    //     Ok(())
    // }

    // #[test]
    // fn system_instruction_parser() -> Result<(), Box<dyn std::error::Error>> {
    //     let encoded_data: &str = "SYXsG5gxn13RGVJBuJ66WMvnpkuC3ZXmxCAkmzi1nLhi459e";  // assign
    //     let bytes: Vec<u8> = bs58::decode(encoded_data).into_vec()?;
    //     let system_instruction = SystemInstruction::unpack(&bytes)?;
    //     let instruction_accounts: Vec<usize> = vec![2];
    //     let static_keys: Vec<String> = vec![
    //         "Ahu5JLfLnCQ12tehXFqn9ZxWpvxkhoByUK8jYS73yt5L",
    //         "852doLyXTw9z1aUdx1t4sj6PvDhyd4JLfHZeWEFz1L71",
    //         "CSKdZyehCa42K26W4njCW3RrHb9q5Wvyki3A2qd9rJ77",
    //         "8bXC3NS7qoRMjpLko3jd82scJvFng9Z6fvvF14tmnWtn",
    //         "26czTm9nAaPtYWTBBoMV6PjgC96puKCP2DD9upNniHch",
    //         "BZLYwHENfNtk1LpeJWanr8GWwyCf3EFxamxPvjekKeJT",
    //         "7mTTn2b1FyVkTcHsSWBUCcRx86Ynv9nzWQJcXkEPjQH4",
    //         "Gtk2paEfXSqVdkMVafLquMQiZgjc5K8HNtKSi3LobfsT",
    //         "7YttLkHDoNj9wyDur5pM1ejNaAvT9X4eqaYcHQqtj2G5",
    //         "F1AgaLPeGz7nGu3EWXD377kXLNYvm4AvH3EdR2QmA8Dy",
    //         "7BE25tn1jem1PHUk9g6s9FMGoAZCFzNcJiVdRWjtfgKw",
    //         "ComputeBudget111111111111111111111111111111",
    //         "11111111111111111111111111111111",
    //         "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    //         "So11111111111111111111111111111111111111112",
    //         "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
    //         "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1",
    //         "GB87YcxkYGJNsfAq4hWR1ycm1mgssPRJvyWj1JSpump",
    //         "9DCxsMizn3H1hprZ7xWe6LDzeUeZBksYFpBWBtSf1PQX",
    //         "7yUYnJVbraMX3gLo7oQaG4n87ktozFpp2K73pTw99WfL",
    //     ].into_iter().map(|i| i.to_owned()).collect();
    //     let loaded_addresses: LoadedAddresses = LoadedAddresses {
    //         writable: vec![],
    //         readonly: vec![
    //             "SysvarRent111111111111111111111111111111111",
    //             "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
    //             "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX",
    //         ].into_iter().map(|i| i.to_owned()).collect()
    //     };
    //     let account_keys = AccountKeys::new(&static_keys, Some(&loaded_addresses));
    //     let parsed_i = system_instruction.parse(&account_keys, &instruction_accounts)?;
    //     println!("{:#?}", parsed_i);

    //     Ok(())
    // }

    // #[test]
    // fn meteora_parse_market_id_from_anchor_cpi_log() -> Result<(), Box<dyn std::error::Error>> {
    //     let encoded_data: &str = "3drYVtAcBYiKzmNPkzG2oeeV8xJDpYQ8QJHMaN1Yy9xzv4iwVZNbHv2MVPVTcJqLWGQrPzd5kHZDVWCRGK47GjpGFB2nn56tx5FURwCtxsgNV67Tbczw58He4sQHZiQ5WLEbmBZErL7nQkpZh1LZpdMxnj3fKrtGsdpyTsE7iQnEAkDhSqX6N";
    //     // let encoded_data: &str = "3drYVtAcBYiKzmNPkzG2oee5T9HqsdFuBqY8daxpjQpsFmKdgEW2UMqJt4jedPSksNm774bpoX3Ar7vT4UKJYfyh3WV4y8izmodKLYv7ZvSsQkEeZtkLF7L2JPYSENMLLaf2ozyaG5ZBwbPEPDhqrz7biDbBJFkbjd1QETwh5LRZnfXKTzWai";
    //     // let encoded_data: &str = "3drYVtAcBYiKzmNPkzG2oegL4SNcPQMkFbQ1uc7NwfcuVphjkSb6z3aKFkLmi6S8wYAaL2FMSGAvxk1t3YzBsg2qX17jwot3X2Z2YNR5zX1D8F5UvCYLHJG3sSnk9bf4gu4DM1DjtLKB2rMvAQXh7K57UKbAL7ZAc7n3zUbFSJHaf5tm1kDQb";
    //     let bytes: Vec<u8> = bs58::decode(encoded_data).into_vec()?;

    //     println!("size: {}", bytes.len());

    //     println!("{}", bs58::encode(&bytes[16..48]).into_string());  // lb pair

    //     Ok(())
    // }

    // #[tokio::test]
    // async fn build_post() -> Result<(), Box<dyn std::error::Error>> {
    //     use teloxide::{
    //         Bot,
    //         prelude::Requester, 
    //         payloads::SendMessageSetters, 
    //         types::ParseMode::MarkdownV2, 
    //     };

    //     bot::config::init_env()?;
    //     pretty_env_logger::init();

    //     let test_pair_meta: PairMeta = PairMeta {
    //         base: SharedTokenMeta {
    //             mint_account: Some(AccountType::Mint {
    //                 mint_authority: None,
    //                 supply: 10000000000000000,
    //                 decimals: 6,
    //                 is_initialized: true,
    //                 freeze_authority: None,
    //             }),
    //             mint: "BiFFcvhZtyjYYag5mERPMwE78AkNm1kecbecYqL9j4Vm".to_owned(),
    //             vault: "DKXWNzTexESPyfsdAu2aLWCz6pupz37u7syh6FqTroUJDKXWNzTexESPyfsdAu2aLWCz6pupz37u7syh6FqTroUJ".to_owned(),
    //             provided_liq_amount: 632831287178,
    //             provided_liq_ratio: Some(
    //                 0.006328312871780001,
    //             ),
    //         },
    //         quote: SharedTokenMeta {
    //             mint_account: None,
    //             mint: "So11111111111111111111111111111111111111112".to_owned(),
    //             vault: "48444tr6CaL2i8WcY16dMa81S1y6CQhBCRq556ACdCZb48444tr6CaL2i8WcY16dMa81S1y6CQhBCRq556ACdCZb".to_owned(),
    //             provided_liq_amount: 79010000000,
    //             provided_liq_ratio: None,
    //         },
    //         signers: vec![
    //             "4vn9jGm463jsdVJtss9wQUVXRFu99cs9gtRXCowmZiVL".to_owned(),
    //         ],
    //         raydium_related: Some(
    //             PairMetaRaydium {
    //                 lp_mint: "826thAa3anaB2B6w6KYHepAZZgKrQnAEpm1M2fYWpoLG".to_owned(),
    //                 lp_tokens_minted_amount: 223605797749,
    //             },
    //         ),
    //         market_id: "EoTVDCVpU4yq7pfxeAYEM4zcuGr3EvTci2Ug2nXSM2kP".to_owned()        
    //     };
        
    //     let post = processing::tg::build_post_as_string(test_pair_meta);
    //     let bot = Bot::from_env();
    //     bot.send_message("@dex_pulse_scanner".to_owned(), post).parse_mode(MarkdownV2).await?;
        
    //     Ok(())
    // }

    // #[test]
    // fn parse_create_pool() -> () {
        // let static_account_keys = vec![
        //     "999998H51LaVaPCK9TDVHiFv8HJGLFaDrPrkYSQeCSLM",
        //     "86baHMQKmNAbkPaAPvxjxHtgKEmSwHxViD3Cq95NfxRE",
        //     "3zTUyUwCx2KQYWCfpkotRSR7AuYyRJHLRh981kZSaYAg",
        //     "8hMRpndoq2ssLGXSWeDSakK9sLRfTF4oJ1XG8UWckcYV",
        //     "5upCJsFQK9cbzejXCskfqq3ZADsFxe1SG9hoJ3gsqGTP",
        //     "Bp717hZ4uTbLxZXjurxrfAtwKeXpfrdHX14fqhZdRPuV",
        //     "37KFah7zDCUbvxdmpzw8dxf4hW2HtQfsAJjYZVNGeRk4",
        //     "Ahu6mifSoaHyuqfBXEmEFH9Sevs5sX27V39inNGjw9XC",
        //     "BgUPrQy7wM4dpfw3Wkgv44hHyPynCyTUaAnYt4NyG21",
        //     "Gdhy34c3HPa46ssjBxzYuEfS2ZpBnvrAQWi3EMeQA7tv",
        //     "FVC1GbffdD54vjpLGKMkHPFwqdbQRxQajgyLJxqM6gLr",
        //     "2MknRtvgBzwTJbJhUbiUdzpahhDGwE7xiXBhEbcCWBDs",
        //     "6T6oxn3AFkQKVQnjzSynYGRmkNobFSf89nV8mAS5v3iH",
        //     "ComputeBudget111111111111111111111111111111",
        //     "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
        //     "BZ3cK6MVJ2FWzuoBci69wdcNM4LGsMNaHpc8Yin8pump",
        //     "BBBBBBZy6FZkuzYrCvYh4DdwGZZMXFeKHZXXMq9vUCHN",
        //     "So11111111111111111111111111111111111111112",
        // ].into_iter().map(|addr| addr.to_owned()).collect::<Vec<_>>();
        // let dynamic_keys = LoadedAddresses {
        //     writable: vec![
        //         "62qc2CNXwrYqQScmEdiZFFAnJR262PxWEuNQtxfafNgV",
        //         "39azUYFWPz3VHgKCf3VChUwbpURdCHRxjWVowf5jUJjg",
        //         "ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw",
        //         "94qWNrtmfn42h3ZjUZwWvK1MEo9uVmmrBPd2hpNjYDjb",
        //     ].into_iter().map(|addr| addr.to_owned()).collect::<Vec<_>>(),
        //     readonly: vec![
        //         "11111111111111111111111111111111",   // 22 index
        //         "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
        //         "4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf",
        //         "SysvarRent111111111111111111111111111111111",
        //         "Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1",
        //         "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
        //         "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA",
        //         "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
        //         "GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR",
        //     ].into_iter().map(|addr| addr.to_owned()).collect::<Vec<_>>()
        // };

        // let account_keys = AccountKeys::new(&static_account_keys, Some(&dynamic_keys));
        // println!("{:#?}", account_keys[22]);


        // let data = "89qBdnKbVfeW4XmoFaNf32P6jSYLhkcG7fhR";
        // // let data: &str = "38ENJsBN1u5UXLrqwVDVJbdJezJ5ENpYk2YRMBuX8dY2kGT7kj21VUuSzdBRhFjrU1ibVWLEdg56bBwL";
        // let bytes: Vec<u8> = bs58::decode(data).into_vec().unwrap();
        // let create_pool = CreatePoolInstruction::unpack(&bytes).unwrap();
        // println!("{:#?}", create_pool);
    // }

    #[test]
    fn parse_dev_from_anchor_cpi_log() -> Result<(), Box<dyn std::error::Error>> {
        // let data = "Ff4kLs7v3LMkTXcjPPitKpfc2geMRyqubgywNUfp5vGuQ6JTSUWw7Qfzm6FKxUawu9v8EYx9NEt4DKWyRM6tFyq93iegXtCYjc7iHxT17A3LVnttXp5dKSKvBcV25fKoPEGovPaS47Na8swCge9LAFXUCULaNFb8VEb7FTEJYXLsbJSuat2cspq42XQDDc2TsvURPKh2XN7zeL3EbP4b6hj6CRQxFwXoJQi7EdtM5FkLeq9WB9xCAVgQ9AjU7Em4hUyPbKGWcs3SAGdrm9VAFM32LFKUoknyUkbJh25bzJ32ordsHfFRu4L2ps7S6qTPW16byCh4rG8TDV8tKLamHCxPmvj57VvgChLwPHT1cM4zvCgs37VurudDxD7AqNCFbEKrR5bwhHk5q1QpQYu5Tj2pMNrrhExSV8TYMvqSWqQ1FacBnSqEcLLTBudkVfEvzFpV8bmZcNi3KBvMWE";
        // let data = "rLaD5MVJGTSekbeMDJ6HPsjCNCh2MRLasnrK6A8jAS1Mh65PBKKHKWspjRHmgrx3BxbxiCN714xMTyWXut9RTD2vthdekLQxgzvRvuT9Ck4UBuXFqnuKWihC4Xu5dTQu6of1vDyP76i3QsMk792En4yLHygSMyEwnK4vze3tyAxKsTbUmuLk7FqHyhUvCGF6YHsPKktHh2bhZ4hwaH5DWJTD7vcTtNJKY9caGL8pq4AQtuQzH8TiiFizXW9AGockT9G38Mz5RasZQxyx5MoCSL8h9Ztk3NRRFvmJKNtbNPKE5oZrxeEgbfxQFQCR9QDNK4WAqFY6jJsN2UiEpCmonPT1HTNxEsAHc8WfKNa3uDNSmG5JLsgKeZyDJ7w9UQVa8qdrGugGFCQFuMT9rTK2ZTrosswjE78eXA8WXY";
        let data = "Ff4kLs7v3LMkTXcjPPitKpRzref5wAPmFqDPqZgWAVwQh1q69dGqgrtbYyh5eAkbkypPfF4huwNxqS1ocpxVvYfPVFGMgs5cM6kQ62CkecxMjtc9vohW2cWqRowwHURxQyCe8vdUqJTcWgaCk5TjJGePiuPU7XLKVTMAxR8mh4tEe8FJRjtiuNuWNfZ6oEVAdkWdcywDXHGSTqm6cz7W31BPPffTSy3kU4Y6oyidvU7dFBnuRA1S3M5qaqNYP6vySCLMZrkevXrjarcbXQQHVMp6NGiKsASFWbY7NJ6U4Hwv18C6ZCWP1zULVEc8nR5WWt13SHUqkR5wk4PbaZ24KLytUhCzA2fsKrYKqgq5ciQw45ewHkPfb7EZ1YJ36fZzbVYGrkAmYgbo8KzoeAQM5EcPAbThXBjQHMeQoCKWtZkrfY8kni4g93LwD5vAxazXqLMNPqpkZjDyUMHmKV";
        let bytes: Vec<u8> = bs58::decode(data).into_vec()?;
        println!("{:?}", AMMAnchorCPILog::try_parse_creator(&bytes)?);
        
        Ok(())
    }
}