// ! NOTE - PARTIALLY DEPRECATED !
// some pieces of code here are deprecated,
// but i decided to keep them, even though they are not used,
// however might be helpful in the future

// use super::shared::{
//     unpack_u16,
//     unpack_u64
// };
use crate::{
    types::{
        error::Error,
        // custom::Unpack
    },
    constants::{
        // PUMPSWAP_CREATE_POOL_DISCRIMINATOR,
        PUMPSWAP_ANCHOR_CPI_LOG_DISCRIMINATOR
    }
};


// This schema is used as a reference for parsing only `coin_creator` OR `creator` from bytes slice,
//   because i dont need other fields therefore deserialization of whole struct is unnecessary.

// + 16 bytes DISCRIMINATOR
// #[derive(Debug, BorshDeserialize, BorshSerialize)]
// struct ReferenceAnchorCPILog {
//     timestamp: u64,
//     index: u16,
//     creator: [u8; 32],
//     base_mint: [u8; 32],
//     quote_mint: [u8; 32],
//     base_mint_decimals: u8,
//     quote_mint_decimals: u8,
//     base_amount_in: u64,
//     quote_amount_in: u64,
//     pool_base_amount: u64,
//     pool_quote_amount: u64,
//     minumum_liquidity: u64,
//     initial_liquidity: u64,
//     lp_token_amount_out: u64,
//     pool_bump: u8,
//     pool: [u8; 32],
//     lp_mint: [u8; 32],
//     user_base_token_account: [u8; 32],
//     user_quote_token_account: [u8; 32],
//     coin_creator: [u8; 32]  // this field doesn't exist sometimes, so it could break BorshDeserialize
// }

pub struct AMMAnchorCPILog;

impl AMMAnchorCPILog {
    // check `ReferenceAnchorCPILog` struct above in order to understand how it was calculated
    const WITHOUT_COIN_CREATOR_LEN: usize = 8 + 2 + 32 + 32 + 32 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1 + 32 + 32 + 32 + 32;
    const EMPTY_ADDRESS: [u8; 32] = [0; 32];

    /// This method will try to extract `coin_creator` OR `creator` from `Pump.fun AMM: anchor Self CPI Log`
    pub fn try_parse_creator(bytes: &[u8]) -> Result<String, Error> {
        let (discriminator, rest_bytes) = bytes.split_at(16);
        
        if discriminator != PUMPSWAP_ANCHOR_CPI_LOG_DISCRIMINATOR {
            return Err(Error::InvalidInstruction);
        }

        let (before_coin_creator, after) = rest_bytes.split_at(Self::WITHOUT_COIN_CREATOR_LEN);
        
        Ok(bs58::encode(if after.len() >= 32 && !after.starts_with(&Self::EMPTY_ADDRESS) {
            &after[..32]                   // take coin_creator 
        } else {
            &before_coin_creator[10..42]   // take creator
        }).into_string())
    }
}


// #[derive(Debug)]
// pub struct CreatePoolInstruction {
//     pub index: u16,
//     pub base_amount_in: u64,
//     pub quote_amount_in: u64,
//     pub coin_creator_bytes: [u8; 32],
//     _is_known_cc: bool
// }

// const DISCRIMINATOR_LEN: usize = 8;
// const UNKNOWN_COIN_CREATOR_LEN: usize = DISCRIMINATOR_LEN + 2 + 8 + 8;
// const KNOWN_COIN_CREATOR_LEN: usize = UNKNOWN_COIN_CREATOR_LEN + 32;

// impl Unpack for CreatePoolInstruction {
//     fn unpack(data: &[u8]) -> Result<Self, Error> {
//         let (discriminator, rest) = data.split_at(8);

//         if discriminator != PUMPSWAP_CREATE_POOL_DISCRIMINATOR {
//             return Err(Error::InvalidInstruction);
//         }

//         Ok(match data.len() {
//             UNKNOWN_COIN_CREATOR_LEN => {
//                 let (index, base_amount_in, quote_amount_in) = Self::unpack_shared(rest)?;
//                 let coin_creator_bytes: [u8; 32] = std::array::from_fn(|_| 1u8);
//                 Self { index, base_amount_in, quote_amount_in, coin_creator_bytes, _is_known_cc: false }
//             },
//             KNOWN_COIN_CREATOR_LEN => {
//                 let (index, base_amount_in, quote_amount_in) = Self::unpack_shared(rest)?;
//                 let coin_creator_bytes: [u8; 32] = data[KNOWN_COIN_CREATOR_LEN - 32..].try_into().unwrap();
//                 Self { index, base_amount_in, quote_amount_in, coin_creator_bytes, _is_known_cc: true }
//             },
//             _ => unreachable!()
//         })
//     }
// }

// impl CreatePoolInstruction {
//     pub fn is_known_coin_creator(&self) -> bool {
//         self._is_known_cc
//     }

//     fn unpack_shared(data: &[u8]) -> Result<(u16, u64, u64), Error> {
//         let (index, rest) = unpack_u16(data)?;
//         let (base_amount_in, rest) = unpack_u64(rest)?;
//         let (quote_amount_in, _) = unpack_u64(rest)?;
//         Ok((index, base_amount_in, quote_amount_in))
//     }
// }