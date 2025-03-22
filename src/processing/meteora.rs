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
            Unpack,
            Parser, 
            PairMeta,
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
    /// 2. Ensure the mint fields in `PairMeta` are populated
    pub async fn meteora_process_transaction(&self, tx: GetTransaction) -> Result<PairMeta, Box<dyn std::error::Error + Send + Sync>> {
        let mut pair_meta: PairMeta = PairMeta::default_preallocated();
        let tx_result: TransactionResult = tx.result;
        let tx_account_keys: &[String] = &tx_result.transaction.message.account_keys[..];
        let loaded_addresses: Option<&LoadedAddresses> = tx_result.meta.loaded_addresses.as_ref();
        let account_keys: AccountKeys = AccountKeys::new(tx_account_keys, loaded_addresses); 

        let inner_instructions: Vec<InnerInstruction> = tx_result.meta.inner_instructions.ok_or(Error::ProcessTransaction)?;
        let add_liquidity_instruction: &InnerInstruction = inner_instructions
            .last()
            .ok_or(Error::ProcessTransaction)?;
        
        for instruction in add_liquidity_instruction.instructions.iter() {
            if pair_meta.base.mint.len() > 0 && pair_meta.quote.mint.len() > 0 { break; }

            let bytes: Vec<u8> = bs58::decode(&instruction.data).into_vec()?;

            if let Ok(token_instruction) = TokenInstruction::unpack(&bytes) {
                let parsed_instruction: ParsedInstruction = match token_instruction.parse(&account_keys, &instruction.accounts) {
                    Ok(v) => v,
                    Err(_) => continue
                };
                
                if let ParsedInstruction::TransferChecked { 
                    signers, 
                    mint, 
                    destination, 
                    amount, 
                    ..
                } = parsed_instruction {
                    let meta: &mut SharedTokenMeta = get_mut_shared_token_meta(pair_meta.base.mint.len() == 0, &mut pair_meta);
                    meta.mint.push_str(&mint);  // using preallocated space
                    meta.vault.push_str(&destination);
                    meta.provided_liq_amount = amount;
                    pair_meta.signers = signers;
                }
            } else {
                // size of anchor cpi log is 132 bytes
                if bytes.len() == 132 {
                    pair_meta.market_id.push_str(&bs58::encode(&bytes[16..48]).into_string());
                }
            }
        }

        // enough fields? (NOTE, this is just a basic check)
        if pair_meta.base.mint.len() == 0 
        || pair_meta.quote.mint.len() == 0 
        || pair_meta.market_id.len() == 0 { return Err(Error::ProcessTransaction.into()); }

        Ok(pair_meta)
    } 
}