use crate::state::settings::SettingsState;
use crate::systems::file_picker::FilePicker;
use crate::views::ViewID;
use serde::{Deserialize, Serialize};

pub mod settings;

#[derive(Default, Serialize, Deserialize)]
pub struct AppState {
    current_view: ViewID,
    settings: SettingsState,
    #[serde(skip, default)]
    pub file_picker: FilePicker,
}

impl AppState {
    pub fn update(&mut self, ctx: &egui::Context) {
        self.settings.update(ctx);
        self.update_file_picker(ctx);
    }

    pub fn current_view(&self) -> ViewID {
        self.current_view
    }

    pub fn switch_view(&mut self, view: ViewID) {
        self.current_view = view;
    }

    pub fn settings(&self) -> &SettingsState {
        &self.settings
    }

    pub fn settings_mut(&mut self) -> &mut SettingsState {
        &mut self.settings
    }

    fn update_file_picker(&mut self, ctx: &egui::Context) {
        let mut file_picker = std::mem::take(&mut self.file_picker);
        file_picker.update(ctx, self);
        self.file_picker = file_picker;
    }
}
