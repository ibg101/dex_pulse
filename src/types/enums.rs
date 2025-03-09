#[allow(dead_code)]
#[derive(serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommitmentLevel {
    Processed,
    Confirmed,
    Finalized
}

#[derive(Debug, Clone, Copy)]
pub enum Dex {
    Raydium,
    Meteora
}