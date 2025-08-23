use crate::state::rom_store::RomStore;
use crate::systems::rom_library::action::RomLibraryAction;
use crate::systems::rom_library::context::RomLibraryContext;

mod action;
pub mod context;

#[derive(Default)]
pub struct RomLibrary {
    context: RomLibraryContext,
    pub selected_rom_index: Option<usize>,
}

impl RomLibrary {
    pub fn update(&mut self, rom_store: &mut RomStore) {
        self.handle_actions(rom_store);
    }

    pub fn context(&self) -> &RomLibraryContext {
        &self.context
    }

    fn handle_actions(&mut self, rom_store: &mut RomStore) {
        for action in self.context.drain_actions() {
            match action {
                RomLibraryAction::Remove(index) => rom_store.remove(index),
                RomLibraryAction::Select(index) => {
                    if Some(index) == self.selected_rom_index {
                        self.selected_rom_index = None;
                        rom_store.reset_loaded_rom();
                    } else {
                        self.selected_rom_index = Some(index);
                        rom_store.load_rom(index);
                    }
                }
            }
        }
    }
}
