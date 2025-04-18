use std::{collections::HashMap, io::Read};

use clap::{Parser, Subcommand};
use pjl_odata::ODataQuery;
use pjl_pg::{Database, SqlTable};
use pjl_tab::Table;
use tracing::{error, level_filters::LevelFilter};

#[derive(Debug, Parser)]
struct Params {
    #[arg(long, short, default_value = "INFO")]
    trace_level: String,

    #[arg(long, default_value("rk"))]
    database: String,

    table_name: String,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Modify,
    Describe,
    Define,
    Select {
        query: String,

        #[arg(long)]
        yaml: bool,
    },
}

#[tokio::main]
async fn main() {
    let args = Params::parse();
    let connection_string = format!(
        "host=localhost user=postgres password=Kennwort01 dbname={}",
        args.database
    );
    let level: LevelFilter = args.trace_level.parse().unwrap();
    tracing_subscriber::fmt().with_max_level(level).init();
    match args.cmd {
        Commands::Modify => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).unwrap();

            let tab2: Table = serde_yaml::from_str(&buf).unwrap();

            if let Ok(mut db) = Database::new(&connection_string).await {
                db.modify(&args.table_name, tab2).await;
            }
        }
        Commands::Select { query, yaml } => {
            if let Ok(mut db) = Database::new(&connection_string).await {
                let q = ODataQuery::new_from(
                    &args.table_name,
                    &HashMap::from([("$filter".into(), query)]),
                );
                let result = db.select(q).await;
                if yaml {
                    let out = serde_yaml::to_string(&result).unwrap();
                    println!("{out}");
                } else {
                    let mut out = String::new();
                    result.dump(&mut out);
                    println!("{out}");
                }
            }
            else {
                error!("connection failed: {connection_string}");
            }
        }
        Commands::Describe => {
            if let Ok(mut db) = Database::new(&connection_string).await {
                let desc = db.describe(&args.table_name).await;
                let out = serde_yaml::to_string(&desc).unwrap();
                // let out = desc;
                println!("{out}");
            }
        }
        Commands::Define => {
            if let Ok(mut db) = Database::new(&connection_string).await {
                let f = std::io::stdin();
                let x = serde_yaml::from_reader::<_, Vec<SqlTable>>(f).unwrap();
                let x = x.iter().find(|x| x.name() == args.table_name).unwrap();
                
                let _desc = db.define(x).await;
            }
        }
    };
}
