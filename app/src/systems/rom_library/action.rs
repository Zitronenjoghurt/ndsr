#[derive(Debug, Copy, Clone)]
pub enum RomLibraryAction {
    Remove(usize),
    Select(usize),
}
