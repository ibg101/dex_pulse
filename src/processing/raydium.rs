use super::core::{
    get_mut_shared_token_meta,
    check_necessary_fields_filled
};
use crate::{
    utils::parser::{
        system_instruction::{
            self,
            SystemInstruction
        },
        token_instruction::{
            self,
            TokenInstruction
        }
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
            Unpack,
            Parser, 
            PairMeta,
            SharedTokenMeta,
            PairMetaRaydium,
            AccountKeys,
        },
    },
    constants::RAYDIUM_LP_V4_PROGRAM_ID
};


impl Dex {
    pub async fn raydium_process_transaction(&self, tx: GetTransaction) -> Result<PairMeta, Box<dyn std::error::Error + Send + Sync>> {
        let mut pair_meta: PairMeta = PairMeta::default_preallocated();
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

            if let Ok(system_instruction) = SystemInstruction::unpack(&bytes) {  
                let parsed_instruction: system_instruction::ParsedInstruction = match system_instruction.parse(&account_keys, &instruction.accounts) {
                    Ok(v) => v,
                    Err(_) => continue
                };
                // since there is only 1 option available -> it's sufficient
                let system_instruction::ParsedInstruction::Assign { account, owner } = parsed_instruction;
                if owner != RAYDIUM_LP_V4_PROGRAM_ID {
                    continue;
                } else {
                    pair_meta.market_id.push_str(&account);
                }
            }

            if let Ok(token_instruction) = TokenInstruction::unpack(&bytes) {
                let parsed_instruction: token_instruction::ParsedInstruction = match token_instruction.parse(&account_keys, &instruction.accounts) {
                    Ok(v) => v,
                    Err(_) => continue
                };

                match parsed_instruction {
                    token_instruction::ParsedInstruction::InitializeAccount { account, mint, .. } => {
                        let meta: &mut SharedTokenMeta = get_mut_shared_token_meta(pair_meta.base.mint.len() == 0, &mut pair_meta);
                        meta.mint.push_str(&mint);
                        meta.vault.push_str(&account);
                    },
                    token_instruction::ParsedInstruction::Transfer { signers, destination, amount, .. } => {
                        let meta: &mut SharedTokenMeta = get_mut_shared_token_meta(pair_meta.base.vault == destination, &mut pair_meta);
                        meta.vault.push_str(&destination);
                        meta.provided_liq_amount = amount;
                        pair_meta.signers = signers;
                    },
                    token_instruction::ParsedInstruction::MintTo { mint, amount, .. } => {
                        let raydium: &mut PairMetaRaydium = pair_meta
                            .raydium_related
                            .get_or_insert(PairMetaRaydium::default_preallocated());
                        raydium.lp_mint.push_str(&mint);
                        raydium.lp_tokens_minted_amount = amount;
                    },
                    _ => continue
                }
            }
        }

        check_necessary_fields_filled(&pair_meta)?;
        
        Ok(pair_meta)
    }
}