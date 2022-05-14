use serde::{Deserialize, Serialize};
use crate::utils::fibonacci;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::Rng;
use crate::resources::{
    IRON,
    OIL,
    WOOD,
    SAND,
    MAX_RESOURCE
};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct PlanetResource {
    pub reference: String,
    pub density: u16, // from 0 to 100
}

impl PlanetResource {
    pub fn new(reference: String) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            reference,
            density: rng.gen_range(0..100),
        }
    }

    pub fn rand_list() -> Vec<PlanetResource> {
        let resource_config_list = get_resource_config_list();

        let mut nb_resources_weight_reversed: Vec<u32> = Vec::new();

        for num in 0..MAX_RESOURCE - 1 {
            nb_resources_weight_reversed.push(fibonacci((num + 1).into()));
        }

        let nb_resources_weight: Vec<u32> =
            nb_resources_weight_reversed.iter().copied().rev().collect();

        let nb_resource_distribution = WeightedIndex::new(&nb_resources_weight).unwrap();

        let mut rng = rand::thread_rng();
        let list: Vec<u8> = (0..MAX_RESOURCE - 1).collect();

        let mut resources: Vec<PlanetResource> = Vec::new();
        let distribution_sample = nb_resource_distribution.sample(&mut rng);
        let max_resources = list[distribution_sample] + 1;

        for _ in 0..max_resources {
            let mut resource_rng = rand::thread_rng();

            let resource_list_frequencies: Vec<u16> = resource_config_list
                .clone()
                .iter()
                .map(|resource_config| resource_config.frequency)
                .collect();

            let resource_distribution = WeightedIndex::new(&resource_list_frequencies).unwrap();
            let resource_index = resource_distribution.sample(&mut resource_rng);

            let resource_config_to_add: PlanetResourceConfig =
                resource_config_list[resource_index].clone();

            if !resources
                .clone()
                .iter()
                .any(|item| item.reference == resource_config_to_add.reference)
            {
                resources.push(PlanetResource::new(resource_config_to_add.reference.to_string()));
            }
        }

        resources
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct PlanetResourceConfig<'a> {
    pub reference: &'a str,
    pub frequency: u16, // from 0 to 100
}

pub const IRON_CONFIG: PlanetResourceConfig = PlanetResourceConfig {
    reference: IRON,
    frequency: 40,
};

pub const WOOD_CONFIG: PlanetResourceConfig = PlanetResourceConfig {
    reference: WOOD,
    frequency: 50,
};

pub const SAND_CONFIG: PlanetResourceConfig = PlanetResourceConfig {
    reference: SAND,
    frequency: 20,
};

pub const OIL_RESOURCE: PlanetResourceConfig = PlanetResourceConfig {
    reference: OIL,
    frequency: 10,
};

pub fn get_resource_config_list<'a>() -> Vec<PlanetResourceConfig<'a>> {
    vec![IRON_CONFIG, WOOD_CONFIG, SAND_CONFIG, OIL_RESOURCE]
}
