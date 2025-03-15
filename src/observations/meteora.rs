use crate::{
    types::{
        custom::Dex,
        rpc::{
            LogsSubscribe, 
            LogsSubscribeValue
        }
    },
    constants::{
        METEORA_INSTRUCTION_ADD_LIQUIDITY,
        METEORA_INSTRUCTION_INITIALIZE_POSITION, 
        METEORA_INSTRUCTION_SUCCESSFUL_ADD_LIQUIDITY
    }
};

use tokio::sync::mpsc;


impl Dex {
    pub async fn meteora_add_liquidity_event(
        &self,
        logs_subscribe: LogsSubscribe<'_>, 
        tx: &mpsc::Sender<(String, Dex)>
    ) -> () {
        let logs_value: LogsSubscribeValue = logs_subscribe.params.result.value;
        let logs: Vec<&str> = logs_value.logs;
        let mut is_add_liq_instruction: bool = false;
        let mut is_init_position_instruction: bool = false;
        let mut is_successfully_added: bool = false; 
    
        for log in logs.into_iter() {
            if is_add_liq_instruction && is_successfully_added { break; }
            
            if log.contains(METEORA_INSTRUCTION_INITIALIZE_POSITION) {
                is_init_position_instruction = true;
            }

            if log.contains(METEORA_INSTRUCTION_ADD_LIQUIDITY) {
                is_add_liq_instruction = true;
            }
    
            if log.contains(METEORA_INSTRUCTION_SUCCESSFUL_ADD_LIQUIDITY) {
                is_successfully_added = true;
            }
        }
    
        if is_init_position_instruction && is_add_liq_instruction && is_successfully_added {
            let signature: String = logs_value.signature.to_owned();
            if let Err(e) = tx.send((signature, *self)).await {
                log::error!("Failed to extend signatures channel! {e}");
            }
        }
    }
}