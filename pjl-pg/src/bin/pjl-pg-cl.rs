use std::{collections::HashMap, io::Read};

use clap::{Parser, Subcommand};
use pjl_odata::ODataQuery;
use pjl_pg::Database;
use pjl_tab::Table;
use tracing::level_filters::LevelFilter;

#[derive(Debug, Parser)]
struct Params {
    #[arg(long, short, default_value = "INFO")]
    trace_level: String,
    table_name: String,
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Modify,
    Select {
        query: String,

        #[arg(long)]
        yaml: bool,
    },
}

#[tokio::main]
async fn main() {
    let args = Params::parse();

    let level: LevelFilter = args.trace_level.parse().unwrap();
    tracing_subscriber::fmt().with_max_level(level).init();
    match args.cmd {
        Commands::Modify => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).unwrap();

            let tab2: Table = serde_yaml::from_str(&buf).unwrap();

            if let Ok(mut db) =
                Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await
            {
                db.modify(&args.table_name, tab2).await;
            }
        }
        Commands::Select { query, yaml } => {
            if let Ok(mut db) =
                Database::new("host=localhost user=postgres password=Kennwort01 dbname=rk").await
            {
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
        }
    };
}
