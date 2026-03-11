pub mod delete;
pub mod get;
pub mod new;
pub mod update;

/// Describes which entity to retrieve from the database.
pub enum EntityKind {
    Collection,
    Notebook,
    Note,
    Tag,
}
