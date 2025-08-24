use crate::components::Component;
use egui::{Align2, Grid, ScrollArea, Ui, Window};
use ndsr_core::codec::rom::NDSRom;

pub struct NDSRomInfo<'a> {
    pub rom: &'a NDSRom,
}

impl<'a> NDSRomInfo<'a> {
    pub fn new(rom: &'a NDSRom) -> Self {
        Self { rom }
    }
}

impl Component for NDSRomInfo<'_> {
    fn ui(self, ui: &mut Ui) {
        let available_height = ui.available_height();

        Window::new(&self.rom.short_title)
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .anchor(Align2::CENTER_TOP, [0.0, available_height / 10.0])
            .show(ui.ctx(), |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        Grid::new("rom_info_grid")
                            .num_columns(2)
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Short Title");
                                ui.label(&self.rom.short_title);
                                ui.end_row();

                                ui.label("Title Code");
                                ui.label(&self.rom.title_code);
                                ui.end_row();

                                ui.label("Maker Code");
                                ui.label(&self.rom.maker_code);
                                ui.end_row();

                                ui.label("Unit Code");
                                ui.label(format!("{:?}", self.rom.unit_code));
                                ui.end_row();

                                ui.label("Category");
                                ui.label(format!("{:?}", self.rom.unique_code_category));
                                ui.end_row();

                                ui.label("Destination Language/Region");
                                ui.label(format!("{:?}", self.rom.destination_language));
                                ui.end_row();
                            });
                    });
                });
            });
    }
}
