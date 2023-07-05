#[derive(Debug, Clone)]
pub struct Actions {
    actions: Vec<Action>,
}

impl Actions {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    pub fn push(&mut self, action: Action) {
        self.actions.push(action);
    }
}

impl IntoIterator for Actions {
    type Item = Action;

    type IntoIter = std::vec::IntoIter<Action>;

    fn into_iter(self) -> Self::IntoIter {
        self.actions.into_iter()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    SetFlag,
    IncIndex,
    IncPC,
    PushStack,
    PopStack,
}
