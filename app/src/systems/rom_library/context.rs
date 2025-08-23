use crate::systems::rom_library::action::RomLibraryAction;
use std::cell::RefCell;

#[derive(Debug, Default)]
pub struct RomLibraryContext {
    action_queue: RefCell<Vec<RomLibraryAction>>,
}

impl RomLibraryContext {
    pub fn drain_actions(&self) -> impl Iterator<Item = RomLibraryAction> {
        if let Ok(mut queue) = self.action_queue.try_borrow_mut() {
            queue.drain(..).collect::<Vec<_>>().into_iter()
        } else {
            Vec::new().into_iter()
        }
    }

    pub fn remove(&self, index: usize) {
        self.push_action(RomLibraryAction::Remove(index));
    }

    pub fn select(&self, index: usize) {
        self.push_action(RomLibraryAction::Select(index));
    }

    pub fn push_action(&self, action: RomLibraryAction) {
        if let Ok(mut queue) = self.action_queue.try_borrow_mut() {
            queue.push(action)
        }
    }
}
