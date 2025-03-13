use crate::{
    utils::http,
    types::{
        error::Error,
        rpc::CommitmentLevel
    },
};


/// ### Lightweight custom RpcClient implementation!
/// 
/// I decided to create my own implementation in order to avoid using solana_client crate.
/// 
/// HOWEVER it's highly recommended to use RpcClient from solana_client.
pub struct RpcClient<U = String> 
where
    U: ToString
{
    pub url: U,
    pub http_client: reqwest::Client,
    pub commitment: CommitmentLevel, 
}

impl<U: ToString> RpcClient<U> {
    pub fn new_with_commitment(url: U, commitment: CommitmentLevel) -> Result<Self, Error> {
        let http_client: reqwest::Client = http::init_client()?;
        Ok(Self { 
            url,
            http_client,
            commitment
        })
    } 
}