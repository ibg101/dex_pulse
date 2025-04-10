use super::core::{
    get_mut_shared_token_meta,
    check_necessary_fields_filled
};
use crate::{
    utils::parser::token_instruction::{
        self,
        TokenInstruction
    },
    types::{
        error::Error,
        custom::{
            Dex, 
            PairMeta,
            Unpack,
            Parser,
            AccountKeys,
            LPTokenMeta,
            SharedTokenMeta
        },
        rpc::{
            GetTransaction,
            TransactionResult,
            LoadedAddresses,
            InnerInstruction
        }
    }
};


impl Dex {
    /// 1. Attempt to decode `TransferChecked` token instructions (for BASE & QUOTE mints, VAULT addresses, provided liquidity AMOUNT)
    /// 2. Attempt to decode `MintTo` token instruction (for LP token meta)
    /// 3. Attempt to decode `Burn` token instruction, if exists (for burnt AMOUNT)
    /// 4. Try to parse `signer` by knowing exact size of Anchor CPI log struct
    /// 5. Ensure the necessary fields in `PairMeta` are populated
    pub async fn pumpswap_process_transaction(&self, tx: GetTransaction) -> Result<PairMeta, Box<dyn std::error::Error + Send + Sync>> {
        let mut pair_meta: PairMeta = PairMeta::default_preallocated();
        let tx_result: TransactionResult = tx.result;
        let tx_account_keys: &[String] = &tx_result.transaction.message.account_keys[..];
        let loaded_addresses: Option<&LoadedAddresses> = tx_result.meta.loaded_addresses.as_ref();
        let account_keys: AccountKeys = AccountKeys::new(tx_account_keys, loaded_addresses); 

        let inner_instructions: Vec<InnerInstruction> = tx_result.meta.inner_instructions.ok_or(Error::ProcessTransaction)?;
                
        let pumpfun_migrate_inner_instruction: InnerInstruction = inner_instructions
            .into_iter()
            .max_by(|i1, i2| i1.instructions.len().cmp(&i2.instructions.len()))
            .ok_or(Error::ProcessTransaction)?;

        // early termination flags
        let mut processed_burn: bool = false;
        let mut processed_mint_to: bool = false;
        let mut processed_transfer_checked_times: u8 = 0;
        
        // using rev because all instructions i need to parse are located at the end => performance boost (only if early termination is implemented)
        for instruction in pumpfun_migrate_inner_instruction.instructions.into_iter().rev() {
            let bytes: Vec<u8> = bs58::decode(instruction.data).into_vec()?;

            if let Ok(token_instruction) = TokenInstruction::unpack(&bytes) {
                let parsed_instruction: token_instruction::ParsedInstruction = match token_instruction.parse(&account_keys, &instruction.accounts) {
                    Ok(v) => v,
                    _ => continue
                };

                match parsed_instruction {
                    token_instruction::ParsedInstruction::Burn { amount, .. } => {
                        let lp_token: &mut LPTokenMeta = pair_meta.get_mut_lp_token();
                        lp_token.tokens_burnt_amount.replace(amount);
                        processed_burn = true;
                    },
                    token_instruction::ParsedInstruction::MintTo { mint_signers, mint, amount, .. } => {
                        let lp_token: &mut LPTokenMeta = pair_meta.get_mut_lp_token();
                        lp_token.mint.push_str(&mint);
                        lp_token.tokens_minted_amount = amount;
                        pair_meta.market_id.push_str(&mint_signers[0]);  // accessing by index is safe in this case
                        processed_mint_to = true;
                    }
                    token_instruction::ParsedInstruction::TransferChecked {
                        mint, 
                        destination, 
                        amount,
                        .. 
                    } => {
                        // keep in mind that the instructions array is reversed!
                        let meta: &mut SharedTokenMeta = get_mut_shared_token_meta(pair_meta.quote.mint.len() != 0, &mut pair_meta);
                        meta.mint.push_str(&mint);
                        meta.vault.push_str(&destination);
                        meta.provided_liq_amount = amount;
                        processed_transfer_checked_times += 1;
                    },
                    _ => continue
                }
            } else if pair_meta.signers.len() == 0 {
                // size of anchor self cpi log is 176 bytes | 309 bytes (based on the tx)
                let pubkey_bytes_slice: &[u8] = match bytes.len() {
                    176 => &bytes[16..48],  // migrate + create pool; example signature: 4ejDPRGBYF43zg8Hpu8gApjZN8yurQkcXNwnMXVKrzQZQHexFXFqVd1VLFZpa6eGWz39Vxo6Nhbz99aKbhz3CfAv 
                    309 => &bytes[26..58],  // create pool; example signature: bfVLmwBzDgNrpycKXHXWg1eWb9r2DJuDMWAfGFRzbenQZBzTCmM7f9VfP5sesFyGdTv4eqb7W9f74u1tckaJ7V2
                    _ => continue
                };
                pair_meta.signers.push(bs58::encode(pubkey_bytes_slice).into_string())
            }

            if processed_burn && processed_mint_to && processed_transfer_checked_times == 2 { break; }
        }

        check_necessary_fields_filled(&pair_meta)?;

        Ok(pair_meta)
    }
}