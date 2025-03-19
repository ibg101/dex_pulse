use crate::types::error::Error;


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

impl AccountType {
    pub fn unpack(data: &[u8]) -> Result<Self, Error> {
        Ok(match data.len() {
            MINT_LEN => {
                let mint_authority: [u8; 36] = Self::try_array_from_slice(&data, 0, 36)?;
                let supply: [u8; 8] = Self::try_array_from_slice(&data, 36, 44)?;
                let decimals: [u8; 1] = Self::try_array_from_slice(&data, 44, 45)?;
                let is_initialized: [u8; 1] = Self::try_array_from_slice(&data, 45, 46)?;
                let freeze_authority: [u8; 36] = Self::try_array_from_slice(&data, 46, 82)?;
                
                let is_initialized: bool = match is_initialized {
                    [0] => false,
                    [1] => true,
                    _ => {
                        log::error!("Invalid is_initialized Byte");
                        return Err(Error::ParseAccount);
                    }
                };

                Self::Mint { 
                    mint_authority: Self::unpack_option_key(mint_authority)?, 
                    supply: u64::from_le_bytes(supply), 
                    decimals: decimals[0], 
                    is_initialized, 
                    freeze_authority: Self::unpack_option_key(freeze_authority)? 
                }
            },
            ACCOUNT_LEN => {
                // Self::Account {...}
                unimplemented!()
            },
            _ => unreachable!()
        })
    }

    fn unpack_option_key(key: [u8; 36]) -> Result<Option<String>, Error> {
        let (tag, pubkey) = key.split_at(4);
        match *tag {
            [0, 0, 0, 0] => Ok(None),
            [1, 0, 0, 0] => Ok(Some(bs58::encode(pubkey).into_string())),
            _ => {
                log::error!("Invalid Pubkey Tag!");
                Err(Error::ParseAccount)
            }
        }
    }

    fn try_array_from_slice<const L: usize>(d: &[u8], start: usize, end: usize) -> Result<[u8; L], Error> {
        d[start..end].try_into().map_err(|e| {
            log::error!("Caused an error: {e}");
            Error::ParseAccount
        })
    }
}