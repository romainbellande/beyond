use beyond_core::entities::planet::{AppCollection, Planet};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};

pub struct PlanetRepository {
    collection: Collection<Planet>,
}

impl PlanetRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection::<Planet>(&Planet::get_collection_name()),
        }
    }

    pub async fn find(&self) -> Option<Vec<Planet>> {
        let mut planets: Vec<Planet> = Vec::new();
        let mut cursor = self.collection.find(None, None).await.ok()?;

        while let Ok(Some(planet)) = cursor.try_next().await {
            planets.push(planet);
        }

        Some(planets)
    }
}
