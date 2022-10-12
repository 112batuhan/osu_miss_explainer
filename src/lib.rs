use std::sync::Arc;

use parking_lot::Mutex;
use tokio_tungstenite::connect_async;
use tts_rust::{languages::Languages, GTTSClient};

pub mod socket_serializations;
pub mod wiki;

use wiki::get_random_summary;

pub struct Context {
    pub last_miss: Mutex<u32>,
    pub state: Mutex<ReadState>,
    pub narrator: Arc<GTTSClient>,
}

impl Context {
    pub fn new(last_miss: u32, state: ReadState, narrator: GTTSClient) -> Self {
        Self {
            last_miss: Mutex::new(last_miss),
            state: Mutex::new(state),
            narrator: Arc::new(narrator),
        }
    }

    pub fn is_idle(&self) -> bool {
        *self.state.lock() == ReadState::Idle
    }

    pub fn is_same_miss(&self, current_miss: u32) -> bool {
        *self.last_miss.lock() == current_miss
    }

    pub async fn read_wiki_page(&self) {
        *self.state.lock() = ReadState::Reading;

        let (tx, rx) = tokio::sync::oneshot::channel();
        let narrator_ref = self.narrator.clone();

        tokio::spawn(async move {
            let summary = get_random_summary().await.unwrap();
            summary.split(". ").for_each(|sentence| {
                narrator_ref.speak(sentence);
            });
            _ = tx.send(());
        });

        _ = rx.await;
        *self.state.lock() = ReadState::Idle;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReadState {
    Idle,
    Reading,
}

pub fn get_narrator() -> GTTSClient {
    let narrator: GTTSClient = GTTSClient {
        volume: 0.7,
        language: Languages::Turkish,
    };

    narrator
}

pub async fn get_gosu_ws_handle(
) -> tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>> {
    let url = url::Url::parse("ws://localhost:24050/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    ws_stream
}
