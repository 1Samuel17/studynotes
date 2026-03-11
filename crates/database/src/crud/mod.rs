pub mod get;
pub mod new;
pub mod update;
pub mod delete;

/// Describes which entity to retrieve from the database.
pub enum EntityKind {
    Collection,
    Notebook,
    Note,
    Tag,
}