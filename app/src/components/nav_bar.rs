use crate::components::ContentComponent;
use crate::state::AppState;
use crate::views::ViewID;
use egui::{Button, Context, MenuBar, TopBottomPanel, Ui};

pub struct NavBar<'a> {
    id: &'a str,
    label: &'a str,
    home_button_enabled: bool,
}

impl<'a> NavBar<'a> {
    pub fn new(id: &'a str) -> Self {
        Self {
            id,
            label: id,
            home_button_enabled: true,
        }
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn disable_home_button(mut self) -> Self {
        self.home_button_enabled = false;
        self
    }
}

impl ContentComponent for NavBar<'_> {
    fn show(
        self,
        ctx: &Context,
        state: &mut AppState,
        content: impl FnOnce(&mut Ui, &mut AppState),
    ) {
        let Self {
            id,
            label,
            home_button_enabled: menu_button_enabled,
        } = self;

        TopBottomPanel::top(id.to_string()).show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                let home_response = ui.add_enabled(menu_button_enabled, Button::new(" 🏠 "));
                if home_response.clicked() {
                    state.switch_view(ViewID::MainMenu)
                }
                ui.separator();
                ui.label(label);
                ui.separator();
                content(ui, state);
            });
        });
    }
}
