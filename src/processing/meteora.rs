use super::core::get_mut_shared_token_meta;
use crate::{
    utils::parser::token_instruction::{
        TokenInstruction,
        ParsedInstruction, 
    },
    types::{
        error::Error, 
        custom::{
            Dex, 
            TokenMeta,
            AccountKeys,
            SharedTokenMeta,
        }, 
        rpc::{
            GetTransaction, 
            InnerInstruction, 
            TransactionResult,
            LoadedAddresses
        }
    } 
};


impl Dex {
    /// 1. Attempt to decode `TransferChecked` instructions (for BASE & QUOTE mints)
    /// 2. Ensure the mint fields in `TokenMeta` are populated
    pub async fn meteora_process_transaction(&self, tx: GetTransaction) -> Result<TokenMeta, Box<dyn std::error::Error + Send + Sync>> {
        let mut token_meta: TokenMeta = TokenMeta::default_preallocated();
        let tx_result: TransactionResult = tx.result;
        let tx_account_keys: &[String] = &tx_result.transaction.message.account_keys[..];
        let loaded_addresses: Option<&LoadedAddresses> = tx_result.meta.loaded_addresses.as_ref();
        let account_keys: AccountKeys = AccountKeys::new(tx_account_keys, loaded_addresses); 
        
        let inner_instructions: &[InnerInstruction] = tx_result.meta.inner_instructions
            .as_ref()
            .ok_or(Error::ProcessTransaction)?;

        let add_liquidity_instruction: &InnerInstruction = inner_instructions
            .last()
            .ok_or(Error::ProcessTransaction)?;
        
        for instruction in add_liquidity_instruction.instructions.iter() {
            if token_meta.base.mint.len() > 0 && token_meta.quote.mint.len() > 0 { break; }

            let bytes: Vec<u8> = bs58::decode(&instruction.data).into_vec()?;

            if let Ok(token_instruction) = TokenInstruction::unpack(&bytes) {
                let parsed_instruction: ParsedInstruction = token_instruction
                    .parse(&account_keys, &instruction.accounts)?;
                
                #[allow(unused_variables)]
                if let ParsedInstruction::TransferChecked { 
                    signers, 
                    source,  // senders_ata
                    mint, 
                    destination, 
                    amount, 
                    decimals 
                } = parsed_instruction {
                    let meta: &mut SharedTokenMeta = get_mut_shared_token_meta(token_meta.base.mint.len() == 0, &mut token_meta);
                    meta.mint = mint;
                    meta.vault = destination;
                    meta.added_liq_amount = amount;
                    meta.decimals = decimals;
                    token_meta.signers = signers;
                }
            }
        }

        // enough fields? (NOTE, this is just a basic check)
        if token_meta.base.mint.len() == 0 || token_meta.quote.mint.len() == 0 { return Err(Error::ProcessTransaction.into()); }

        Ok(token_meta)
    } 
}