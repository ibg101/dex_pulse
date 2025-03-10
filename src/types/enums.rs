#[allow(dead_code)]
#[derive(serde::Serialize, PartialEq)]
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

#[derive(Debug)]
pub enum Error {
    CreateHttpClient,
    UnsupportedCommitment,
    ReachedMaxRetries
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: &str = match self {
            Self::CreateHttpClient => "Failed to initialize HttpClient!",
            Self::UnsupportedCommitment => "Attempted to use an unsupported CommitmentLevel.\n\
            Please verify that the RPC method you are using supports this commitment level.",
            Self::ReachedMaxRetries => "Reached max retries while calling RPC method!"
        };

        std::fmt::write(f, format_args!("{msg}"))
    }
}