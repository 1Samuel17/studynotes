use clap::{Args, Parser, Subcommand};
use database::connection::{check_db, set_db_options};
use database::crud::EntityKind;
use database::crud::delete::delete_one;
use database::crud::get::{GetAllQueryResult, GetByNameQueryResult, get_all, get_one};
use database::crud::new::{CreateResult, NewEntityData, create_one};
use database::crud::update::{UpdateEntityData, UpdateResult, update_one};
use sea_orm::Database;
use tracing_subscriber::EnvFilter;

// Define command-line arguments using clap
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage collections of study notes
    Collections(CollectionArgs),
    /// Manage notebooks within collections
    Notebooks(NotebookArgs),
    /// Manage individual notes
    Notes(NoteArgs),
    /// Manage tags for notes
    Tags(TagArgs),
}

#[derive(Args)]
struct CollectionArgs {
    /// Show a list of all collections
    #[arg(long)]
    all: bool,
    /// Show the notebooks of a specific collection
    #[arg(long)]
    show: Option<String>,
    /// Create a new collection (requires --name and --description)
    #[arg(long)]
    new: bool,
    /// Update an existing collection by name (use --name/--description to set new values)
    #[arg(short, long)]
    update: Option<String>,
    /// Delete a collection by name
    #[arg(short, long)]
    delete: Option<String>,
    /// Name for the collection (used with --new or --update)
    #[arg(short, long)]
    name: Option<String>,
    /// Description for the collection (used with --new or --update)
    #[arg(short, long = "desc")]
    description: Option<String>,
}

#[derive(Args)]
struct NotebookArgs {
    /// Show a list of all notebooks
    #[arg(long)]
    all: bool,
    /// Show a list of the notes of a specific notebook
    #[arg(long)]
    show: Option<String>,
    /// Create a new notebook (requires --name, --description, --collection)
    #[arg(long)]
    new: bool,
    /// Update an existing notebook by name (use --name/--description/--collection to set new values)
    #[arg(short, long)]
    update: Option<String>,
    /// Delete a notebook by name
    #[arg(short, long)]
    delete: Option<String>,
    /// Name for the notebook (used with --new or --update)
    #[arg(short, long)]
    name: Option<String>,
    /// Description as JSON for the notebook (used with --new or --update)
    #[arg(short, long = "desc")]
    description: Option<String>,
    /// Collection name the notebook belongs to (used with --new or --update)
    #[arg(short, long)]
    collection: Option<String>,
}

#[derive(Args)]
struct NoteArgs {
    /// Show a list of all notes
    #[arg(long)]
    all: bool,
    /// Show the content of a specific note
    #[arg(long)]
    show: Option<String>,
    /// Create a new note (requires --name, --topic, --content, --notebook)
    #[arg(long)]
    new: bool,
    /// Update an existing note by name (use --name/--topic/--content/--notebook to set new values)
    #[arg(short, long)]
    update: Option<String>,
    /// Delete a note by name
    #[arg(short, long)]
    delete: Option<String>,
    /// Name for the note (used with --new or --update)
    #[arg(short, long)]
    name: Option<String>,
    /// Topic for the note (used with --new or --update)
    #[arg(short, long)]
    topic: Option<String>,
    /// Content as JSON for the note (used with --new or --update)
    #[arg(short, long)]
    content: Option<String>,
    /// Notebook name the note belongs to (used with --new or --update)
    #[arg(short = 'b', long)]
    notebook: Option<String>,
}
#[derive(Args)]
struct TagArgs {
    /// Show a list of all tags
    #[arg(long)]
    all: bool,
    /// Show the notes associated with a specific tag
    #[arg(long)]
    show: Option<String>,
    /// Create a new tag by its value (e.g. "Important")
    #[arg(long)]
    new: Option<String>,
    /// Delete a tag by its value
    #[arg(long)]
    delete: Option<String>,
}

// Application entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up logging with tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("off")),
        )
        .init();

    // Set up the database connection
    let db_options = set_db_options().await.unwrap();
    let db = &Database::connect(db_options).await?;

    // Check the database connection
    check_db(db).await;

    // synchronizes database schema with entity definitions
    db.get_schema_registry("database::models::*")
        .sync(db)
        .await?;

    // Insert sample data into the database for testing purposes
    // database::sampledata::insert_sample_data(db).await?;

    // Delete sample data from the database after testing
    // database::sampledata::remove_sample_data(db).await?;

    // Parse command-line arguments
    let cli = Cli::parse();
    // Handle commands based on user input
    match cli.command {
        // Collections command
        Commands::Collections(args) => {
            if args.all {
                let result: GetAllQueryResult = get_all(db, EntityKind::Collection).await?;
                if let GetAllQueryResult::Collections(collections) = result {
                    println!("\nCollections:\n");
                    for collection in collections {
                        println!(
                            "- {} | Description: {}",
                            collection.name, collection.description
                        );
                    }
                } else {
                    println!("No collections found.");
                }
            } else if let Some(collection_name) = args.show {
                let result = get_one(db, EntityKind::Collection, &collection_name).await?;
                match result {
                    Some(GetByNameQueryResult::Collection(collection)) => {
                        println!("\nCollection: {}", collection.name);
                        println!("  Description: {}", collection.description);
                        if collection.notebooks.is_empty() {
                            println!("  Notebooks: (none)");
                        } else {
                            println!("  Notebooks:");
                            for notebook in &collection.notebooks {
                                println!(
                                    "    - {} | Description: {} | Collection: {}",
                                    notebook.name, notebook.description, notebook.collection_name
                                );
                            }
                        }
                    }
                    _ => println!("Collection not found."),
                }
            } else if args.new {
                let name = args.name.expect("--name is required when using --new");
                let description = args
                    .description
                    .expect("--description is required when using --new");
                let result =
                    create_one(db, NewEntityData::Collection { name, description }).await?;
                if let CreateResult::Collection(col) = result {
                    println!("Created collection: {}", col.name);
                }
            } else if let Some(current_name) = args.update {
                let result = update_one(
                    db,
                    &current_name,
                    UpdateEntityData::Collection {
                        name: args.name,
                        description: args.description,
                    },
                )
                .await?;
                match result {
                    Some(UpdateResult::Collection(col)) => {
                        println!("Updated collection: {}", col.name);
                    }
                    _ => println!("Collection '{}' not found.", current_name),
                }
            } else if let Some(collection_name) = args.delete {
                let deleted = delete_one(db, EntityKind::Collection, &collection_name).await?;
                if deleted {
                    println!("Deleted collection: {}", collection_name);
                } else {
                    println!("Collection '{}' not found.", collection_name);
                }
            }
        }
        // Notebooks command
        Commands::Notebooks(args) => {
            if args.all {
                let result: GetAllQueryResult = get_all(db, EntityKind::Notebook).await?;
                if let GetAllQueryResult::Notebooks(notebooks) = result {
                    println!("\nNotebooks:\n");
                    for notebook in notebooks {
                        println!(
                            "- {} | Description: {} | Collection: {}",
                            notebook.name, notebook.description, notebook.collection_name
                        );
                    }
                } else {
                    println!("No notebooks found.");
                }
            } else if let Some(notebook_name) = args.show {
                let result = get_one(db, EntityKind::Notebook, &notebook_name).await?;
                match result {
                    Some(GetByNameQueryResult::Notebook(notebook)) => {
                        println!("\nNotebook: {}", notebook.name);
                        println!("  Collection: {}", notebook.collection_name);
                        println!(
                            "  Description:\n{}",
                            serde_json::to_string_pretty(&notebook.description)
                                .unwrap_or_else(|_| notebook.description.to_string())
                        );
                        if notebook.notes.is_empty() {
                            println!("  Notes: (none)");
                        } else {
                            println!("  Notes:");
                            for note in &notebook.notes {
                                println!(
                                    "    - {} | Topic: {} | Notebook: {}",
                                    note.name, note.topic, note.notebook_name
                                );
                            }
                        }
                    }
                    _ => println!("Notebook not found."),
                }
            } else if args.new {
                let name = args.name.expect("--name is required when using --new");
                let desc_str = args
                    .description
                    .expect("--description is required when using --new");
                let description: serde_json::Value = serde_json::from_str(&desc_str)
                    .unwrap_or_else(|_| serde_json::json!({ "text": desc_str }));
                let collection_name = args
                    .collection
                    .expect("--collection is required when using --new");
                let result = create_one(
                    db,
                    NewEntityData::Notebook {
                        name,
                        description,
                        collection_name,
                    },
                )
                .await?;
                if let CreateResult::Notebook(nb) = result {
                    println!(
                        "Created notebook: {} (collection: {})",
                        nb.name, nb.collection_name
                    );
                }
            } else if let Some(current_name) = args.update {
                let description = args.description.map(|d| {
                    serde_json::from_str(&d).unwrap_or_else(|_| serde_json::json!({ "text": d }))
                });
                let result = update_one(
                    db,
                    &current_name,
                    UpdateEntityData::Notebook {
                        name: args.name,
                        description,
                        collection_name: args.collection,
                    },
                )
                .await?;
                match result {
                    Some(UpdateResult::Notebook(nb)) => {
                        println!("Updated notebook: {}", nb.name);
                    }
                    _ => println!("Notebook '{}' not found.", current_name),
                }
            } else if let Some(notebook_name) = args.delete {
                let deleted = delete_one(db, EntityKind::Notebook, &notebook_name).await?;
                if deleted {
                    println!("Deleted notebook: {}", notebook_name);
                } else {
                    println!("Notebook '{}' not found.", notebook_name);
                }
            }
        }
        // Notes command
        Commands::Notes(args) => {
            if args.all {
                let result: GetAllQueryResult = get_all(db, EntityKind::Note).await?;
                if let GetAllQueryResult::Notes(notes) = result {
                    println!("\nNotes:\n");
                    for note in notes {
                        println!(
                            "- {} | Topic: {} | Notebook: {}",
                            note.name, note.topic, note.notebook_name
                        );
                    }
                } else {
                    println!("No notes found.");
                }
            } else if let Some(note_name) = args.show {
                let result = get_one(db, EntityKind::Note, &note_name).await?;
                match result {
                    Some(GetByNameQueryResult::Note(note)) => {
                        println!("\nNote: {}", note.name);
                        println!("  Topic: {}", note.topic);
                        println!("  Notebook: {}", note.notebook_name);
                        println!("  Collection: {}", note.collection_name);
                        let tag_strs: Vec<String> =
                            note.tags.iter().map(|t| format!("{:?}", t)).collect();
                        if tag_strs.is_empty() {
                            println!("  Tags: (none)");
                        } else {
                            println!("  Tags: {}", tag_strs.join(", "));
                        }
                        println!(
                            "  Content:\n{}",
                            serde_json::to_string_pretty(&note.content)
                                .unwrap_or_else(|_| note.content.to_string())
                        );
                    }
                    _ => println!("Note not found."),
                }
            } else if args.new {
                let name = args.name.expect("--name is required when using --new");
                let topic = args.topic.expect("--topic is required when using --new");
                let content_str = args
                    .content
                    .expect("--content is required when using --new");
                let content: serde_json::Value = serde_json::from_str(&content_str)
                    .unwrap_or_else(|_| serde_json::json!({ "text": content_str }));
                let notebook_name = args
                    .notebook
                    .expect("--notebook is required when using --new");
                let result = create_one(
                    db,
                    NewEntityData::Note {
                        name,
                        topic,
                        content,
                        notebook_name,
                    },
                )
                .await?;
                if let CreateResult::Note(n) = result {
                    println!("Created note: {} (notebook: {})", n.name, n.notebook_name);
                }
            } else if let Some(current_name) = args.update {
                let content = args.content.map(|c| {
                    serde_json::from_str(&c).unwrap_or_else(|_| serde_json::json!({ "text": c }))
                });
                let result = update_one(
                    db,
                    &current_name,
                    UpdateEntityData::Note {
                        name: args.name,
                        topic: args.topic,
                        content,
                        notebook_name: args.notebook,
                    },
                )
                .await?;
                match result {
                    Some(UpdateResult::Note(n)) => {
                        println!("Updated note: {}", n.name);
                    }
                    _ => println!("Note '{}' not found.", current_name),
                }
            } else if let Some(note_name) = args.delete {
                let deleted = delete_one(db, EntityKind::Note, &note_name).await?;
                if deleted {
                    println!("Deleted note: {}", note_name);
                } else {
                    println!("Note '{}' not found.", note_name);
                }
            }
        }
        // Tags command
        Commands::Tags(args) => {
            if args.all {
                let result: GetAllQueryResult = get_all(db, EntityKind::Tag).await?;
                if let GetAllQueryResult::Tags(tags) = result {
                    println!("\nTags:\n");
                    for tag in tags {
                        println!("- {:?}", tag.tag);
                    }
                } else {
                    println!("No tags found.");
                }
            } else if let Some(tag_name) = args.show {
                let result = get_one(db, EntityKind::Tag, &tag_name).await?;
                match result {
                    Some(GetByNameQueryResult::Tag(tag)) => {
                        println!("\nTag: {:?}", tag.tag);
                    }
                    _ => println!("Tag not found."),
                }
            } else if let Some(tag_value) = args.new {
                use database::models::taxonomy::Tag;
                use sea_orm::{ActiveEnum, Iterable};
                let tag_variants: Vec<Tag> = Tag::iter().collect();
                let matched = tag_variants
                    .iter()
                    .find(|t: &&Tag| t.to_value() == tag_value);
                match matched {
                    Some(tag_enum) => {
                        let result = create_one(
                            db,
                            NewEntityData::Tag {
                                tag: tag_enum.clone(),
                            },
                        )
                        .await?;
                        if let CreateResult::Tag(t) = result {
                            println!("Created tag: {:?}", t.tag);
                        }
                    }
                    None => {
                        println!("Unknown tag value '{}'. Available tags:", tag_value);
                        for t in &tag_variants {
                            println!("  - {}", t.to_value());
                        }
                    }
                }
            } else if let Some(tag_name) = args.delete {
                let deleted = delete_one(db, EntityKind::Tag, &tag_name).await?;
                if deleted {
                    println!("Deleted tag: {}", tag_name);
                } else {
                    println!("Tag '{}' not found.", tag_name);
                }
            }
        }
    }

    Ok(())
}
