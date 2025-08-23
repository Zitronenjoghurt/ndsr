use crate::components::Component;
use crate::state::rom_store::RomRef;
use crate::systems::rom_library::context::RomLibraryContext;
use egui::{CursorIcon, Frame, Ui, UiBuilder};

pub struct RomRefListEntry<'a> {
    rom_ref: &'a RomRef,
    rom_library_context: Option<&'a RomLibraryContext>,
    selected: bool,
    index: usize,
}

impl<'a> RomRefListEntry<'a> {
    pub fn new(index: usize, rom_ref: &'a RomRef) -> Self {
        Self {
            rom_ref,
            rom_library_context: None,
            selected: false,
            index,
        }
    }

    pub fn rom_library_context(mut self, context: &'a RomLibraryContext) -> Self {
        self.rom_library_context = Some(context);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Component for RomRefListEntry<'_> {
    fn ui(self, ui: &mut Ui) {
        let desired_size = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        let scope = UiBuilder::new().max_rect(rect);

        let fill = if self.selected {
            ui.style().visuals.selection.bg_fill
        } else if response.hovered() {
            ui.style().visuals.widgets.hovered.bg_fill
        } else {
            ui.style().visuals.widgets.noninteractive.bg_fill
        };

        if response.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::PointingHand);
        }

        ui.scope_builder(scope, |ui| {
            Frame::default()
                .inner_margin(4.0)
                .corner_radius(4.0)
                .fill(fill)
                .stroke(ui.style().visuals.widgets.noninteractive.bg_stroke)
                .show(ui, |ui| {
                    ui.style_mut().interaction.selectable_labels = false;
                    ui.vertical_centered_justified(|ui| {
                        ui.label(&self.rom_ref.titles().english.title);
                        if let Some(subtitle) = &self.rom_ref.titles().english.sub_title {
                            ui.small(subtitle);
                        }
                    });
                });
        });

        if response.clicked()
            && let Some(context) = self.rom_library_context
        {
            context.select(self.index);
        }
    }
}
