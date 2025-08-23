use crate::components::rom_ref_list_entry::RomRefListEntry;
use crate::components::Component;
use crate::state::rom_store::RomRef;
use crate::systems::rom_library::context::RomLibraryContext;
use egui::{ScrollArea, Ui};

pub struct RomRefList<'a> {
    rom_refs: &'a [RomRef],
    id: &'a str,
    rom_library_context: Option<&'a RomLibraryContext>,
    selected_index: Option<usize>,
}

impl<'a> RomRefList<'a> {
    pub fn new(rom_refs: &'a [RomRef]) -> Self {
        Self {
            rom_refs,
            id: "rom_ref_list",
            rom_library_context: None,
            selected_index: None,
        }
    }

    pub fn rom_library_context(mut self, rom_library_context: &'a RomLibraryContext) -> Self {
        self.rom_library_context = Some(rom_library_context);
        self
    }

    pub fn selected_index(mut self, selected_index: Option<usize>) -> Self {
        self.selected_index = selected_index;
        self
    }
}

impl Component for RomRefList<'_> {
    fn ui(self, ui: &mut Ui) {
        let scroll_area = ScrollArea::vertical().id_salt(self.id);

        scroll_area.show(ui, |ui| {
            for (index, rom_ref) in self.rom_refs.iter().enumerate() {
                let selected = Some(index) == self.selected_index;
                let mut entry = RomRefListEntry::new(index, rom_ref).selected(selected);
                if let Some(rom_library_context) = self.rom_library_context {
                    entry = entry.rom_library_context(rom_library_context);
                }
                entry.ui(ui);
            }
        });
    }
}
