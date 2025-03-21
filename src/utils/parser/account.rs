use super::shared::{
    unpack_option_key,
    try_array_from_slice
};
use crate::types::{
    error::Error,
    custom::Unpack
};


// EXAMPLE: use this for get_account_info()
/// Custom Lightweight Parser for Solana Account Data
///
/// This custom parser is designed to decode Solana account data in a fast and efficient manner, 
/// specifically tailored to the needs of this project. By avoiding the use of JSON encoded formats 
/// and utilizing Base58 decoding, this parser provides significant performance gains in scenarios 
/// where speed is critical. The implementation is optimized to handle only the account types and 
/// structures necessary for this project, eliminating the overhead introduced by more general parsers.
/// This approach results in faster data parsing, reduced memory usage, and improved overall system efficiency.
#[derive(Debug)]
pub enum AccountType {
    Mint {
        mint_authority: Option<String>,
        supply: u64,
        decimals: u8,
        is_initialized: bool,
        freeze_authority: Option<String>
    },
    // .. other options may be added
}

const MINT_LEN: usize = 82;
const ACCOUNT_LEN: usize = 165;

impl Unpack for AccountType {
    fn unpack(data: &[u8]) -> Result<Self, Error> {
        Ok(match data.len() {
            MINT_LEN => {
                let mint_authority: [u8; 36] = try_array_from_slice(&data, 0, 36)?;
                let supply: [u8; 8] = try_array_from_slice(&data, 36, 44)?;
                let decimals: [u8; 1] = try_array_from_slice(&data, 44, 45)?;
                let is_initialized: [u8; 1] = try_array_from_slice(&data, 45, 46)?;
                let freeze_authority: [u8; 36] = try_array_from_slice(&data, 46, 82)?;
                
                let is_initialized: bool = match is_initialized {
                    [0] => false,
                    [1] => true,
                    _ => {
                        log::error!("Invalid is_initialized Byte");
                        return Err(Error::ParseAccount);
                    }
                };

                Self::Mint { 
                    mint_authority: unpack_option_key(mint_authority)?, 
                    supply: u64::from_le_bytes(supply), 
                    decimals: decimals[0], 
                    is_initialized, 
                    freeze_authority: unpack_option_key(freeze_authority)? 
                }
            },
            ACCOUNT_LEN => {
                // Self::Account {...}
                unimplemented!()
            },
            _ => unreachable!()
        })
    }
}