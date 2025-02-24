use chrono::DateTime;
use clap::{Parser, Subcommand};
use localdb::KvConfig;
use pocket::{modify::AddUrlRequest, GetOptions, PocketClient};
use readlater::{
    config::Config,
    native_host::{
        install::{install_linux, Manifest},
        native_host_handler,
    },
};
use std::{collections::HashMap, path::PathBuf};
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
    Auth,
    Sync,
}

#[tokio::main]
async fn main() {
    let config: Config = Config::new().expect("error loading config");
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

    let pool = localdb::open_database(config.database_dir.to_str().unwrap())
        .await
        .unwrap();
    let mut kv_config = KvConfig::new(pool.clone());

    let args = Args::parse();
    match args.command {
        Commands::Pocket { subcommand } => {
            let access_token = kv_config.get_pocket_access_token().await;

            match subcommand {
                PocketCommands::Get => {
                    let access_token = access_token.expect("no access token available");
                    let mut pocket = PocketClient::new(&config.pocket_consumer_key, &access_token);

                    let article = pocket.get(Default::default()).await.unwrap();
                    for (_, article) in article.list {
                        println!("{} {}", article.item_id, article.resolved_title);
                        dbg!(&article);
                    }
                }
                PocketCommands::Add { url } => {
                    let access_token = access_token.expect("no access token available");
                    let mut pocket = PocketClient::new(&config.pocket_consumer_key, &access_token);
                    pocket.add(vec![AddUrlRequest::new(url)]).await.unwrap();
                }
                PocketCommands::Auth => {
                    let redirect_uri = "https://localhost:3000".to_string();
                    let auth_client = pocket::auth::PocketAuthClient::new(
                        config.pocket_consumer_key.clone(),
                        redirect_uri.clone(),
                    );
                    let login_code = auth_client.login_code().await.unwrap();
                    let redirection_uri = pocket::auth::redirection_uri(&login_code, &redirect_uri);
                    println!("Please visit {}", redirection_uri);
                    println!("Please press enter after you have authenticated");
                    let mut input = String::new();
                    std::io::stdin().read_line(&mut input).unwrap();

                    let login_response = auth_client
                        .access_token(&login_code)
                        .await
                        .expect("error getting access token");

                    kv_config
                        .set_pocket_access_token(&login_response.access_token)
                        .await
                        .expect("error setting pocket access token");
                }
                PocketCommands::Archive { items } => {
                    if items.is_empty() {
                        eprintln!("No items to archive");
                        return;
                    }
                    let access_token = access_token.expect("no access token available");
                    let mut pocket = PocketClient::new(&config.pocket_consumer_key, &access_token);
                    pocket.archive(items).await.unwrap();
                }
                PocketCommands::Sync => {
                    let pool = localdb::open_database(config.database_dir.to_str().unwrap())
                        .await
                        .unwrap();
                    let mut db = localdb::LocalDb::new(pool.clone());
                    let since = kv_config.get_pocket_since().await;
                    let offset = kv_config.get_pocket_offset().await;

                    let (since, mut offset) = match (since, offset) {
                        (Some(since), _) => (since, 0),
                        (_, Some(offset)) => (0, offset),
                        _ => (0, 0),
                    };

                    let datetime =
                        DateTime::from_timestamp(since as i64, 0).expect("unexpected date time");
                    println!("Syncing data since {} with offset {}", datetime, offset);

                    let access_token = access_token.expect("no access token available");
                    let mut pocket = PocketClient::new(&config.pocket_consumer_key, &access_token);
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

                        for article in response.list.values() {
                            let article: localdb::Item = article.into();
                            db.add(&article).await.unwrap();
                            println!("{} {}", article.id, article.title);
                        }

                        offset += 30;
                        kv_config.set_pocket_since(response.since).await.unwrap();

                        let has_more = response.has_more().expect("invalid request");
                        if response.list.is_empty() || !has_more {
                            kv_config.set_pocket_since(response.since).await.unwrap();
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

            let access_token = kv_config.get_pocket_access_token().await;
            let access_token = access_token.expect("no access token available");
            let mut pocket = PocketClient::new(&config.pocket_consumer_key, &access_token);
            pocket
                .add(vec![AddUrlRequest::new(url).tags(tags)])
                .await
                .unwrap();
        }
    };
}
