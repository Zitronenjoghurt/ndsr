use crate::state::AppState;
use egui::Context;
use egui_file_dialog::{FileDialog, FileDialogConfig};
use std::path::PathBuf;

pub struct FilePicker {
    file_dialog: FileDialog,
    callback: Option<Box<dyn FnOnce(&mut AppState, Vec<PathBuf>)>>,
}

impl Default for FilePicker {
    fn default() -> Self {
        Self {
            file_dialog: FileDialog::default().add_save_extension("NDS", "nds"),
            callback: None,
        }
    }
}

impl FilePicker {
    pub fn open<F>(&mut self, config: FilePickerConfig, callback: F)
    where
        F: FnOnce(&mut AppState, Vec<PathBuf>) + 'static,
    {
        *self.file_dialog.config_mut() = config.dialog_config;
        self.callback = Some(Box::new(callback));
        match config.mode {
            FilePickerMode::PickSingle => self.file_dialog.pick_file(),
            FilePickerMode::PickMultiple => self.file_dialog.pick_multiple(),
            FilePickerMode::PickDir => self.file_dialog.pick_directory(),
            FilePickerMode::Save => self.file_dialog.save_file(),
        }
    }

    pub fn update(&mut self, ctx: &Context, state: &mut AppState) {
        self.file_dialog.update(ctx);

        let paths = if let Some(path) = self.file_dialog.take_picked() {
            vec![path]
        } else if let Some(paths) = self.file_dialog.take_picked_multiple() {
            paths
        } else {
            return;
        };

        let Some(callback) = self.callback.take() else {
            return;
        };

        callback(state, paths);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilePickerMode {
    #[default]
    PickSingle,
    PickMultiple,
    PickDir,
    Save,
}

#[derive(Default)]
pub struct FilePickerConfig {
    mode: FilePickerMode,
    dialog_config: FileDialogConfig,
}

impl FilePickerConfig {
    pub fn pick_single() -> Self {
        Self {
            mode: FilePickerMode::PickSingle,
            ..Default::default()
        }
    }

    pub fn pick_multiple() -> Self {
        Self {
            mode: FilePickerMode::PickMultiple,
            ..Default::default()
        }
    }

    pub fn pick_directory() -> Self {
        Self {
            mode: FilePickerMode::PickDir,
            ..Default::default()
        }
    }

    pub fn save() -> Self {
        Self {
            mode: FilePickerMode::Save,
            ..Default::default()
        }
    }

    pub fn mode(mut self, mode: FilePickerMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn default_file_name(mut self, default_file_name: &str) -> Self {
        self.dialog_config.default_file_name = default_file_name.to_string();
        self
    }
}
