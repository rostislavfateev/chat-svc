
use std::collections::HashMap;
use std::env;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;

/*
use tokio::io::{
    AsyncReadExt,
    AsyncWriteExt
};
*/
use tokio::net::{
    TcpListener,
    TcpStream
};
use tokio::sync::{
    mpsc,
    Mutex
};
use tokio_stream::StreamExt;
use tokio_util::codec::{
    Framed,
    LinesCodec
};
use futures::SinkExt;


// Typedefs
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
/// Transmission part of the message channel.
type Tx = mpsc::UnboundedSender<String>;
/// Receive part of the message channel.
type Rx = mpsc::UnboundedReceiver<String>;


// Constants
const DEFAULT_ADDR: &str = "127.0.0.1:6142";


// Data Structures
/// Set of Tx handles for all clients. On receive, message is broadcasted to all peers.
struct Shared {
    /// Tx handles of connected clients.
    peers: HashMap<SocketAddr, Tx>,
}

impl Shared {
    /// Default constructor.
    pub fn new() -> Shared {
        Shared {
            peers: HashMap::new()
        }
    }

    /// Broadcast message to peers with cleanup.
    async fn broadcast(&mut self,
                       sender: SocketAddr,
                       message: &str) {
        let mut failed_peers = Vec::new();
        let msg_str = message.to_string();

        // Broadcast message
        for (addr, tx) in self.peers.iter() {
            if *addr != sender && tx.send(msg_str.clone()).is_err() {
                failed_peers.push(*addr);
            }
        }

        // Clean up detached peers
        for addr in failed_peers {
            self.peers.remove(&addr);

            tracing::debug!("Removed disconnected peer: {addr}");
        }
    }
}


/// State of each client.
struct Peer {
    /// TCP socket wrapped in lines codec for read/write operations on string data (instead of raw bytes).
    lines: Framed<TcpStream, LinesCodec>,
    /// Receive channel.
    rx: Rx,
}

impl Peer {
    /// Constructor.
    pub async fn new(state: Arc<Mutex<Shared>>,
                     lines: Framed<TcpStream, LinesCodec>) -> io::Result<Peer> {
        // client's socket address
        let addr = lines.get_ref().peer_addr()?;
        let (tx, rx) = mpsc::unbounded_channel();

        state.lock().await
             .peers.insert(addr, tx);

        Ok(Peer { lines, rx })
    }
}

/// Process individual client
async fn process(state: Arc<Mutex<Shared>>,
                 stream: TcpStream,
                 addr: SocketAddr) -> Result<()> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    lines.send("Enter your username:").await?;

    let Some(Ok(username)) = lines.next().await else {
        tracing::error!("Failed to get username from {addr} - client disconnected.");

        return Ok(());
    };

    let mut peer = Peer::new(state.clone(), lines).await?;

    // Client has connected - notify everybody.
    {
        let mut state = state.lock().await;
        let msg = format!("{username} has joined the chat");

        tracing::info!("{msg}");

        state.broadcast(addr, &msg).await;
    }

    // Process messages until stream is exhausted
    loop {
        tokio::select! {
            // message was received from a peer, send it to current user.
            Some(msg) = peer.rx.recv() => {
                if let Err(e) = peer.lines.send(&msg).await {
                    tracing::error!("Failed to send message <{msg}> to {username}: {e:?}");
                    break;
                }
            }

            // message received from the current user, broadcast to peers
            result = peer.lines.next() => match result {
                Some(Ok(msg)) => {
                    let mut state = state.lock().await;
                    let msg = format!("{username}: {msg}");

                    state.broadcast(addr, &msg).await;
                }
                Some(Err(e)) => {
                    tracing::error!("Failed to process message for {username}: {e:?}");
                    break;
                }
                // Stream is exhausted
                None => break,
            },
        }
    }

    // Client disconnected
    {
        let mut state = state.lock().await;
        state.peers.remove(&addr);

        let msg = format!("{username} has left the chat");

        tracing::info!("{msg}");

        state.broadcast(addr, &msg).await;
    }

    Ok(())
}


#[tokio::main]
async fn main() -> Result<()> {
    // logging
    use tracing_subscriber::{fmt::format::FmtSpan, EnvFilter};
    tracing_subscriber::fmt()
        // Filters debug output based on RUST_LOG env var; currently chat-only (but can be RUST_LOG=tokio=trace)
        .with_env_filter(EnvFilter::from_default_env().add_directive("chat=info".parse()?))
        // Log events lifespan (useful with tokio=trace)
        .with_span_events(FmtSpan::FULL)
        .init();

    // Shared state: (held by server task)
    // new client connects -> state handle is cloned -> state handle passed to processing task
    let state = Arc::new(Mutex::new(Shared::new()));

    // Start listening
    let addr = env::args()
                .nth(1)
                .unwrap_or_else(|| DEFAULT_ADDR.to_string());
    let listener = TcpListener::bind(&addr).await?;

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            tracing::debug!("Accepted connection from {addr}");

            if let Err(e) = process(state, stream, addr).await {
                tracing::warn!("Connection from {addr} failed: {e:?}");
            }
        });
    }
}
