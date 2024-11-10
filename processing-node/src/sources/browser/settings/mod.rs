use super::actionmap::ActionMap;

pub struct Settings {
    actions: ActionMap,
}

impl Settings {
    pub fn new() -> Self {
        let actions = ActionMap::new();
        Self { actions }
    }

    pub fn actions(&self) -> &ActionMap {
        &self.actions
    }
}
