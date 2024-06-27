use ic_stable_structures::StableBTreeMap;

use crate::declarations::storable::{Candid, StoredPrincipal};

use crate::declarations::user::UserData;
use crate::types::memory::VMem;
// The user map or storage.
pub type UserMap = StableBTreeMap<StoredPrincipal, Candid<Vec<UserData>>, VMem>;
