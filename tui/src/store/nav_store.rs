#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Screens {
    Index,
    Create,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavStore {
    current_screen: Screens,
}

impl NavStore {
    pub fn new() -> Self {
        Self {
            current_screen: Screens::Index,
        }
    }

    pub fn go_to(&mut self, next_screen: Screens) {
        self.current_screen = next_screen;
    }

    pub fn get_current_screen<'a>(&'a self) -> &'a Screens {
        &self.current_screen
    }
}
