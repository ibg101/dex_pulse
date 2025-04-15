use crate::{
    types::{
        custom::Dex,
        rpc::{
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
    pub async fn raydium_lp_creation_event(
        &self,
        logs_subscribe: LogsSubscribe<'_>, 
        tx: &mpsc::Sender<(String, Dex)>
    ) -> () {
        let logs_value: LogsSubscribeValue = logs_subscribe.params.result.value;
        let logs: Vec<&str> = logs_value.logs;
        let mut is_creation_instruction: bool = false;
        let mut is_successfully_created: bool = false;  // not 100%, so check tx.err in order to ensure lp's creation status
    
        for log in logs {
            if !is_creation_instruction && log.find(RAYDIUM_INSTRUCTION_CREATE_NEW_LP).is_some() {
                is_creation_instruction = true;
            }

            if !is_successfully_created && log.find(RAYDIUM_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP).is_some() {
                is_successfully_created = true;
            }

            if is_creation_instruction && is_successfully_created { 
                self.push_signature_to_channel(logs_value.signature, tx).await;
                break; 
            }
        }
    }
}