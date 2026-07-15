

pub enum PeerId {
    Uuid(x)
}

pub struct PeerHndl {
    /// The TCP socket wrapped with the `Lines` codec, defined below.
    lines: Framed<TcpStream, LinesCodec>,

    /// Receive half of the message channel.
    rx: Rx,
}

impl PeerHndl {
    pub fn new(
        state: Arc<Mutex<Shared>>,
        lines: Framed<TcpStream, LinesCodec>
    ) -> PeerHndl {
        let addr = lines.get_ref().peer_addr()?;

        let (tx, rx) = mpsc::channel();

        state.lock().await.peers.insert(addr, tx);

        Ok( PeerHndl { lines, rx } )
    }
}
