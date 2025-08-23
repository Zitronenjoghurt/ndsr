use crate::components::nav_bar::NavBar;
use crate::components::rom_ref_list::RomRefList;
use crate::components::window_renderer::WindowRenderer;
use crate::components::{Component, ContentComponent};
use crate::state::AppState;
use crate::systems::file_picker::FilePickerConfig;
use crate::views::View;
use crate::windows::settings::SettingsWindow;
use egui::{Context, SidePanel, Ui};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LibraryView {
    settings_window: SettingsWindow,
}

impl LibraryView {
    fn render_left_panel(&mut self, ui: &mut Ui, state: &mut AppState) {
        ui.horizontal(|ui| {
            if ui.button(" 🗀 ").clicked() {
                state
                    .file_picker
                    .open(FilePickerConfig::pick_multiple(), |state, paths| {
                        state.rom_store_mut().load_roms(&paths);
                    });
            }
        });
        ui.separator();

        RomRefList::new(state.rom_store().roms())
            .rom_library_context(state.rom_library.context())
            .selected_index(state.rom_library.selected_rom_index)
            .ui(ui);
    }
}

impl View for LibraryView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        WindowRenderer::new(ctx, state)
            .window(&mut self.settings_window)
            .render();

        NavBar::new("library_nav")
            .label("Library")
            .settings_window(&mut self.settings_window)
            .show(ctx, state, |_, _| {});

        SidePanel::left("library_left_panel").show(ctx, |ui| {
            self.render_left_panel(ui, state);
        });
    }
}
