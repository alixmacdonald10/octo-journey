use std::{collections::HashMap, sync::Arc};

use rand::{distributions::{Distribution, Standard}, Rng};
use serde::Serialize;
use tokio::sync::{RwLock, Mutex};
use uuid::Uuid;
use lazy_static::lazy_static;

lazy_static! {
    static ref NAME_COUNTER: Mutex<usize> = Mutex::new(0);
}

pub(crate) type SharedState = Arc<RwLock<AppState>>;

#[derive(Default, Debug)]
pub(crate) struct AppState {
    pub(crate) untagged_octopi: HashMap<Uuid, UntaggedOctopus>,
    pub(crate) tagged_octopi: HashMap<Uuid, TaggedOctopus>
}


#[derive(Debug, Serialize, Clone)]
pub(crate) struct UntaggedOctopus {
    pub(crate) identifying_feature: IdentifyingFeature,
}

impl Default for UntaggedOctopus {
    fn default() -> Self {
        let identifying_feature: IdentifyingFeature = rand::random();
        
        UntaggedOctopus {
            identifying_feature 
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

#[derive(Debug, Serialize, Clone)]
pub(crate) struct TaggedOctopus {
    pub(crate) name: String,
    pub(crate) identifying_feature: IdentifyingFeature,
}

#[derive(Default, Debug, Serialize, Clone, Eq, PartialEq)]
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


