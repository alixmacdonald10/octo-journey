use axum::{extract::State, Json};
use axum_macros::debug_handler;
use rand::distributions::{Distribution, Uniform};
use tracing::{event, instrument, Level};
use uuid::Uuid;

use crate::utils::{Octopus, SharedState};

// TODO: handle the errors
/// Endpoint used to check the register of octopi
#[instrument]
pub(crate) async fn spot_check(State(state): State<SharedState>) -> Json<serde_json::Value> {
    event!(Level::INFO, "Checking current information for Octopi!");

    let octopi = &state.read().await.octopi;
    tracing::debug!("octopi: {:?}", octopi);
    Json(serde_json::json!(octopi))
}

// TODO: handle the error
/// Nonensense endpoint which can randomly finds a new octopus and adds to the register.
#[instrument]
pub(crate) async fn analyze(State(state): State<SharedState>) -> Json<serde_json::Value> {
    event!(Level::INFO, "Searching for Octopi to analyze!");
    let random_value = {
        let mut rng = rand::thread_rng();
        let possibilities = Uniform::from(1..100);
        possibilities.sample(&mut rng) 
    }; 

    if random_value > 50 {
        tracing::debug!("No octopus found this time... slipery little buggers");
        return Json(serde_json::json!({"message": "No octopus analyzed!"}))
    } else { 
        tracing::debug!("Octopus found!");

        let octopus_id = Uuid::new_v4();
        let octopus = Octopus::default();
        
        tracing::trace!("Grabbing octopi register");
        {
            let lock = &mut state.write().await;
            let octopi = &mut lock.octopi; 
            octopi.insert(octopus_id, octopus.clone());
        }

        return Json(serde_json::json!(octopus))
    }
}
