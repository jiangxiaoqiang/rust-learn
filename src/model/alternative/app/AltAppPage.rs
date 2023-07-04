use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct AltAppPage {
    pub items: Vec<AltAppItem>
}
