use std::{
    any::Any,
    cell::RefCell,
    collections::HashMap,
    fmt::Debug,
    fs::{metadata, read_dir},
    path::Path,
    sync::Arc,
};

use ffmpeg_next::{ffi::NAME_MAX, format::Pixel};
use pjl_error::AppError;
use pjl_odata::ODataQuery;
use pjl_pg::Database;
use processing_node::{MessageDispatch, ObjectID, Value};
use tokio::{
    runtime::{Handle, Runtime},
    sync::Mutex,
};
use tracing::{error, info, instrument, trace};

const REALM: &str = "thumbs";

#[derive(Clone)]
pub enum LocalValue {
    None,
    FileMeta,
    Meta(String),
    DirMeta,
    Dir { id: String, name: String },
    File { dirid: String, name: String },
}

impl LocalValue {
    pub fn is_file_with_name(&self, p_dirid: &str, p_name: &str) -> bool {
        match self {
            Self::File { dirid, name } => p_dirid == dirid && p_name == name,
            _ => false,
        }
    }

    pub fn is_dir_with_name(&self, p_name: &str) -> bool {
        match self {
            Self::Dir { name, .. } => p_name == name,
            _ => false,
        }
    }

    pub fn is_dir_with_id(&self, p_id: &str) -> bool {
        match self {
            Self::Dir { id, .. } => p_id == id,
            _ => false,
        }
    }

    pub fn execute_message(
        self: &Self,
        thumbs: &Thumbs,
        id: usize,
        selector: &str,
        args: &[Value],
    ) -> Value {
        match selector {
            "self" => ObjectID::new_value(REALM, &id.to_string()),
            "new:" => todo!(),
            "files" => match self {
                LocalValue::Dir { id, name } => read_dir_into_value(thumbs, id, name),
                _ => todo!(),
            },
            "exists" => match self {
                LocalValue::None => todo!(),
                LocalValue::FileMeta => todo!(),
                LocalValue::Meta(p) => Value::Boolean(std::fs::exists(p).unwrap()),
                _ => todo!(),
            },
            "thumbAt:" => match self {
                LocalValue::None => todo!(),
                LocalValue::FileMeta => todo!(),
                LocalValue::Meta(v) => match generate_thumb(&v, args[0].to_int().unwrap()) {
                    Ok(v) => v,
                    Err(e) => {
                        error!("generate thumb: {}", e.message());
                        Value::Void
                    }
                },
                _ => todo!(),
            },
            "find:" => match self {
                LocalValue::None => todo!(),
                LocalValue::FileMeta => {
                    let p = args[0].to_string();
                    let id = thumbs.new_object(LocalValue::Meta(p));
                    ObjectID::new_value(REALM, &id.to_string())
                }
                LocalValue::Meta(path) => todo!(),
                _ => todo!(),
            },
            "at:" => match self {
                LocalValue::DirMeta => {
                    let dir_id = args[0].to_string();
                    let objid = if let Some(objid) = thumbs.find_dir(&dir_id) {
                        objid
                    } else {
                        let db = thumbs.db();
                        {
                            let x = std::thread::spawn(move || {
                                new_pg_runtime_thread().block_on(async move {
                                    let mut q = ODataQuery::new_from("path", &HashMap::from([]));
                                    q.add_condition("id", "eq", dir_id.as_str());
                                    let result = db.lock().await.select(q).await;
                                    result
                                })
                            })
                            .join()
                            .unwrap();
                            let mut o = String::new();
                            x.dump(&mut o);
                            info!("result: {}", o);
                            let row = x.row(1);
                            thumbs.new_object(LocalValue::Dir {
                                id: row.get("id").unwrap(),
                                name: row.get("name").unwrap(),
                            })
                        }
                    };
                    ObjectID::new_value(REALM, objid.to_string().as_str())
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

fn read_dir_into_value(thumbs: &Thumbs, id: &String, name: &String) -> Value {
    let mut lst = vec![];
    for f in read_dir(&name).unwrap() {
        if let Ok(f) = f {
            if let Some(idx) = thumbs.find_file(id, f.file_name().to_str().unwrap()) {
                lst.push(ObjectID::new_value(REALM, idx.to_string().as_str()))
            } else {
                let idx = thumbs.new_object(LocalValue::File {
                    dirid: id.clone(),
                    name: name.to_string(),
                });
                lst.push(ObjectID::new_value(REALM, idx.to_string().as_str()))
            }
        }
    }
    info!("file list with length {} produced", lst.len());
    Value::List(lst)
}

pub struct Thumbs {
    objs: RefCell<Vec<Arc<LocalValue>>>,
    db: Arc<Mutex<Database>>,
}

impl Thumbs {
    pub async fn new() -> Self {
        let mut objs = vec![];
        objs.push(Arc::new(LocalValue::None));
        objs.push(Arc::new(LocalValue::FileMeta)); // 1
        objs.push(Arc::new(LocalValue::DirMeta)); // 2
        let db =
            Database::new("host=localhost user=postgres password=Kennwort01 dbname=fdb-test").await;
        Self {
            objs: RefCell::new(objs),
            db: Arc::new(Mutex::new(db)),
        }
    }

    pub fn find_dir(&self, dir_id: &str) -> Option<usize> {
        let objs = self.objs.borrow();
        let x = objs
            .iter()
            .enumerate()
            .find(|(_, x)| x.is_dir_with_name(dir_id));
        match x {
            Some((id, _)) => Some(id),
            None => None,
        }
    }

    pub fn find_file(&self, dir_id: &str, file_name: &str) -> Option<usize> {
        let objs = self.objs.borrow();
        let x = objs
            .iter()
            .enumerate()
            .find(|(_, x)| x.is_file_with_name(dir_id, file_name));
        match x {
            Some((id, _)) => Some(id),
            None => None,
        }
    }

    pub fn get_object_by_id(&self, idx: usize) -> Option<Arc<LocalValue>> {
        let objs = self.objs.borrow();
        let r = objs.get(idx).unwrap();
        Some(r.clone())
    }

    fn new_object(&self, v: LocalValue) -> usize {
        let mut objx = self.objs.borrow_mut();
        objx.push(Arc::new(v));
        objx.len() - 1
    }

    fn db(&self) -> Arc<Mutex<Database>> {
        self.db.clone()
    }
}

impl Debug for Thumbs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Thumbs")
            .field("objs", &self.objs.borrow().len().to_string())
            .finish()
    }
}

impl MessageDispatch for Thumbs {
    #[instrument(skip_all)]
    fn dispatch(&mut self, id: usize, selector: &str, args: &[Value]) -> Value {
        if let Some(o) = self.get_object_by_id(id) {
            o.execute_message(self, id, selector, args)
        } else {
            todo!("object {id} not found.")
        }
    }

    #[instrument]
    fn resolve_id(&self, name: &str) -> usize {
        match name {
            "Files" => 1,
            "Dirs" => 2,
            _ => todo!(),
        }
    }
}

fn new_pg_runtime_thread() -> Runtime {
    tokio::runtime::Builder::new_current_thread()
        .worker_threads(1)
        .thread_name("postgres")
        .enable_all()
        .build()
        .unwrap()
}

fn video_new_scaler(
    decoder: &ffmpeg_next::decoder::Video,
) -> ffmpeg_next::software::scaling::Context {
    tracing::trace!("original size: {}x{}", decoder.width(), decoder.height());
    let dst_w = 480;
    let dst_h = decoder.height() * dst_w / decoder.width();
    // let dst_h = decoder.height();
    // let dst_w = decoder.width();
    tracing::trace!("new size: {}x{}", dst_w, dst_h);

    let scaler = ffmpeg_next::software::scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        dst_w,
        dst_h,
        ffmpeg_next::software::scaling::Flags::BILINEAR,
    )
    .unwrap();
    scaler
}
fn video_convert_to_frame(rgb_frame: ffmpeg_next::frame::Video) -> Vec<u8> {
    let logical_size_3: usize = rgb_frame.width() as usize * rgb_frame.height() as usize * 3;
    let bytes = rgb_frame.data(0);
    tracing::trace!(
        "writing image to buffer: {}x{} -> {} - {}",
        rgb_frame.width(),
        rgb_frame.height(),
        logical_size_3,
        bytes.len()
    );
    let b3: Vec<u8> = vec![];
    let b2 = std::io::Cursor::new(b3);
    let mut b4 = std::io::BufWriter::new(b2);

    image::write_buffer_with_format(
        &mut b4,
        bytes,
        rgb_frame.width(),
        rgb_frame.height(),
        image::ColorType::Rgb8,
        image::ImageFormat::Jpeg,
    )
    .unwrap();
    let inner = b4.into_inner().unwrap().into_inner();
    inner
}

fn generate_thumb(path: &str, pos: i64) -> Result<Value, AppError> {
    if let Ok(meta) = metadata(path) {
        if meta.is_file() {
            info!("thumbnail of {}", path);
            let ictx = &mut ffmpeg_next::format::input(&path)?;
            let input = ictx
                .streams()
                .best(ffmpeg_next::media::Type::Video)
                .ok_or(AppError::new("no input stream"))?;
            let video_stream_index = input.index();
            let context_decoder =
                ffmpeg_next::codec::context::Context::from_parameters(input.parameters())?;
            let mut decoder = context_decoder.decoder().video()?;
            let mut scaler = video_new_scaler(&decoder);
            info!("duration: {}", ictx.duration());
            let start = (ictx.duration() * pos) / 1000;
            ictx.seek(start, start - 1000..start + 1000)?;
            for (stream, x) in ictx.packets() {
                if stream.index() == video_stream_index {
                    decoder.send_packet(&x)?;
                    let mut frame = ffmpeg_next::frame::Video::empty();
                    while decoder.receive_frame(&mut frame).is_ok() {
                        // if let Some(pts) = frame.pts() {
                        //     if pts < 1000000 {
                        //         continue;
                        //     }
                        // }
                        let mut rgb_frame = ffmpeg_next::frame::Video::empty();
                        scaler.run(&frame, &mut rgb_frame)?;
                        let inner = video_convert_to_frame(rgb_frame);

                        // generate a mime type value (to be defined)
                        let v = Value::MimeData("image/jpeg".into(), inner);
                        return Ok(v);
                    }
                }
            }
        }
    }
    println!("thumb generated");
    Ok(Value::Void)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut n = processing_node::Node::new(REALM, Box::new(Thumbs::new().await));
    n.run().await?;
    Ok(())
}
