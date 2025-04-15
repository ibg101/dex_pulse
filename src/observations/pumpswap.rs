use crate::{
    types::{
        custom::Dex, 
        rpc::{
            LogsSubscribe, 
            LogsSubscribeValue
        }
    },
    constants::{
        PUMPSWAP_AMM_PROGRAM_ID,
        PUMPSWAP_INSTRUCTION_CREATE_NEW_LP
    }
};

use tokio::sync::mpsc;


impl Dex {
    pub async fn pumpswap_lp_creation_event(
        &self,
        logs_subscribe: LogsSubscribe<'_>, 
        tx: &mpsc::Sender<(String, Dex)>
    ) -> () {
        let logs_value: LogsSubscribeValue = logs_subscribe.params.result.value;
        let logs: Vec<&str> = logs_value.logs;
        let mut is_pumpswap_amm: bool = false;
        let mut is_creation_instruction: bool = false;

        for log in logs {
            if !is_pumpswap_amm && log.find(PUMPSWAP_AMM_PROGRAM_ID).is_some() {
                is_pumpswap_amm = true;
            }

            if !is_creation_instruction && log.find(PUMPSWAP_INSTRUCTION_CREATE_NEW_LP).is_some() {
                is_creation_instruction = true;
            }

            if is_creation_instruction && is_pumpswap_amm { 
                self.push_signature_to_channel(logs_value.signature, tx).await; 
                break; 
            }
        }
    }
}