use std::sync::Arc;

pub struct Object {
    class: String,
}

pub struct ObjectRef {
    r: Arc<Object>,
}
