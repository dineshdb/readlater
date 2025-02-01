use std::collections::HashMap;

use clap::{Parser, Subcommand};
use pocket::{modify::AddUrlRequest, PocketClient};
use readlater::config::get_config;
use url::Url;
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Pocket {
        #[clap(subcommand)]
        subcommand: PocketCommands,
    },
    Handler {
        #[clap(subcommand)]
        subcommand: HandlerCommands,
    },
    Handle {
        #[arg(long)]
        url: Url,
    },
}

#[derive(Subcommand)]
enum HandlerCommands {
    Register,
}

#[derive(Subcommand)]
enum PocketCommands {
    Get,
    Add { url: Url },
    Archive { items: Vec<u64> },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let config = get_config();
    match args.command {
        Commands::Pocket { subcommand } => {
            let mut pocket = PocketClient::new(&config.consumer_key, &config.access_token);

            match subcommand {
                PocketCommands::Get => {
                    let article = pocket.get(Default::default()).await.unwrap();
                    for (_, article) in article.list {
                        println!("{} {}", article.item_id, article.resolved_title);
                    }
                }
                PocketCommands::Add { url } => {
                    pocket.add(vec![AddUrlRequest::new(url)]).await.unwrap();
                }
                PocketCommands::Archive { items } => {
                    if items.is_empty() {
                        eprintln!("No items to archive");
                        return;
                    }
                    pocket.archive(items).await.unwrap();
                }
            }
        }
        Commands::Handler { subcommand } => match subcommand {
            HandlerCommands::Register => readlater::proto_handler::register_url_handler(),
        },
        Commands::Handle { url } => {
            let url_parts = url::Url::parse(url.as_ref()).unwrap();
            assert_eq!(url_parts.scheme(), "readlater");
            let query_params = url_parts.query_pairs().collect::<HashMap<_, _>>();

            let url = query_params.get("url");
            if url.is_none() {
                eprintln!("No url provided");
                return;
            }
            let url = url.unwrap().to_string();
            let url = Url::parse(&url).expect("malformed url");

            let tags = query_params
                .get("tags")
                .map(|tags| tags.to_string())
                .unwrap_or_default();

            let mut tags = tags
                .split(',')
                .map(|tag| tag.to_string())
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>();
            tags.push("readlater".to_string());

            let mut pocket = PocketClient::new(&config.consumer_key, &config.access_token);
            pocket
                .add(vec![AddUrlRequest::new(url).tags(tags)])
                .await
                .unwrap();
        }
    }
}
