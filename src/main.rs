use std::{collections::HashMap, path::PathBuf};

use clap::{Parser, Subcommand};
use localdb::KvDB;
use pocket::{modify::AddUrlRequest, GetOptions, PocketClient};
use readlater::{
    config::get_config,
    native_host::{
        install::{install_linux, Manifest},
        native_host_handler,
    },
};
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
    Setup,
    Handle {
        #[arg(long)]
        url: Url,
    },
}

#[derive(Subcommand)]
enum PocketCommands {
    Get,
    Add { url: Url },
    Archive { items: Vec<u64> },
    Sync,
}

const DATABASE_PATH: &str = "readlater.sqlite";

#[tokio::main]
async fn main() {
    let config = get_config().expect("error loading config");
    let args: Vec<String> = std::env::args().collect();

    // todo: proper handling so that we don't have to do this
    if args
        .into_iter()
        .any(|arg| arg == "readlater@dbhattarai.info.np")
    {
        // we were called from firefox extension
        native_host_handler(config).await;
        return;
    }

    let args = Args::parse();
    match args.command {
        Commands::Pocket { subcommand } => {
            let mut pocket =
                PocketClient::new(&config.pocket_consumer_key, &config.pocket_access_token);

            match subcommand {
                PocketCommands::Get => {
                    let article = pocket.get(Default::default()).await.unwrap();
                    for (_, article) in article.list {
                        println!("{} {}", article.item_id, article.resolved_title);
                        dbg!(&article);
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
                PocketCommands::Sync => {
                    let pool = localdb::open_database(DATABASE_PATH).await.unwrap();
                    let mut db = localdb::LocalDb::new(pool.clone()).unwrap();
                    let mut kv_db = KvDB::new(pool.clone());
                    let since = kv_db
                        .get_kv::<i32>("pocket_since")
                        .await
                        .map(|k| k.value)
                        .unwrap_or(0);

                    let mut offset = kv_db
                        .get_kv::<i32>("pocket_offset")
                        .await
                        .map(|k| k.value)
                        .unwrap_or(0);

                    loop {
                        let response = pocket
                            .get(GetOptions {
                                since: Some(since),
                                offset: Some(offset),
                                count: 30,
                                ..GetOptions::for_pagination()
                            })
                            .await
                            .unwrap();

                        offset += 30;
                        kv_db
                            .set_kv(&("pocket_offset", offset).into())
                            .await
                            .unwrap();

                        for article in response.list.values() {
                            let article: localdb::Item = article.into();
                            db.add(&article).await.unwrap();
                            println!("{} {}", article.id, article.title);
                        }

                        let has_more = response.has_more().expect("invalid request");
                        if response.list.is_empty() || !has_more {
                            println!("Since {}", response.since);
                            kv_db
                                .set_kv(
                                    &("pocket_since".to_string(), response.since.to_string())
                                        .into(),
                                )
                                .await
                                .unwrap();
                            break;
                        }
                    }
                }
            }
        }
        Commands::Setup => {
            let cli = std::env::current_exe().unwrap();
            let cli = cli.to_str().to_owned().unwrap();
            readlater::proto_handler::register_url_handler();
            let manifest = Manifest {
                name: "readlater".to_string(),
                description: "readlater native messaging host".to_string(),
                path: PathBuf::from(cli),
                io_type: "stdio".to_string(),
                allowed_extensions: Some(vec!["readlater@dbhattarai.info.np".to_string()]),
            };

            install_linux(&manifest).unwrap();
        }
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

            let mut pocket =
                PocketClient::new(&config.pocket_consumer_key, &config.pocket_access_token);
            pocket
                .add(vec![AddUrlRequest::new(url).tags(tags)])
                .await
                .unwrap();
        }
    };
}
