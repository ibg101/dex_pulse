use crate::{
    types::{
        custom::Dex,
        rpc::{
            LogsSubscribe, 
            LogsSubscribeValue
        }
    },
    constants::{
        METEORA_INSTRUCTION_CREATE_NEW_LP, 
        METEORA_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP
    }
};

use tokio::sync::mpsc;


impl Dex {
    pub async fn meteora_creation_event(
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
    
            if log.contains(METEORA_INSTRUCTION_CREATE_NEW_LP) {
                is_creation_instruction = true;
            }
    
            if log.contains(METEORA_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP) {
                is_successfully_created = true;
            }
        }
    
        if is_creation_instruction && is_successfully_created {
            let signature: String = logs_value.signature.to_owned();
            if let Err(e) = tx.send((signature, *self)).await {
                log::error!("Failed to extend signatures channel! {e}");
            }
        }
    }
}