#[derive(Debug)]
pub enum Error {
    CreateHttpClient,
    UnsupportedCommitment,
    ReachedMaxRetries,
    InvalidInstruction,
    ParseInstruction,
    ProcessTransaction,
}

impl std::error::Error for Error {}
unsafe impl std::marker::Send for Error {}
unsafe impl std::marker::Sync for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: &str = match self {
            Self::CreateHttpClient => "Failed to initialize HttpClient!",
            Self::UnsupportedCommitment => "Attempted to use an unsupported CommitmentLevel.\n\
            Please verify that the RPC method you are using supports this commitment level.",
            Self::ReachedMaxRetries => "Reached max retries while calling RPC method!",
            Self::InvalidInstruction => "Failed to unpack the instruction due to invalid data!",
            Self::ParseInstruction => "Failed to parse an instruction!",
            Self::ProcessTransaction => "Failed to process transaction!",
        };

        f.write_str(msg)
    }
}