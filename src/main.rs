use clap::{command, Parser, Subcommand};
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
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open("url.txt")
                .unwrap();
            use std::io::Write;
            file.write_all(url.as_str().as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    }
}
