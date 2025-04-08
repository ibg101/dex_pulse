use super::shared;
use crate::types::{
    error::Error,
    custom::{AccountKeys, Unpack, Parser}, 
};


/// ### Mini Custom parser for spl-token-2022 instruction.
///
/// Note: Some parts of the code are commented out intentionally.
/// They are implemented and working, but currently disabled to reduce runtime overhead.
/// These parts may be re-enabled in the future if needed.
/// 
/// https://docs.rs/spl-token-2022/7.0.0/src/spl_token_2022/instruction.rs.html#724-864
#[derive(Debug)]
pub enum TokenInstruction {
    InitializeAccount,
    // InitializeAccount3 {
    //     owner: String
    // },
    MintTo {
        amount: u64
    },
    Burn {
        amount: u64
    },
    Transfer {
        amount: u64
    },
    TransferChecked {
        amount: u64,
        decimals: u8
    }
}

#[derive(Debug)]
pub enum ParsedInstruction {
    InitializeAccount {
        account: String,
        mint: String,
        owner: String,
        rent_sysvar: String
    },
    // InitializeAccount3 {
    //     account: String,
    //     mint: String,
    //     owner: String, 
    // },
    MintTo {
        mint_signers: Vec<String>,
        mint: String,
        account: String,
        amount: u64
    },
    Burn {
        signers: Vec<String>,
        mint: String,
        account: String,
        amount: u64
    },
    Transfer {
        signers: Vec<String>,
        source: String,
        destination: String,
        amount: u64
    },
    TransferChecked {
        signers: Vec<String>,
        source: String,
        mint: String,
        destination: String,
        amount: u64,
        decimals: u8,
    }
}

impl ParsedInstruction {
    pub fn parse_signers(mut self, last_nonsigner_index: usize, account_keys: &AccountKeys, instruction_accounts: &[usize]) -> Result<Self, Error> {
        let signers: &mut Vec<String> = match &mut self {
            Self::MintTo { mint_signers, .. } => mint_signers,
            Self::Burn { signers, .. } 
            | Self::Transfer { signers, .. } 
            | Self::TransferChecked { signers, .. } => signers,
            _ => return Err(Error::ParseInstruction)
        };

        if instruction_accounts.len() > last_nonsigner_index + 1 {
            for &i in instruction_accounts[last_nonsigner_index + 1..].into_iter() {
                signers.push(account_keys[i].clone());
            } 
        } else {
            signers.push(account_keys[instruction_accounts[last_nonsigner_index]].clone());
        }

        Ok(self)
    }
}

impl Unpack for TokenInstruction {
    /// **The First Byte** of the instruction data is always - **Instruction Type**.
    fn unpack(data: &[u8]) -> Result<Self, Error> {        
        let (&instruction_type, rest) = data.split_first().ok_or(Error::InvalidInstruction)?;  

        Ok(match instruction_type {
            1 => Self::InitializeAccount,
            3 | 7 | 8 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                match instruction_type {                    
                    3 => Self::Transfer { amount },
                    7 => Self::MintTo { amount },
                    8 => Self::Burn { amount },
                    _ => unreachable!()
                }
            },
            12 => {
                let (amount, decimals, _) = Self::unpack_amount_and_decimals(rest)?;
                Self::TransferChecked { amount, decimals }
            },
            // 18 => {
            //     let (owner, _) =  Self::unpack_key(rest)?;
            //     Self::InitializeAccount3 { owner }
            // },
            _ => return Err(Error::InvalidInstruction)
        })
    }
}

impl Parser<ParsedInstruction> for TokenInstruction {
    // https://github.com/solana-labs/solana/blob/master/transaction-status/src/parse_token.rs#L19
    fn parse(self, account_keys: &AccountKeys, instruction_accounts: &[usize]) -> Result<ParsedInstruction, Error> {
        let parsed_instruction = 
            match self {
                Self::InitializeAccount => {
                    shared::validate_instruction_accounts_len(instruction_accounts, 4)?;
                    ParsedInstruction::InitializeAccount { 
                        account: account_keys[instruction_accounts[0]].clone(), 
                        mint: account_keys[instruction_accounts[1]].clone(), 
                        owner: account_keys[instruction_accounts[2]].clone(), 
                        rent_sysvar: account_keys[instruction_accounts[3]].clone() 
                    }
                },
                // Self::InitializeAccount3 { owner } => {
                //     shared::validate_instruction_accounts_len(instruction_accounts, 2)?;
                //     ParsedInstruction::InitializeAccount3 { 
                //         account: account_keys[instruction_accounts[0]].clone(), 
                //         mint: account_keys[instruction_accounts[1]].clone(), 
                //         owner 
                //     }
                // },
                Self::MintTo { amount } => {
                    shared::validate_instruction_accounts_len(instruction_accounts, 3)?;
                
                    let unfinished_instruction: ParsedInstruction = ParsedInstruction::MintTo { 
                        mint_signers: vec![], 
                        mint: account_keys[instruction_accounts[0]].clone(), 
                        account: account_keys[instruction_accounts[1]].clone(), 
                        amount
                    };

                    unfinished_instruction.parse_signers(2, account_keys, instruction_accounts)?
                },
                Self::Burn { amount } => {
                    shared::validate_instruction_accounts_len(instruction_accounts, 3)?;

                    let unfinished_instruction: ParsedInstruction = ParsedInstruction::Burn { 
                        signers: vec![], 
                        account: account_keys[instruction_accounts[0]].clone(), 
                        mint: account_keys[instruction_accounts[1]].clone(), 
                        amount
                    };

                    unfinished_instruction.parse_signers(2, account_keys, instruction_accounts)?
                },
                Self::Transfer { amount } => {
                    shared::validate_instruction_accounts_len(instruction_accounts, 3)?;

                    let unfinished_instruction: ParsedInstruction = ParsedInstruction::Transfer { 
                        signers: vec![], 
                        source: account_keys[instruction_accounts[0]].clone(), 
                        destination: account_keys[instruction_accounts[1]].clone(), 
                        amount 
                    };

                    unfinished_instruction.parse_signers(2, account_keys, instruction_accounts)?
                },
                Self::TransferChecked { amount, decimals } => {
                    shared::validate_instruction_accounts_len(instruction_accounts, 4)?;

                    let unfinished_instruction: ParsedInstruction = ParsedInstruction::TransferChecked {
                        signers: vec![],  // must be filled
                        source: account_keys[instruction_accounts[0]].clone(),
                        mint: account_keys[instruction_accounts[1]].clone(),
                        destination: account_keys[instruction_accounts[2]].clone(),
                        amount, 
                        decimals
                    };
                    
                    unfinished_instruction.parse_signers(3, account_keys, instruction_accounts)?
                }
            };
        
        Ok(parsed_instruction)
    }
}

impl TokenInstruction {
    // fn unpack_key(data: &[u8]) -> Result<(String, &[u8]), Error> {
    //     let key: String = data
    //         .get(..32)
    //         .map(|slice| bs58::encode(slice).into_string())
    //         .ok_or(Error::InvalidInstruction)?;

    //     Ok((key, &data[32..]))
    // }

    fn unpack_u64(data: &[u8]) -> Result<(u64, &[u8]), Error> {
        let amount: u64 = data
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(Error::InvalidInstruction)?;
        
        Ok((amount, &data[8..]))
    }

    fn unpack_amount_and_decimals(data: &[u8]) -> Result<(u64, u8, &[u8]), Error> {
        let (amount, rest) = Self::unpack_u64(data)?;
        let (&decimals, rest) = rest.split_first().ok_or(Error::InvalidInstruction)?;
        Ok((amount, decimals, rest))
    }
}