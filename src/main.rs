use crate::wiki::{get_random_summary, Summary};

mod wiki;

#[tokio::main]
async fn main() {
    
    let summary:Summary = get_random_summary().await.unwrap();
    dbg!(summary);

}
