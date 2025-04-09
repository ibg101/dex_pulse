use crate::{
    types::{
        custom::Dex,
        rpc::{
            LogsSubscribe, 
            LogsSubscribeValue
        }
    },
    constants::{
        METEORA_INSTRUCTION_INITIALIZE_POSITION, 
        METEORA_INSTRUCTION_INITIALIZE_BIN_ARRAY,
        METEORA_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP
    }
};

use tokio::sync::mpsc;


impl Dex {
    pub async fn meteora_lp_creation_event(
        &self,
        logs_subscribe: LogsSubscribe<'_>, 
        tx: &mpsc::Sender<(String, Dex)>
    ) -> () {
        let logs_value: LogsSubscribeValue = logs_subscribe.params.result.value;
        let logs: Vec<&str> = logs_value.logs;
        let mut times_bin_array_was_init: u8 = 0; 
        let mut is_init_pos_instruction: bool = false;
        let mut is_successfully_created: bool = false;  // not 100%, so check tx.err in order to ensure lp's creation status
    
        for log in logs {
            if !is_init_pos_instruction && log.find(METEORA_INSTRUCTION_INITIALIZE_POSITION).is_some() {
                is_init_pos_instruction = true;
            }

            if times_bin_array_was_init != 2 && log.find(METEORA_INSTRUCTION_INITIALIZE_BIN_ARRAY).is_some() {
                times_bin_array_was_init += 1;
            }

            if !is_successfully_created && log.find(METEORA_INSTRUCTION_SUCCESSFUL_CREATION_NEW_LP).is_some() {
                is_successfully_created = true;
            }
            
            if is_init_pos_instruction && times_bin_array_was_init == 2 && is_successfully_created { break; }
        }
    
        if is_init_pos_instruction && times_bin_array_was_init == 2 && is_successfully_created {
            self.push_signature_to_channel(logs_value.signature, tx).await;
        }
    }
}