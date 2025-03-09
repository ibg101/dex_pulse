use crate::{
    types::{
        enums::Dex,
        structs::{
            LogsSubscribe, 
            LogsSubscribeValue
        }
    },
    constants::{
        RAYDIUM_INSTRUCTION_CREATE_NEW_LP, 
        RAYDIUM_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP
    }
};

use tokio::sync::mpsc;


impl Dex {
    pub async fn raydium_creation_event(
        &self,
        logs_subscribe: LogsSubscribe<'_>, 
        tx: &mpsc::Sender<(String, Dex)>
    ) -> () {
        let logs_value: LogsSubscribeValue = logs_subscribe.params.result.value;
        let logs: Vec<&str> = logs_value.logs;
        let mut is_creation_instruction: bool = false;
        let mut is_successfully_created: bool = false; 
    
        for log in logs.into_iter() {
            if is_creation_instruction && is_successfully_created { break; }
    
            if log.contains(RAYDIUM_INSTRUCTION_CREATE_NEW_LP) {
                is_creation_instruction = true;
            }
    
            if log.contains(RAYDIUM_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP) {
                is_successfully_created = true;
            }
        }
    
        if is_creation_instruction && is_successfully_created {
            let signature: String = logs_value.signature.to_owned();
            let dex: Dex = self.clone();
            if let Err(e) = tx.send((signature, dex)).await {
                log::error!("Failed to extend signatures channel! {e}");
            }
        }
    }
}