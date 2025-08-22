use crate::components::nav_bar::NavBar;
use crate::components::ContentComponent;
use crate::state::AppState;
use crate::views::View;
use egui::{CentralPanel, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct LibraryView;

impl View for LibraryView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        NavBar::new("library_nav")
            .label("Library")
            .show(ctx, state, |_, _| {});
        CentralPanel::default().show(ctx, |ui| {});
    }
}
