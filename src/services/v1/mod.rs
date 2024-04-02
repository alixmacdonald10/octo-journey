use std::{collections::HashMap, time::Duration};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use rand::distributions::{Distribution, Uniform};
use tokio::time::sleep;
use tracing::{event, instrument, Level};
use uuid::Uuid;

use crate::utils::{
    IdentifyingFeature, OctopiSnapshot, SharedState, TaggedOctopus, UntaggedOctopus,
};

// TODO: handle the errors
/// A quick check in the bags to see how many Octopi you've kidnapped. Splits the Octopi into
/// untagged and tagged versions.
#[utoipa::path(
    get,
    path = "/v1/spot-check",
    responses(
        (status = 200, description = "Success", body = OctopiSnapshot, example = json!(OctopiSnapshot::new(HashMap::from([(Uuid::new_v4(), UntaggedOctopus::new())]), HashMap::from([(Uuid::new_v4(), TaggedOctopus { name: "original Barry".to_string(), identifying_feature: IdentifyingFeature::RatherRude })]))))
    )
)]
#[instrument]
pub(crate) async fn spot_check(State(state): State<SharedState>) -> impl IntoResponse {
    event!(Level::INFO, "Checking current information for Octopi!");

    let lock = &state.read().await;

    let untagged_octopi = &lock.untagged_octopi;
    tracing::debug!("Untagged octopi: {:?}", untagged_octopi);

    let tagged_octopi = &lock.tagged_octopi;
    tracing::debug!("Tagged octopi: {:?}", tagged_octopi);

    (
        StatusCode::OK,
        Json(OctopiSnapshot::new(
            untagged_octopi.to_owned(),
            tagged_octopi.to_owned(),
        )),
    )
        .into_response()
}

// TODO: handle the error
/// Search the depths for any a hidden Octopus. If found its dragged kicking and screaming and shoved into your UntaggedOctopus register.
#[utoipa::path(
    post,
    path = "/v1/capture",
    responses(
        (status = 201, description = "Octopus captured successfully", body = UntaggedOctopus, example = json!(UntaggedOctopus::new())),
        (status = 200, description = "No Octopus found")
    )
)]
#[instrument]
pub(crate) async fn capture(State(state): State<SharedState>) -> impl IntoResponse {
    event!(Level::INFO, "Searching for Octopi to analyze!");
    let random_value = {
        // range is not send so make sure its out of context prior to awaiting
        let mut rng = rand::thread_rng();
        let possibilities = Uniform::from(1..100);
        possibilities.sample(&mut rng)
    };

    if random_value > 50 {
        tracing::debug!("No octopus found this time... slipery little buggers");
        (StatusCode::OK,).into_response()
    } else {
        tracing::debug!("Octopus found!");

        let octopus_id = Uuid::new_v4();
        let octopus = UntaggedOctopus::new();

        tracing::trace!("Grabbing octopi register");
        {
            let lock = &mut state.write().await;
            let octopi = &mut lock.untagged_octopi;
            octopi.insert(octopus_id, octopus.clone());
        }

        (StatusCode::CREATED, Json(octopus)).into_response()
    }
}

// TODO: handle errors
/// Get your thinking cap out, sit down and pull the Octopi one by one from the Untagged register.
/// Giving each a imaginative name (tagging) and then stuffing them back into the Tagged register.
/// A deliberately long winded function for load testing purposes. Because why not.
#[utoipa::path(
    post,
    path = "/v1/tag",
    responses(
        (status = 200, description = "Octopus succesfully tagged", body = HashMap<Uuid, TaggedOctopus>, example = json!(HashMap::from([(Uuid::new_v4(), TaggedOctopus { name: "original Barry".to_string(), identifying_feature: IdentifyingFeature::RatherRude })]))),
    )
)]
#[instrument]
pub(crate) async fn tag(State(state): State<SharedState>) -> impl IntoResponse {
    event!(Level::INFO, "Tagging currently untagged Octopi!");

    let mut updated_octopi: Vec<(Uuid, TaggedOctopus)> = Vec::new();
    {
        let untagged_octopi = &state.read().await.untagged_octopi;

        for (id, octopus) in untagged_octopi {
            tracing::trace!("Checking Octopus {}", id);
            sleep(Duration::from_millis(100)).await;

            tracing::debug!("Tagging Octopus {}", id);
            let tagged_octopus = octopus.tag().await;

            let _ = &updated_octopi.push((*id, tagged_octopus));
            tracing::trace!("Octopus {} tagged!", id);
        }
    }

    // FIXME: doing this twice is far from ideal but havn't had time to fix it yet!
    let lock = &mut state.write().await;
    let untagged_octopi = &mut lock.untagged_octopi;
    let mut updated_octopi_copy = updated_octopi.clone();
    while let Some((id, _)) = updated_octopi.pop() {
        tracing::trace!("Removing octopus {} from untagged octopi", id);
        untagged_octopi.remove(&id);
    }
    let tagged_octopi = &mut lock.tagged_octopi;
    while let Some((id, octopus)) = updated_octopi_copy.pop() {
        tracing::trace!("Adding octopus {} to tagged octopi", id);
        tagged_octopi.insert(id.to_owned(), octopus.to_owned());
    }
    (StatusCode::OK, Json(tagged_octopi.clone())).into_response()
}
