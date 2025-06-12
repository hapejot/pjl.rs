pub struct ValueMapping<From, To> {
    map: Vec<(From, To)>,
}

impl<From, To> ValueMapping<From, To>
where
    From: Clone + PartialEq,
    To: Clone + PartialEq,
{
    pub fn new_from(map: &[(From, To)]) -> Self {
        Self {
            map: map.iter().cloned().collect(),
        }
    }

    pub fn new() -> Self {
        Self { map: vec![] }
    }

    pub fn map(&self, f: &From) -> Option<To> {
        if let Some((_, t)) = self.map.iter().find(|(from, _)| from == f) {
            Some(t.clone())
        } else {
            None
        }
    }

    pub fn rmap(&self, t: &To) -> Option<From> {
        if let Some((f, _)) = self.map.iter().find(|(_, to)| to == t) {
            Some(f.clone())
        } else {
            None
        }
    }
}
