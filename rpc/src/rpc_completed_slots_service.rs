use {
    crate::rpc_subscriptions::RpcSubscriptions,
    nexis_client::rpc_response::SlotUpdate,
    nexis_ledger::blockstore::CompletedSlotsReceiver,
    nexis_sdk::timing::timestamp,
    std::{
        sync::Arc,
        thread::{Builder, JoinHandle},
    },
};

pub struct RpcCompletedSlotsService;
impl RpcCompletedSlotsService {
    pub fn spawn(
        completed_slots_receiver: CompletedSlotsReceiver,
        rpc_subscriptions: Arc<RpcSubscriptions>,
    ) -> JoinHandle<()> {
        Builder::new()
            .name("nexis-rpc-completed-slots-service".to_string())
            .spawn(move || {
                for slots in completed_slots_receiver.iter() {
                    for slot in slots {
                        rpc_subscriptions.notify_slot_update(SlotUpdate::Completed {
                            slot,
                            timestamp: timestamp(),
                        });
                    }
                }
            })
            .unwrap()
    }
}
