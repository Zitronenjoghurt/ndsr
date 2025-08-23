use ndsr_core::codec::rom::icon::RomIcon;
use ndsr_core::codec::rom::titles::Titles;
use ndsr_core::codec::rom::NDSRom;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct RomStore {
    roms: Vec<RomRef>,
    #[serde(skip, default = "default_true")]
    dirty: bool,
}

impl Default for RomStore {
    fn default() -> Self {
        Self {
            roms: Vec::new(),
            dirty: true,
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

    pub fn load_rom(&mut self, path: &Path) {
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

    pub fn load_roms(&mut self, paths: &[PathBuf]) {
        for path in paths {
            self.load_rom(path);
        }
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

    fn sort(&mut self) {
        self.roms
            .sort_by(|a, b| a.titles.english.title.cmp(&b.titles.english.title));
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RomRef {
    path: PathBuf,
    identifier: String,
    icon: RomIcon,
    titles: Titles,
}

impl RomRef {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }

    pub fn load(&mut self) -> bool {
        let Ok(data) = std::fs::read(&self.path) else {
            return false;
        };

        let Ok(rom) = NDSRom::from_bytes(&data) else {
            return false;
        };

        let identifier = rom.unique_identifier();
        if identifier == self.identifier {
            return true;
        }

        self.identifier = identifier;
        self.icon = rom.icon;
        self.titles = rom.titles;

        true
    }

    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn titles(&self) -> &Titles {
        &self.titles
    }
}

fn default_true() -> bool {
    true
}
