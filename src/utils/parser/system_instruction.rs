use super::shared::{
    unpack_option_key,
    try_array_from_slice,
    validate_instruction_accounts_len
};
use crate::types::{
    error::Error,
    custom::{AccountKeys, Unpack, Parser},
};


#[derive(Debug)]
pub enum SystemInstruction {
    Assign {
        owner: Option<String>
    }
}

#[derive(Debug)]
pub enum ParsedInstruction {
    Assign {
        account: String,
        owner: String
    }
}

const ASSIGN_LEN: usize = 36;

impl Unpack for SystemInstruction {    
    fn unpack(data: &[u8]) -> Result<Self, Error> {
        Ok(match data.len() {
            ASSIGN_LEN => {
                let owner: [u8; 36] = try_array_from_slice(data, 0, 36)?; 
                Self::Assign { owner: unpack_option_key(owner)? }
            },
            _ => return Err(Error::ParseSystemInstruction)
        })
    }
}

impl Parser<ParsedInstruction> for SystemInstruction {
    fn parse(self, account_keys: &AccountKeys, instruction_accounts: &[usize]) -> Result<ParsedInstruction, Error> {
        validate_instruction_accounts_len(instruction_accounts, 1)?;
        Ok(match self {
            Self::Assign { owner } => ParsedInstruction::Assign { 
                account: account_keys[instruction_accounts[0]].clone(), 
                owner: owner.ok_or(Error::ParseSystemInstruction)? 
            }
        })        
    }
}