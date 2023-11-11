use std::sync::Mutex;

use crate::node::node::Node;

pub struct AppState {
    pub vaults: Mutex<Vec<Node>>
}

impl AppState {
    pub fn new() -> Self {
        AppState { vaults: Mutex::new(Vec::new()) }
    }
}