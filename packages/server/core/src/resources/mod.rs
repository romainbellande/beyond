use crate::utils::fibonacci;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub const MAX_RESOURCE: u8 = 6;
pub const IRON: &str = "iron";
pub const WOOD: &str = "wood";
pub const SAND: &str = "sand";
pub const OIL: &str = "oil";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize, Eq)]
pub struct Resource {
    pub reference: String,
    pub density: u16, // from 0 to 100
}

impl Resource {
    pub fn new(reference: String) -> Self {
        let mut rng = rand::thread_rng();

        Resource {
            reference,
            density: rng.gen_range(0..100),
        }
    }

    pub fn rand_list() -> Vec<Resource> {
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

        let mut resources: Vec<Resource> = Vec::new();
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

            let resource_config_to_add: ResourceConfig =
                resource_config_list[resource_index].clone();

            if !resources
                .clone()
                .iter()
                .any(|item| item.reference == resource_config_to_add.reference)
            {
                resources.push(Resource::new(resource_config_to_add.reference.to_string()));
            }
        }

        resources
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct ResourceConfig<'a> {
    pub reference: &'a str,
    pub frequency: u16, // from 0 to 100
}

pub const IRON_CONFIG: ResourceConfig = ResourceConfig {
    reference: IRON,
    frequency: 40,
};

pub const WOOD_CONFIG: ResourceConfig = ResourceConfig {
    reference: WOOD,
    frequency: 50,
};

pub const SAND_CONFIG: ResourceConfig = ResourceConfig {
    reference: SAND,
    frequency: 20,
};

pub const OIL_RESOURCE: ResourceConfig = ResourceConfig {
    reference: OIL,
    frequency: 10,
};

pub fn get_resource_config_list<'a>() -> Vec<ResourceConfig<'a>> {
    vec![IRON_CONFIG, WOOD_CONFIG, SAND_CONFIG, OIL_RESOURCE]
}
