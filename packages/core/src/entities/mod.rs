pub mod planet;
pub mod player;

pub trait AppCollection {
    fn get_collection_name() -> String;
}
