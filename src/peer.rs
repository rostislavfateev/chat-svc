/*
use axum::extract::ws::WebSocket; //{Message, Utf8Bytes,
use futures_util::stream::SplitStream;
/// Single client connection representation
use uuid::Uuid;
//pub struct SplitStream<S>(BiLock<S>)


pub struct PeerHndl {
    id: Uuid,
    name: String,
    receiver: SplitStream<WebSocket>,
}

impl PeerHndl {
    pub fn new(name: String, receiver: SplitStream<WebSocket>) -> PeerHndl {
        PeerHndl {
            id: Uuid::new_v4(),
            name,
            receiver,
        }
    }
}
 */
