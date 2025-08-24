use crate::components::nav_bar::NavBar;
use crate::components::rom_ref_list::RomRefList;
use crate::components::window_renderer::WindowRenderer;
use crate::components::{Component, ContentComponent};
use crate::state::AppState;
use crate::systems::file_picker::FilePickerConfig;
use crate::views::View;
use crate::windows::settings::SettingsWindow;
use egui::{Button, Context, ScrollArea, SidePanel, Ui};
use egui_ltreeview::{NodeBuilder, TreeView, TreeViewBuilder, TreeViewState};
use ndsr_core::codec::data::filesystem::{Filesystem, FilesystemEntry};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct LibraryView {
    settings_window: SettingsWindow,
    #[serde(skip, default)]
    file_tree: TreeViewState<u16>,
}

impl LibraryView {
    fn render_left_panel(&mut self, ui: &mut Ui, state: &mut AppState) {
        ui.horizontal(|ui| {
            let import_response = ui.add(Button::new(" 📥 "));
            if import_response.clicked() {
                state
                    .file_picker
                    .open(FilePickerConfig::pick_multiple(), |state, paths| {
                        state.rom_store_mut().add_roms(&paths);
                    });
            }
            import_response.on_hover_text("Import ROMs");
        });
        ui.separator();

        RomRefList::new(state.rom_store().roms())
            .rom_library_context(state.rom_library.context())
            .selected_index(state.rom_library.selected_rom_index)
            .ui(ui);
    }

    fn render_right_panel(&mut self, ui: &mut Ui, state: &mut AppState) {
        ui.vertical(|ui| {
            let selected_file_nodes = self.file_tree.selected().to_vec();
            ui.horizontal(|ui| {
                let export_response =
                    ui.add_enabled(!selected_file_nodes.is_empty(), Button::new(" 📤 "));
                if export_response.clicked() {
                    state
                        .file_picker
                        .open(FilePickerConfig::pick_directory(), |state, paths| {
                            let Some(output_dir) = paths.first() else {
                                return;
                            };

                            if !output_dir.is_dir() {
                                return;
                            }

                            let Some(rom) = state.rom_store().loaded_rom() else {
                                return;
                            };

                            let Ok(file_system) = rom.get_filesystem() else {
                                return;
                            };

                            for node in selected_file_nodes {
                                let Some(entry) = file_system.get_entry(node) else {
                                    continue;
                                };

                                let children = match entry {
                                    FilesystemEntry::File(file) => vec![node],
                                    FilesystemEntry::Directory(dir) => {
                                        let Some(children) = file_system.get_children(node) else {
                                            continue;
                                        };
                                        children.iter().copied().collect()
                                    }
                                };

                                // ToDo: Reorganize + recursive
                                for child in children {
                                    let Some(export_path) =
                                        file_system.build_path(child, output_dir)
                                    else {
                                        continue;
                                    };

                                    let Some(data) =
                                        file_system.extract_file_data(child, &rom.data)
                                    else {
                                        continue;
                                    };

                                    if let Some(parent) = export_path.parent() {
                                        std::fs::create_dir_all(parent).ok();
                                    }

                                    std::fs::write(export_path, data).ok();
                                }
                            }
                        });
                }
                export_response.on_hover_text("Export selected files to directory");
            });

            ui.separator();

            let Some(rom) = state.rom_store().loaded_rom() else {
                return;
            };

            let Ok(fs) = rom.get_filesystem() else {
                return;
            };

            ScrollArea::vertical().show(ui, |ui| {
                TreeView::new(ui.make_persistent_id("library_file_tree")).show_state(
                    ui,
                    &mut self.file_tree,
                    |builder| {
                        if let Some(root_children) = fs.get_children(Filesystem::ROOT) {
                            for child in root_children {
                                Self::file_tree_rec(*child, &fs, builder);
                            }
                        }
                    },
                );
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
