use std::{collections::HashMap, sync::Arc};

use lazy_static::lazy_static;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use serde::Serialize;
use tokio::sync::{Mutex, RwLock};
use utoipa::ToSchema;
use uuid::Uuid;

lazy_static! {
    static ref NAME_COUNTER: Mutex<usize> = Mutex::new(0);
}

pub(crate) type SharedState = Arc<RwLock<AppState>>;

#[derive(Default, Debug)]
pub(crate) struct AppState {
    pub(crate) untagged_octopi: HashMap<Uuid, UntaggedOctopus>,
    pub(crate) tagged_octopi: HashMap<Uuid, TaggedOctopus>,
}

/// Best described as a quick glance in your Octopus sacks
#[derive(Debug, Serialize, Clone, ToSchema)]
pub(crate) struct OctopiSnapshot {
    pub(crate) untagged_octopi: HashMap<Uuid, UntaggedOctopus>,
    pub(crate) tagged_octopi: HashMap<Uuid, TaggedOctopus>,
}

impl OctopiSnapshot {
    pub(crate) fn new(
        untagged_octopi: HashMap<Uuid, UntaggedOctopus>,
        tagged_octopi: HashMap<Uuid, TaggedOctopus>,
    ) -> Self {
        OctopiSnapshot {
            untagged_octopi,
            tagged_octopi,
        }
    }
}

/// An unknown octopus that you are yet to get acquianted with.
#[derive(Debug, Serialize, Clone, ToSchema)]
pub(crate) struct UntaggedOctopus {
    #[schema(example = IdentifyingFeature::RatherRude)]
    pub(crate) identifying_feature: IdentifyingFeature,
}

impl Default for UntaggedOctopus {
    fn default() -> Self {
        let identifying_feature: IdentifyingFeature = rand::random();

        UntaggedOctopus {
            identifying_feature,
        }
    }
}

impl UntaggedOctopus {
    pub(crate) fn new() -> UntaggedOctopus {
        UntaggedOctopus::default()
    }

    pub(crate) async fn tag(self: &Self) -> TaggedOctopus {
        let mut name = String::new();
        let mut name_counter = NAME_COUNTER.lock().await;

        if *name_counter == 0 as usize {
            name = String::from("Original Barry");
        } else {
            name = format!("Barry {:?}", name_counter);
        }

        *name_counter += 1;

        TaggedOctopus {
            name,
            identifying_feature: self.identifying_feature.clone(),
        }
    }
}

/// A an Octopus thats now your friend. Oooooo friends
#[derive(Debug, Serialize, Clone, ToSchema)]
pub(crate) struct TaggedOctopus {
    pub(crate) name: String,
    #[schema(example = IdentifyingFeature::RatherRude)]
    pub(crate) identifying_feature: IdentifyingFeature,
}

/// As you're rather obeservant you notice all Octopus have some distinguishing feature.
/// And what makes it even stranger is there only appears to be a finite amount!
#[derive(Default, Debug, Serialize, Clone, Eq, PartialEq, ToSchema)]
pub(crate) enum IdentifyingFeature {
    ReadsTooMuchFiction,
    TopHat,
    AngryExpression,
    ChainSmoker,
    #[default]
    RatherRude,
    FundamentalDistrustInGovernments,
    ActuallyADecentBloke,
}

impl Distribution<IdentifyingFeature> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> IdentifyingFeature {
        match rng.gen_range(0..=7) {
            0 => IdentifyingFeature::ReadsTooMuchFiction,
            1 => IdentifyingFeature::TopHat,
            2 => IdentifyingFeature::AngryExpression,
            3 => IdentifyingFeature::ChainSmoker,
            4 => IdentifyingFeature::RatherRude,
            5 => IdentifyingFeature::FundamentalDistrustInGovernments,
            6 => IdentifyingFeature::ActuallyADecentBloke,
            _ => todo!(),
        }
    }
}
