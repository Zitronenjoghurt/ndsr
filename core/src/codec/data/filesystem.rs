use crate::codec::data::filesystem::fat::FAT;
use crate::codec::data::filesystem::fnt::FNT;
use crate::error::NDSRResult;
use std::collections::{BTreeSet, HashMap};

pub mod fat;
pub mod fnt;

#[derive(Debug, Default)]
pub struct Filesystem {
    pub entries: HashMap<u16, FilesystemEntry>,
    // 0xF000 will be the root
    children: HashMap<u16, BTreeSet<u16>>,
}

impl Filesystem {
    pub fn build(fat: &FAT, fnt: &FNT) -> NDSRResult<Self> {
        let mut fs = Self::default();

        for (dir_index, sub_table) in fnt.sub_tables.iter().enumerate() {
            let parent_dir_id = 0xF000 + dir_index as u16;

            let mut file_id = if dir_index == 0 {
                fnt.root_main_entry.first_file_id
            } else {
                fnt.main_table[dir_index - 1].first_file_id
            };

            for sub_entry in sub_table {
                if let Some(sub_dir_id) = sub_entry.sub_dir_id {
                    fs.entries.insert(
                        sub_dir_id,
                        FilesystemEntry::Directory(FilesystemDirectory {
                            name: sub_entry.name.clone(),
                            parent: parent_dir_id,
                        }),
                    );
                } else {
                    let start_address = fat.entries[file_id as usize].start_address;
                    let end_address = fat.entries[file_id as usize].end_address;
                    fs.entries.insert(
                        file_id,
                        FilesystemEntry::File(FilesystemFile {
                            name: sub_entry.name.clone(),
                            address: start_address,
                            size: end_address.saturating_sub(start_address),
                            parent: parent_dir_id,
                        }),
                    );
                    file_id += 1;
                }
            }
        }

        fs.init_parent_children();
        Ok(fs)
    }

    fn init_parent_children(&mut self) {
        for (id, entry) in self.entries.iter() {
            self.children
                .entry(entry.get_parent())
                .or_default()
                .insert(*id);
        }
    }

    pub fn print_tree(&self) {
        self.rec_print_tree(0xF000, 0);
    }

    fn rec_print_tree(&self, id: u16, depth: usize) {
        let indent = format!("|{}", "-".repeat(depth));

        if let Some(entry) = self.entries.get(&id) {
            println!("{}/{}", indent, entry.get_name());
        } else if id == 0xF000 {
            println!("/");
        }

        let children = self.children.get(&id);
        if let Some(children) = children {
            for child in children {
                self.rec_print_tree(*child, depth + 1); // Increase depth!
            }
        }
    }
}

#[derive(Debug)]
pub enum FilesystemEntry {
    Directory(FilesystemDirectory),
    File(FilesystemFile),
}

impl FilesystemEntry {
    pub fn get_parent(&self) -> u16 {
        match self {
            FilesystemEntry::Directory(dir) => dir.parent,
            FilesystemEntry::File(file) => file.parent,
        }
    }

    pub fn get_name(&self) -> &str {
        match self {
            FilesystemEntry::Directory(dir) => &dir.name,
            FilesystemEntry::File(file) => &file.name,
        }
    }
}

#[derive(Debug)]
pub struct FilesystemDirectory {
    pub name: String,
    pub parent: u16,
}

#[derive(Debug)]
pub struct FilesystemFile {
    pub name: String,
    pub address: u32,
    pub size: u32,
    pub parent: u16,
}
