//you can create implementation for any struct or enum in this folder
//you can also create new files in this folder
use ic_stable_structures::{memory_manager::MemoryManager, DefaultMemoryImpl};

use crate::constants::memory::USER_MAP_MEMORY_ID;
use crate::declarations::state::State;
use crate::types::user::UserMap;

impl State {
    pub fn new(memory_manager: &MemoryManager<DefaultMemoryImpl>) -> Self {
        Self {
            user_data: UserMap::init(memory_manager.get(USER_MAP_MEMORY_ID)),
        }
    }
}
