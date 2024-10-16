use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(Parser)]
#[command(name = "Wikipedia CLI")]
#[command(about = "A CLI tool to interact with Wikipedia", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search Wikipedia pages
    Search {
        /// The search query
        #[arg(long)]
        query: String,
    },
    // Get a Wikipedia page content
    Content {
        /// The search query
        #[arg(long)]
        query: String,
    },
    /// Get images from a Wikipedia page
    Images {
        /// The search query
        #[arg(long)]
        query: String,
    },
    /// Get categories from a Wikipedia page
    Categories {
        /// The search query
        #[arg(long)]
        query: String,
    },
    /// Get links from a Wikipedia page
    Links {
        /// The search query
        #[arg(long)]
        query: String,
    },
    /// Get languages from a Wikipedia page
    Languages {
        /// The search query
        #[arg(long)]
        query: String,
    },
    /// Get views of a Wikipedia page
    Views {
        /// The search query
        #[arg(long)]
        query: String,
        /// The start date in YYYYMMDD format
        #[arg(long)]
        start_date: String,
        /// The number of days
        #[arg(long)]
        nb_days: i64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search { query } => {
            let output = serde_json::to_string_pretty(&wikipedia_client::search(query).await?)?;
            println!("{}", output);
        }
        Commands::Content { query } => {
            let output = wikipedia_client::get_content(query.clone()).await?;
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        Commands::Images { query } => {
            let output = serde_json::to_string_pretty(&wikipedia_client::get_images(query).await?)?;
            println!("{}", output);
        }
        Commands::Categories { query } => {
            let output =
                serde_json::to_string_pretty(&wikipedia_client::get_categories(query).await?)?;
            println!("{}", output);
        }
        Commands::Links { query } => {
            let output = serde_json::to_string_pretty(&wikipedia_client::get_links(query).await?)?;
            println!("{}", output);
        }
        Commands::Languages { query } => {
            let output =
                serde_json::to_string_pretty(&wikipedia_client::get_languages(query).await?)?;
            println!("{}", output);
        }
        Commands::Views {
            query,
            start_date,
            nb_days,
        } => {
            let views = wikipedia_client::get_views(query, start_date, *nb_days).await?;
            println!("{:?}", views);
        }
    }

    Ok(())
}
