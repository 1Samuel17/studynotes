use clap::{Args, Parser, Subcommand};
use database::connection::{check_db, set_db_options};
use database::crud::get::{
    EntityKind, GetAllQueryResult, GetByNameQueryResult, get_all, get_by_name,
};
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
}

#[derive(Args)]
struct NotebookArgs {
    /// Show a list of all notebooks
    #[arg(long)]
    all: bool,
    /// Show a list of the notes of a specific notebook
    #[arg(long)]
    show: Option<String>,
}

#[derive(Args)]
struct NoteArgs {
    /// Show a list of all notes
    #[arg(long)]
    all: bool,
    /// Show the content of a specific note
    #[arg(long)]
    show: Option<String>,
}
#[derive(Args)]
struct TagArgs {
    /// Show a list of all tags
    #[arg(long)]
    all: bool,
    /// Show the notes associated with a specific tag
    #[arg(long)]
    show: Option<String>,
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
            // If --all flag is set, retrieve and display all collections
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
            }
            // If --show flag is set with a collection name, retrieve and display that collection's details
            if let Some(collection_name) = args.show {
                let result = get_by_name(db, EntityKind::Collection, &collection_name).await?;
                match result {
                    Some(GetByNameQueryResult::Collection(collection)) => {
                        println!("\nCollection: {}", collection.name);
                        println!(
                            "  Description:\n{}",
                            serde_json::to_string_pretty(&collection.description)
                                .unwrap_or_else(|_| collection.description.to_string())
                        );
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
            }
        }
        // Notebooks command
        Commands::Notebooks(args) => {
            // If --all flag is set, retrieve and display all notebooks
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
            }
            // If --show flag is set with a notebook name, retrieve and display that notebook's details
            if let Some(notebook_name) = args.show {
                let result = get_by_name(db, EntityKind::Notebook, &notebook_name).await?;
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
            }
        }
        // Notes command
        Commands::Notes(args) => {
            // If --all flag is set, retrieve and display all notes
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
            }
            // If --show flag is set with a note name, retrieve and display that note's details
            if let Some(note_name) = args.show {
                let result = get_by_name(db, EntityKind::Note, &note_name).await?;
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
            }
        }
        // Tags command
        Commands::Tags(args) => {
            // If --all flag is set, retrieve and display all tags
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
            }
            // If --show flag is set with a tag name, retrieve and display that tag's details
            if let Some(tag_name) = args.show {
                let result = get_by_name(db, EntityKind::Tag, &tag_name).await?;
                match result {
                    Some(GetByNameQueryResult::Tag(tag)) => {
                        println!("\nTag: {:?}", tag.tag);
                    }
                    _ => println!("Tag not found."),
                }
            }
        }
    }

    Ok(())
}
