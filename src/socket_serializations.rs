use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WsObject {
    gameplay: Gameplay,
}
#[derive(Deserialize, Debug)]
pub struct Gameplay {
    hits: Hits,
}
#[derive(Deserialize, Debug)]
pub struct Hits {
    #[serde(alias = "0")]
    misses: u32,
}

impl WsObject {
    pub fn unwrap_misses(self) -> u32 {
        self.gameplay.hits.misses
    }
}
