use edm::csdl::{EntityType, Key, Property};
use pjl_pg::Database;
use pjl_tab::Table;
use std::env;
use tracing::*;
use tracing_subscriber;
use edm::*;

#[tokio::main]
async fn main() -> Result<(), String> {
    tracing_subscriber::fmt::init();
    info!("Starting dir_stats");
    let args: Vec<String> = env::args().collect();
    let root = if args.len() > 1 { &args[1] } else { "." };
    let pg_url = std::env::var("PG_URL").unwrap_or_else(|_| {
        "host=localhost user=peter password=Kennwort01 dbname=rk".to_string()
    });
    let mut db = Database::new(&pg_url).await.expect("connect to postgres");
    info!("Connected");

    // Ensure the table exists using activate
    let schema = Schema {
        entity_types: vec![EntityType {
            name: "dir_stats".to_string(),
            properties: vec![
                Property { name: "dir".to_string(), ptype: "text".to_string(), .. Default::default() },
                Property { name: "total_size".to_string(), ptype: "bigint".to_string(), .. Default::default()  },
                Property { name: "file_count".to_string(), ptype: "bigint".to_string(), .. Default::default()  },
                Property { name: "newest".to_string(), ptype: "text".to_string(), .. Default::default()  },
                Property { name: "oldest".to_string(), ptype: "text".to_string(), .. Default::default()  },
                Property { name: "date_diff".to_string(), ptype: "bigint".to_string(), .. Default::default()  },
            ],
            key: Some(Key { properties: vec!["dir".to_string()] }),
            .. Default::default()
        }],
        .. Default::default()
    };
    db.activate(schema).await.map_err(|x| x.to_string())?;

    let tab = Table::new();
    tab.add_column("dir").unwrap();
    tab.add_column("total_size").unwrap();
    tab.add_column("file_count").unwrap();
    tab.add_column("newest").unwrap();
    tab.add_column("oldest").unwrap();
    tab.add_column("date_diff").unwrap();
    let mut stack = vec![std::path::Path::new(root).to_path_buf()];
    while let Some(dir) = stack.pop() {
        debug!("Scanning directory: {}", dir.display());
        let mut total_size = 0u64;
        let mut file_count = 0u64;
        let mut newest: Option<std::time::SystemTime> = None;
        let mut oldest: Option<std::time::SystemTime> = None;
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path.clone());
                    debug!("Found subdir: {}", path.display());
                } else if path.is_file() {
                    if let Ok(meta) = std::fs::metadata(&path) {
                        file_count += 1;
                        total_size += meta.len();
                        let mtime = meta.modified().unwrap_or(std::time::UNIX_EPOCH);
                        newest = match newest {
                            Some(n) if n > mtime => Some(n),
                            _ => Some(mtime),
                        };
                        oldest = match oldest {
                            Some(o) if o < mtime => Some(o),
                            _ => Some(mtime),
                        };
                        trace!("File: {} size={} mtime={:?}", path.display(), meta.len(), mtime);
                    }
                }
            }
        }
        let newest_str = newest
            .map(|t| humantime::format_rfc3339(t).to_string())
            .unwrap_or("-".to_string());
        let oldest_str = oldest
            .map(|t| humantime::format_rfc3339(t).to_string())
            .unwrap_or("-".to_string());
        let date_diff = match (newest, oldest) {
            (Some(n), Some(o)) => n.duration_since(o).map(|d| d.as_secs()).unwrap_or(0),
            _ => 0,
        };
        let row = tab.new_row();
        row.set("dir", &dir.display().to_string());
        row.set("total_size", &total_size.to_string());
        row.set("file_count", &file_count.to_string());
        row.set("newest", &newest_str);
        row.set("oldest", &oldest_str);
        row.set("date_diff", &date_diff.to_string());
        info!("Dir: {} size={} files={} newest={} oldest={} date_diff={}", dir.display(), total_size, file_count, newest_str, oldest_str, date_diff);
    }
    // Ensure the dir_stats table exists in the database
    let mut schema = edm::Schema::new();
    schema.new_entity("dir_stats");
    schema.new_property("dir_stats", "dir");
    schema.new_property("dir_stats", "total_size");
    schema.new_property("dir_stats", "file_count");
    schema.new_property("dir_stats", "newest");
    schema.new_property("dir_stats", "oldest");
    schema.new_property("dir_stats", "date_diff");
    schema.new_key("dir_stats", &["dir"]);
    db.activate(schema).await.map_err(|x| x.to_string())?;
    db.modify("dir_stats", tab).await.map_err(|x| x.to_string())?;
    info!("Stats written to Postgres table 'dir_stats'");
    Ok(())
}
