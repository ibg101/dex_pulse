use crate::types::{
    custom::Dex, 
    rpc::LogsSubscribe
};

use tokio::sync::mpsc;


impl Dex {
    pub async fn pumpswap_lp_creation_event(
        &self,
        logs_subscribe: LogsSubscribe<'_>, 
        tx: &mpsc::Sender<(String, Dex)>
    ) -> () {
        todo!()
    }
}