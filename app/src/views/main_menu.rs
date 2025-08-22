use crate::components::nav_bar::NavBar;
use crate::components::window_button::WindowButton;
use crate::components::{Component, ContentComponent};
use crate::state::AppState;
use crate::views::View;
use crate::windows::settings::SettingsWindow;
use egui::{CentralPanel, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MainMenuView {
    settings_window: SettingsWindow,
}

impl MainMenuView {
    fn render_windows(&mut self, ctx: &Context, state: &mut AppState) {
        self.settings_window.render(ctx, state);
    }
}

impl View for MainMenuView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.render_windows(ctx, state);

        NavBar::new("main_menu_nav")
            .label("Main Menu")
            .disable_home_button()
            .show(ctx, state, |ui, state| {
                WindowButton::new(&mut self.settings_window, "🛠").ui(ui, state);
                ui.separator();
            });

        CentralPanel::default().show(ctx, |ui| {});
    }
}
