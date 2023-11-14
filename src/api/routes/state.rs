use crate::node::node::Node;
use std::sync::Mutex;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct AppState {
    pub vaults: Mutex<Vec<Node>>,
    pub replicas: Mutex<Vec<Node>>,
}

impl AppState {
    pub fn new(vaults: Mutex<Vec<Node>>) -> Self {
        AppState {
            vaults: vaults.into(),
            replicas: Mutex::new(Vec::new()),
        }
    }
}
