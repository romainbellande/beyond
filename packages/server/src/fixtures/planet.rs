use beyond_core::entities::planet::AppCollection;
use beyond_core::entities::planet::Planet;
use mongodb::Database;

pub async fn fixture(db: Database) {
    let collection = db.collection::<Planet>(&Planet::get_collection_name());

    collection.drop(None).await.unwrap_or_else(|_| {
        panic!(
            "drop {} collection should succeed",
            &Planet::get_collection_name()
        )
    });

    let planets: Vec<Planet> = Planet::rand("./planets.txt".to_string(), 12);

    collection
        .insert_many(planets.clone(), None)
        .await
        .expect("an issue occured during planets insertion");

    log::info!("inserted {} planets", planets.len());
}
