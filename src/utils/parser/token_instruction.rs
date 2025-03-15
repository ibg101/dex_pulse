use crate::types::error::Error;


/// ### Mini Custom parser for spl-token-2022 instruction.
/// 
/// https://docs.rs/spl-token-2022/7.0.0/src/spl_token_2022/instruction.rs.html#724-864
#[derive(Debug)]
pub enum TokenInstruction {
    TransferChecked {
        amount: u64,
        decimals: u8
    }
}

#[derive(Debug)]
pub enum ParsedInstruction {
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
    fn parse_signers(mut self, last_nonsigner_index: usize, account_keys: &[String], instruction_accounts: &[usize]) -> Self {
        match &mut self {
            Self::TransferChecked { signers, .. } => {
                if instruction_accounts.len() > last_nonsigner_index + 1 {
                    for &i in instruction_accounts[last_nonsigner_index + 1..].into_iter() {
                        signers.push(account_keys[i as usize].clone());
                    } 
                } else {
                    signers.push(account_keys[instruction_accounts[last_nonsigner_index] as usize].clone());
                }
                self
            },

        }
    }
}

impl TokenInstruction {
    /// https://github.com/solana-labs/solana/blob/master/transaction-status/src/parse_token.rs#L19
    pub fn parse(&self, account_keys: &[String], instruction_accounts: &[usize]) -> Result<ParsedInstruction, Error> {
        let parsed_instruction = 
            match self {
                &Self::TransferChecked { amount, decimals } => {
                    if instruction_accounts.len() != 4 { return Err(Error::ParseInstruction); }
                    
                    let unfinished_instruction: ParsedInstruction = ParsedInstruction::TransferChecked {
                        signers: vec![],  // must be filled
                        source: account_keys[instruction_accounts[0] as usize].clone(),
                        mint: account_keys[instruction_accounts[1] as usize].clone(),
                        destination: account_keys[instruction_accounts[2] as usize].clone(),
                        amount, 
                        decimals
                    };
                    
                    unfinished_instruction.parse_signers(3, account_keys, instruction_accounts)
                },
                _ => return Err(Error::ParseInstruction) 
            };
        
        Ok(parsed_instruction)
    }

    /// **The First Byte** of the instruction data is always - **Instruction Type**.
    pub fn unpack(data: &[u8]) -> Result<Self, Error> {        
        let (&instruction_type, rest) = data.split_first().ok_or(Error::InvalidInstruction)?;  

        Ok(match instruction_type {
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
}