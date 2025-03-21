use super::{
    rpc::LoadedAddresses,
    error::Error
};
use crate::utils::parser::account::AccountType;


#[derive(Debug, Clone, Copy)]
pub enum Dex {
    Raydium,
    Meteora
}

// ---- my custom pair meta ----
#[derive(Default, Debug)]
pub struct PairMeta {
    pub base: SharedTokenMeta,
    pub quote: SharedTokenMeta,
    pub signers: Vec<String>,
    pub raydium_related: Option<PairMetaRaydium>,
    pub market_id: String
}

#[derive(Default, Debug)]
pub struct SharedTokenMeta {
    pub mint_account: Option<AccountType>,
    pub mint: String,
    pub vault: String,  // Pool 1 / Pool 2
    pub provided_liq_amount: u64,  // raw,
    pub provided_liq_ratio: Option<f64>
}

#[derive(Default, Debug)]
pub struct PairMetaRaydium {
    pub lp_mint: String,
    pub lp_tokens_minted_amount: u64  // raw
}

const PUBKEY_LEN: usize = 32;

impl PairMeta {
    pub fn default_preallocated() -> Self {
        Self { 
            base: Self::default_preallocated_shared_meta(), 
            quote: Self::default_preallocated_shared_meta(), 
            ..Default::default() 
        }        
    }

    fn default_preallocated_shared_meta() -> SharedTokenMeta {
        SharedTokenMeta {
            mint: String::with_capacity(PUBKEY_LEN),
            vault: String::with_capacity(PUBKEY_LEN),
            ..Default::default()
        }
    }
}

impl PairMetaRaydium {
    pub fn default_preallocated() -> Self {
        Self {
            lp_mint: String::with_capacity(PUBKEY_LEN), 
            ..Default::default() 
        }
    }
}
// ---- my custom token meta ----

#[derive(Debug)]
pub struct AccountKeys<'a> {
    static_keys: &'a [String],
    dynamic_keys: Option<&'a LoadedAddresses>
}

// syntactic sugar (allows using account_keys[i] instead of account_keys.get(i).unwrap())
impl std::ops::Index<usize> for AccountKeys<'_> {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.get(index).expect("invalid index!")
    }
}

impl<'a> AccountKeys<'a> {
    pub fn new(static_keys: &'a [String], dynamic_keys: Option<&'a LoadedAddresses>) -> Self {
        Self { static_keys, dynamic_keys }
    }

    fn segments_iter(&self) -> impl Iterator<Item = &[String]> {
        let segments_collection: [&[String]; 3] = if let Some(dyn_keys) = self.dynamic_keys {
            [self.static_keys, &dyn_keys.writable, &dyn_keys.readonly]
        } else {
            [self.static_keys, &[], &[]]
        };
        segments_collection.into_iter()
    }
    
    pub fn get(&self, mut index: usize) -> Option<&String> {
        for segment in self.segments_iter() {
            let segment_len: usize = segment.len();
            if segment_len > index {
                return Some(&segment[index]);
            }
            index = index.saturating_sub(segment_len);  // using -= in order to work with new segment from proper index
        }
        None
    } 
}

pub trait Unpack: Sized {
    fn unpack(data: &[u8]) -> Result<Self, Error>; 
}

pub trait Parser<T> {
    fn parse(self, account_keys: &AccountKeys, instruction_accounts: &[usize]) -> Result<T, Error>;
}

// can be used for ParsedInstruction (each parser has it's own struct, however by now only token_instruction's uses parse_signers method)
// pub trait Signable: Sized {
//     fn parse_signers(self, last_nonsigner_index: usize, account_keys: &AccountKeys, instruction_accounts: &[usize]) -> Result<Self, Error>;
// }