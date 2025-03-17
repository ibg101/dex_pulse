use crate::types::{
    custom::AccountKeys, 
    error::Error
};


/// ### Mini Custom parser for spl-token-2022 instruction.
/// 
/// https://docs.rs/spl-token-2022/7.0.0/src/spl_token_2022/instruction.rs.html#724-864
#[derive(Debug)]
pub enum TokenInstruction {
    InitializeAccount,
    MintTo {
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
    MintTo {
        mint_signers: Vec<String>,
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
    fn parse_signers(mut self, last_nonsigner_index: usize, account_keys: &AccountKeys, instruction_accounts: &[usize]) -> Result<Self, Error> {
        let signers: &mut Vec<String> = match &mut self {
            Self::MintTo { mint_signers, .. } => mint_signers,
            Self::Transfer { signers, .. } => signers,
            Self::TransferChecked { signers, .. } => signers,
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

impl TokenInstruction {
    /// https://github.com/solana-labs/solana/blob/master/transaction-status/src/parse_token.rs#L19
    pub fn parse(&self, account_keys: &AccountKeys, instruction_accounts: &[usize]) -> Result<ParsedInstruction, Error> {
        let parsed_instruction = 
            match self {
                &Self::InitializeAccount => {
                    Self::validate_instruction_accounts_len(instruction_accounts, 4)?;
                    ParsedInstruction::InitializeAccount { 
                        account: account_keys[instruction_accounts[0]].clone(), 
                        mint: account_keys[instruction_accounts[1]].clone(), 
                        owner: account_keys[instruction_accounts[2]].clone(), 
                        rent_sysvar: account_keys[instruction_accounts[3]].clone() 
                    }
                },
                &Self::MintTo { amount } => {
                    Self::validate_instruction_accounts_len(instruction_accounts, 3)?;
                
                    let unfinished_instruction: ParsedInstruction = ParsedInstruction::MintTo { 
                        mint_signers: vec![], 
                        mint: account_keys[instruction_accounts[0]].clone(), 
                        account: account_keys[instruction_accounts[1]].clone(), 
                        amount
                    };

                    unfinished_instruction.parse_signers(2, account_keys, instruction_accounts)?
                },
                &Self::Transfer { amount } => {
                    Self::validate_instruction_accounts_len(instruction_accounts, 3)?;

                    let unfinished_instruction: ParsedInstruction = ParsedInstruction::Transfer { 
                        signers: vec![], 
                        source: account_keys[instruction_accounts[0]].clone(), 
                        destination: account_keys[instruction_accounts[1]].clone(), 
                        amount 
                    };

                    unfinished_instruction.parse_signers(2, account_keys, instruction_accounts)?
                },
                &Self::TransferChecked { amount, decimals } => {
                    Self::validate_instruction_accounts_len(instruction_accounts, 4)?;

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

    /// **The First Byte** of the instruction data is always - **Instruction Type**.
    pub fn unpack(data: &[u8]) -> Result<Self, Error> {        
        let (&instruction_type, rest) = data.split_first().ok_or(Error::InvalidInstruction)?;  

        Ok(match instruction_type {
            1 => Self::InitializeAccount,
            3 | 7 => {
                let (amount, _) = Self::unpack_u64(rest)?;
                match instruction_type {                    
                    3 => Self::Transfer { amount },
                    7 => Self::MintTo { amount },
                    _ => unreachable!()
                }
            },
            12 => {
                let (amount, decimals, _) = Self::unpack_amount_and_decimals(rest)?;
                Self::TransferChecked { amount, decimals }
            },
            _ => return Err(Error::InvalidInstruction)
        })
    }

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

    fn validate_instruction_accounts_len(instruction_accounts: &[usize], expected_len: usize) -> Result<(), Error> {
        if instruction_accounts.len() != expected_len { return Err(Error::ParseInstruction); }
        Ok(())
    }
}