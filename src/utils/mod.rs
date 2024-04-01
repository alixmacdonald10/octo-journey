use std::{collections::HashMap, sync::Arc};

use serde::Serialize;
use tokio::sync::RwLock;
use uuid::Uuid;

pub(crate) type SharedState = Arc<RwLock<AppState>>;

#[derive(Default, Debug)]
pub(crate) struct AppState {
    pub(crate) octopi: HashMap<Uuid, Octopus>,
}

#[derive(Default, Debug, Serialize, Clone)]
pub(crate) struct Octopus {
    pub(crate) status: ResearchStatus,
}

#[derive(Default, Debug, Serialize, Clone)]
pub(crate) enum ResearchStatus {
    #[default]
    UnTagged,
    Tagged,
}
