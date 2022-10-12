use futures_util:: StreamExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32,Ordering};
use tokio::sync::Mutex;
use tokio::task::{spawn, JoinHandle};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tts_rust::{languages::Languages, GTTSClient};

use crate::socket_serializations::WsObject;
use crate::wiki::get_random_summary;
mod socket_serializations;
mod wiki;

#[tokio::main]
pub async fn main() {
    let narrator: GTTSClient = GTTSClient {
        volume: 0.7,
        language: Languages::Turkish,
    };
    
    let narrator = Arc::new(narrator);

    let url = url::Url::parse("ws://localhost:24050/ws").unwrap();
    let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
    let (_, read) = ws_stream.split();

    let last_miss = Arc::new(AtomicU32::new(0));
    let join_handler: JoinHandle<()> = spawn(async{});
    let join_handler = Arc::new(Mutex::new(join_handler));

    let read_future = read.for_each(|message| async {
        let message = message.unwrap();
        let gosu_data: Option<WsObject> = match message {
            Message::Text(text) => {
                let gosu_data: WsObject = serde_json::from_str(&text).unwrap();
                Some(gosu_data)
            }
            _ => None,
        };
        let misses = gosu_data.unwrap().unwrap_misses();
        
    
        let cloned_handler = join_handler.clone();
        let mut join_handler_lock = cloned_handler.lock().await;
        
        let last_miss_clone = last_miss.clone();
        let last_miss_value = last_miss_clone.load(Ordering::Acquire);

        if misses != 0 && misses != last_miss_value && join_handler_lock.is_finished() {
            let narrator_clone = narrator.clone();
            *join_handler_lock = spawn(async move {
                let summary = get_random_summary().await.unwrap();
                summary.split(". ").for_each(|sentence| {
                    narrator_clone.speak(sentence);
                });
            });
        }
        last_miss_clone.store(misses, Ordering::Release)
    });
    read_future.await;
}
