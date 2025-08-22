use crate::state::AppState;
use crate::views::{View, ViewManager};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct NDSRApp {
    state: AppState,
    view_manager: ViewManager,
}

impl NDSRApp {
    pub fn new(cc: &eframe::CreationContext) -> Self {
        cc.storage
            .and_then(|storage| eframe::get_value::<Self>(storage, eframe::APP_KEY))
            .unwrap_or_default()
    }
}

impl eframe::App for NDSRApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.view_manager.render(ctx, &mut self.state);
        self.state.update(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
}
