use super::core::get_mut_shared_token_meta;
use crate::{
    utils::parser::token_instruction::{
        TokenInstruction, 
        ParsedInstruction
    },
    types::{
        error::Error,
        rpc::{
            GetTransaction, 
            TransactionResult, 
            LoadedAddresses,
            InnerInstruction
        },
        custom::{
            Dex, 
            TokenMeta, 
            SharedTokenMeta,
            TokenMetaRaydium,
            AccountKeys,
        },
    }
};


impl Dex {
    pub async fn raydium_process_transaction(&self, tx: GetTransaction) -> Result<TokenMeta, Box<dyn std::error::Error + Send + Sync>> {
        let mut token_meta: TokenMeta = TokenMeta::default_preallocated();
        let tx_result: TransactionResult = tx.result;
        let tx_account_keys: &[String] = &tx_result.transaction.message.account_keys[..];
        let loaded_addresses: Option<&LoadedAddresses> = tx_result.meta.loaded_addresses.as_ref();
        let account_keys: AccountKeys = AccountKeys::new(tx_account_keys, loaded_addresses);

        let inner_instructions: &[InnerInstruction] = tx_result.meta.inner_instructions
            .as_ref()
            .ok_or(Error::ProcessTransaction)?;    

        let initialize_lp_instruction: &InnerInstruction = inner_instructions
            .last()
            .ok_or(Error::ProcessTransaction)?;

        for instruction in initialize_lp_instruction.instructions.iter() {
            let bytes: Vec<u8> = bs58::decode(&instruction.data).into_vec()?;

            if let Ok(token_instruction) = TokenInstruction::unpack(&bytes) {
                let parsed_instruction: ParsedInstruction = match token_instruction.parse(&account_keys, &instruction.accounts) {
                    Ok(v) => v,
                    Err(_) => continue
                };

                match parsed_instruction {
                    ParsedInstruction::InitializeAccount { account, mint, .. } => {
                        let meta: &mut SharedTokenMeta = get_mut_shared_token_meta(token_meta.base.mint.len() == 0, &mut token_meta);
                        meta.mint = mint;
                        meta.vault = account;
                    },
                    ParsedInstruction::Transfer { signers, destination, amount, .. } => {
                        let meta: &mut SharedTokenMeta = get_mut_shared_token_meta(token_meta.base.vault == destination, &mut token_meta);
                        meta.vault = destination;
                        meta.added_liq_amount = amount;
                        token_meta.signers = signers;
                    },
                    ParsedInstruction::MintTo { mint, amount, .. } => {
                        let raydium: &mut TokenMetaRaydium = token_meta
                            .raydium_related
                            .get_or_insert(TokenMetaRaydium::default_preallocated());
                        raydium.lp_mint = mint;
                        raydium.added_liq_amount = amount;
                    },
                    _ => continue
                }
            }
        }

        // enough fields? (NOTE, this is just a basic check)
        if token_meta.base.mint.len() == 0 || token_meta.quote.mint.len() == 0 { return Err(Error::ProcessTransaction.into()); }

        Ok(token_meta)
    }
}