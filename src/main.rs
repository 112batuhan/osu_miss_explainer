use futures_util::StreamExt;
use osu_miss_explainer::{get_gosu_ws_handle, get_narrator, Context, ReadState};
use std::sync::Arc;

use tokio_tungstenite::tungstenite::protocol::Message;

use crate::socket_serializations::WsObject;
mod socket_serializations;
mod wiki;

#[tokio::main]
pub async fn main() {
    let ws_handle = get_gosu_ws_handle().await;
    let read = ws_handle.split().1; // (write , read)

    let ctx = Arc::new(Context::new(0, ReadState::Idle, get_narrator()));

    let read_future =
        read.for_each(|message| async { handle_message(message, ctx.clone()).await.unwrap() });

    read_future.await;
}

async fn handle_message(
    ws_message: Result<Message, tokio_tungstenite::tungstenite::Error>,
    ctx: Arc<Context>,
) -> anyhow::Result<()> {
    let message = ws_message?;

    let gosu_data: WsObject = match message {
        Message::Text(text) => {
            let gosu_data: WsObject = serde_json::from_str(&text).unwrap();
            gosu_data
        }
        _ => return Ok(()),
    };

    let current_miss = gosu_data.unwrap_misses();
    
    if current_miss != 0 && !ctx.is_same_miss(current_miss) && ctx.is_idle() {
        let ctx_clone = ctx.clone();
        tokio::spawn(async move{
            ctx_clone.read_wiki_page().await;
            
        });
    }
    ctx.update_last_miss(current_miss);
    Ok(())
}
