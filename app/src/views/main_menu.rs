use crate::components::nav_bar::NavBar;
use crate::components::window_renderer::WindowRenderer;
use crate::components::ContentComponent;
use crate::state::AppState;
use crate::views::{View, ViewID};
use crate::windows::settings::SettingsWindow;
use egui::{Button, CentralPanel, Context, Frame};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MainMenuView {
    settings_window: SettingsWindow,
}

impl MainMenuView {
    fn render_center(&mut self, ui: &mut egui::Ui, state: &mut AppState) {
        ui.vertical_centered_justified(|ui| {
            let available_height = ui.available_height();
            let available_width = ui.available_width();
            ui.set_max_width(available_width / 3.0);

            ui.add_space(available_height / 10.0);

            Frame::default()
                .inner_margin(10.0)
                .corner_radius(4.0)
                .shadow(ui.style().visuals.window_shadow)
                .fill(ui.style().visuals.window_fill)
                .stroke(ui.style().visuals.widgets.noninteractive.bg_stroke)
                .show(ui, |ui| {
                    ui.heading("NDSR");
                    ui.label("A Nintendo DS ROM reader");

                    ui.separator();

                    ui.vertical_centered_justified(|ui| {
                        let library_response = ui.add(Button::new("Library"));

                        if library_response.clicked() {
                            self.on_library_clicked(state);
                        }
                    });
                })
        });
    }

    fn on_library_clicked(&self, state: &mut AppState) {
        state.switch_view(ViewID::Library);
    }
}

impl View for MainMenuView {
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        WindowRenderer::new(ctx, state)
            .window(&mut self.settings_window)
            .render();

        NavBar::new("main_menu_nav")
            .label("Main Menu")
            .hide_home_button()
            .show_github_button()
            .settings_window(&mut self.settings_window)
            .show(ctx, state, |_, _| {});

        CentralPanel::default().show(ctx, |ui| {
            self.render_center(ui, state);
        });
    }
}
