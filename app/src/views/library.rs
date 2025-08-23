use crate::components::nav_bar::NavBar;
use crate::components::rom_ref_list::RomRefList;
use crate::components::window_renderer::WindowRenderer;
use crate::components::{Component, ContentComponent};
use crate::state::AppState;
use crate::systems::file_picker::FilePickerConfig;
use crate::views::View;
use crate::windows::settings::SettingsWindow;
use egui::{Context, ScrollArea, SidePanel, Ui};
use egui_ltreeview::{NodeBuilder, TreeView, TreeViewBuilder};
use ndsr_core::codec::data::filesystem::{Filesystem, FilesystemEntry};
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
                        state.rom_store_mut().add_roms(&paths);
                    });
            }
        });
        ui.separator();

        RomRefList::new(state.rom_store().roms())
            .rom_library_context(state.rom_library.context())
            .selected_index(state.rom_library.selected_rom_index)
            .ui(ui);
    }

    fn render_right_panel(&mut self, ui: &mut Ui, state: &mut AppState) {
        let Some(rom) = state.rom_store().loaded_rom() else {
            return;
        };

        let Ok(fs) = rom.get_filesystem() else {
            return;
        };

        let id = ui.make_persistent_id("library_file_tree");
        ScrollArea::vertical().show(ui, |ui| {
            TreeView::new(id).show(ui, |builder| {
                if let Some(root_children) = fs.get_children(Filesystem::ROOT) {
                    for child in root_children {
                        Self::file_tree_rec(*child, &fs, builder);
                    }
                }
            });
        });
    }

    fn file_tree_rec(node: u16, fs: &Filesystem, builder: &mut TreeViewBuilder<u16>) {
        let Some(fs_entry) = fs.get_entry(node) else {
            return;
        };

        match fs_entry {
            FilesystemEntry::Directory(dir) => {
                builder.node(
                    NodeBuilder::dir(node)
                        .default_open(false)
                        .label(format!("🗁 {}", dir.name)),
                );
                if let Some(children) = fs.get_children(node) {
                    for child in children {
                        Self::file_tree_rec(*child, fs, builder)
                    }
                }
                builder.close_dir();
            }
            FilesystemEntry::File(file) => builder.leaf(node, format!("🗋 {}", file.name)),
        }
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

        SidePanel::right("library_right_panel").show(ctx, |ui| {
            self.render_right_panel(ui, state);
        });
    }
}
