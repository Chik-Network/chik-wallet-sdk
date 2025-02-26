use chik_protocol::{CoinStateUpdate, Message, ProtocolMessageTypes};
use chik_traits::Streamable;
use tokio::sync::mpsc;

pub fn coin_state_updates(receiver: &mut mpsc::Receiver<Message>) -> Vec<CoinStateUpdate> {
    let mut items = Vec::new();
    while let Ok(message) = receiver.try_recv() {
        if message.msg_type != ProtocolMessageTypes::CoinStateUpdate {
            continue;
        }
        items.push(CoinStateUpdate::from_bytes(&message.data).unwrap());
    }
    items
}
