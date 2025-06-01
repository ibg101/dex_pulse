pub const NATIVE_MINT: &'static str = "So11111111111111111111111111111111111111112";
pub const USDC_MINT: &'static str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const NATIVE_DECIMALS: u8 = 9;
pub const USDC_DECIMALS: u8 = 6;

pub const RAYDIUM_LP_V4_PROGRAM_ID: &'static str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
pub const RAYDIUM_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP: &'static str = "Program 675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8 success";
pub const RAYDIUM_INSTRUCTION_CREATE_NEW_LP: &'static str = "initialize2";

pub const METEORA_DLMM_PROGRAM_ID: &'static str = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo";
pub const METEORA_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP: &'static str = "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo success"; 
pub const METEORA_INSTRUCTION_INITIALIZE_POSITION: &'static str = "InitializePosition";
pub const METEORA_INSTRUCTION_INITIALIZE_BIN_ARRAY: &'static str = "InitializeBinArray";

pub const PUMPSWAP_AMM_PROGRAM_ID: &'static str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
// Although `initializeMint2` can also be used, `CreatePool` appears earlier in the logs array,
// allowing to terminate the loop sooner and improve performance
pub const PUMPSWAP_INSTRUCTION_CREATE_NEW_LP: &'static str = "CreatePool";
pub const PUMPSWAP_CREATE_POOL_DISCRIMINATOR: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
pub const PUMPSWAP_ANCHOR_CPI_LOG_DISCRIMINATOR: [u8; 16] = [228, 69, 165, 46, 81, 203, 154, 29, 177, 49, 12, 210, 160, 118, 167, 116];