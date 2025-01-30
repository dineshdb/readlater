use clap::{command, Parser, Subcommand};
use getpocket::pocket::modify::AddUrlRequest;
use getpocket::{config::get_config, pocket::PocketClient};
use url::Url;
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Get,
    Add { url: Url },
    Archive { items: Vec<u64> },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = get_config();
    let mut pocket = PocketClient::new(
        &config.consumer_key,
        &config.access_token,
        reqwest::Client::new(),
    );
    match args.command {
        Commands::Get => {
            let article = pocket.get(Default::default()).await.unwrap();
            for (_, article) in article.list {
                println!("{} {}", article.item_id, article.resolved_title);
            }
        }
        Commands::Add { url } => {
            pocket.add(vec![AddUrlRequest::new(url)]).await.unwrap();
        }
        Commands::Archive { items } => {
            if items.is_empty() {
                eprintln!("No items to archive");
                return;
            }
            pocket.archive(items).await.unwrap();
        }
    }
}
