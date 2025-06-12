use std::{
    collections::BTreeMap,
    fs::File,
    io::{Read, Write},
    sync::Arc,
};

use crate::Result;
use tokio::sync::Mutex;
use tracing::{error, info};
pub struct Database {
    pub name: String,
    pub entries: BTreeMap<String, String>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            entries: BTreeMap::new(),
        }
    }
}

pub struct Data {
    pub d: Mutex<Database>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            d: Mutex::new(Database::new()),
        }
    }
}

#[derive(Clone)]
pub struct SharedData {
    pub r: Arc<Data>,
}

impl SharedData {
    pub fn new() -> Self {
        Self {
            r: Arc::new(Data::new()),
        }
    }
    pub async fn db_save(self) -> Result<()> {
        let x = self.r.d.lock().await;
        match File::create(x.name.clone()) {
            Ok(file) => {
                let mut zip = zip::ZipWriter::new(file);

                let options = zip::write::FileOptions::default()
                    .compression_method(zip::CompressionMethod::Bzip2);
                for k in x.entries.keys() {
                    zip.start_file(k, options.clone())?;
                    zip.write(x.entries.get(k).unwrap().as_bytes())?;
                }

                zip.finish()?;
            }
            Err(e) => error!("{e}"),
        }
        Ok(())
    }

    pub async fn db_open(fname: String) -> Result<SharedData> {
        match File::open(fname.clone()) {
            Ok(file) => match zip::ZipArchive::new(file) {
                Ok(mut archive) => {
                    let db = SharedData::new();
                    {
                        let mut x = db.r.d.lock().await;
                        x.name = fname.clone();
                        for i in 0..archive.len() {
                            let mut entry = archive.by_index(i).unwrap();
                            if let Some(outpath) = entry.enclosed_name() {
                                let key: String = outpath.to_str().unwrap().into();
                                info!("entry: {:?}", entry.last_modified());

                                let mut buffer = String::new();
                                entry.read_to_string(&mut buffer).expect("read to string");
                                // let s = std::str::from_utf8(buffer.as_slice()).expect("utf8");
                                x.entries.insert(key, buffer);
                            };
                        }
                    }
                    Ok(db)
                }
                Err(e) => {
                    error!("archive: {e}");
                    Err(Box::new(e))
                }
            },
            Err(e) => {
                error!("zip file: {fname} <- {e}");
                Err(Box::new(e))
            }
        }
    }
}
