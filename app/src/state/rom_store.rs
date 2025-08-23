use ndsr_core::codec::rom::icon::RomIcon;
use ndsr_core::codec::rom::titles::Titles;
use ndsr_core::codec::rom::NDSRom;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
pub struct RomStore {
    roms: Vec<RomRef>,
    #[serde(skip, default = "default_true")]
    dirty: bool,
    #[serde(skip, default)]
    loaded_rom: Option<NDSRom>,
    #[serde(skip, default)]
    loaded_rom_index: Option<usize>,
}

impl Default for RomStore {
    fn default() -> Self {
        Self {
            roms: Vec::new(),
            dirty: true,
            loaded_rom: None,
            loaded_rom_index: None,
        }
    }
}

impl RomStore {
    pub fn update(&mut self) {
        if !self.dirty {
            return;
        }
        self.roms.retain_mut(|rom| rom.load());
        self.dirty = false;
    }

    pub fn add_rom(&mut self, path: &Path) {
        let mut rom_ref = RomRef::new(PathBuf::from(path));
        let load_success = rom_ref.load();
        if !load_success {
            return;
        }

        let identifier = &rom_ref.identifier;
        if self
            .roms
            .iter()
            .any(|rom_ref| &rom_ref.identifier == identifier)
        {
            return;
        }

        self.roms.push(rom_ref);

        self.sort();
    }

    pub fn add_roms(&mut self, paths: &[PathBuf]) {
        for path in paths {
            self.add_rom(path);
        }
    }

    pub fn load_rom(&mut self, index: usize) {
        if self.loaded_rom_index == Some(index) {
            return;
        }

        let Some(rom_ref) = self.roms.get(index) else {
            return;
        };

        self.loaded_rom = rom_ref.load_rom();
        self.loaded_rom_index = Some(index);
    }

    pub fn reset_loaded_rom(&mut self) {
        self.loaded_rom = None;
        self.loaded_rom_index = None;
    }

    pub fn loaded_rom(&self) -> Option<&NDSRom> {
        self.loaded_rom.as_ref()
    }

    pub fn refresh(&mut self) {
        self.dirty = true;
    }

    pub fn remove(&mut self, index: usize) {
        self.roms.remove(index);
    }

    pub fn roms(&self) -> &[RomRef] {
        &self.roms
    }

    pub fn roms_mut(&mut self) -> &mut [RomRef] {
        &mut self.roms
    }

    fn sort(&mut self) {
        self.roms
            .sort_by(|a, b| a.titles.english.title.cmp(&b.titles.english.title));
    }
}

#[derive(Default, Serialize, Deserialize)]
pub struct RomRef {
    path: PathBuf,
    identifier: String,
    titles: Titles,
    icon: RomIcon,
    #[serde(skip)]
    icon_texture: RefCell<Option<egui::TextureHandle>>,
}

impl RomRef {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub fn load(&mut self) -> bool {
        let Some(rom) = self.load_rom() else {
            return false;
        };

        let identifier = rom.unique_identifier();
        if identifier == self.identifier {
            return true;
        }

        self.identifier = identifier;
        self.titles = rom.titles;
        self.icon = rom.icon;

        true
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn titles(&self) -> &Titles {
        &self.titles
    }

    pub fn get_icon_texture(&self, ctx: &egui::Context) -> Option<egui::TextureHandle> {
        if self.icon_texture.borrow().is_some() {
            return self.icon_texture.borrow().clone();
        }

        let mut png_bytes = Vec::new();
        self.icon.render_icon_png_32x(&mut png_bytes);

        let Ok(image) = image::load_from_memory(&png_bytes) else {
            return None;
        };

        let rgba = image.to_rgba8();
        let color_image = egui::ColorImage::from_rgba_unmultiplied([32, 32], rgba.as_raw());
        let texture = ctx.load_texture(
            format!("rom_icon_{}", self.identifier),
            color_image,
            Default::default(),
        );

        *self.icon_texture.borrow_mut() = Some(texture.clone());
        Some(texture)
    }

    pub fn load_rom(&self) -> Option<NDSRom> {
        let Ok(data) = std::fs::read(&self.path) else {
            return None;
        };
        NDSRom::from_bytes(&data).ok()
    }
}

fn default_true() -> bool {
    true
}
