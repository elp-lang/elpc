use crate::bindings::ui;

pub enum Target {
    IOS,
    MAC,
}

pub struct App {
    pub target: Target,
}

impl App {
    pub fn new() -> App {
        let app = ui::initialize();
        App {
            target: Target::MAC,
        }
    }
}
