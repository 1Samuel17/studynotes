use clap::{Args, Parser, Subcommand};
use database::connection::{check_db, set_db_options};
use sea_orm::Database;

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
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
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

    // Parse command-line arguments
    let cli = Cli::parse();
    // Handle commands based on user input
    match cli.command {
        // Collections command
        Commands::Collections(args) => {
            if args.all {
                println!("Listing all collections...");
            }
            if let Some(collection_name) = args.show {
                println!("Showing notebooks of collection with name: {}", collection_name);
            }
        }
        // Notebooks command
        Commands::Notebooks(args) => {
            if args.all {
                println!("Listing all notebooks...");
            }
            if let Some(notebook_name) = args.show {
                println!("Showing notes of notebook with name: {}", notebook_name);
            }
        }
        // Notes command
        Commands::Notes(args) => {
            if args.all {
                println!("Listing all notes...");
            }
            if let Some(note_name) = args.show {
                println!("Showing content of note with name: {}", note_name);
            }
        }
        // Tags command
        Commands::Tags(args) => {
            if args.all {
                println!("Listing all tags...");
            }
            if let Some(tag_name) = args.show {
                println!("Showing notes associated with tag name: {}", tag_name);
            }
        }
    }

    Ok(())
}
