use candid::{CandidType, Deserialize, Principal};
use core::ops::Deref;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    storable::{Blob, Bound, Storable},
    DefaultMemoryImpl, StableBTreeMap,
};
use std::borrow::Cow;
use std::cell::RefCell;

use crate::user::UserData;

// The virtual memory type used in this example is the default memory implementation.
pub type VMem = VirtualMemory<DefaultMemoryImpl>;

// The user map or storage.
pub type UserMap = StableBTreeMap<StoredPrincipal, Candid<Vec<UserData>>, VMem>;

/* The memory id for the user map. This is the only memory id used in this example.
you can add more memory ids if you want to use more memories.*/

pub const USER_MAP_MEMORY_ID: MemoryId = MemoryId::new(0);


thread_local! {

    // The memory manager.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    // The state of the canister.
    static STATE: RefCell<State> = RefCell::new(
        MEMORY_MANAGER.with(|mm| State {

            user_data: UserMap::init(mm.borrow().get(USER_MAP_MEMORY_ID)),

        })
    );
}


// read_state should be used when you want to read the state of the canister. For eg. when you want to get the user data
pub fn read_state<R>(f: impl FnOnce(&State) -> R) -> R {
    STATE.with(|cell| f(&cell.borrow()))
}


// mutate_state should be used when you want to mutate the state of the canister. For eg. when you want to add user data
pub fn mutate_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}



// The Candid struct is a wrapper around the data that needs to be stored in the canister. It proivdes the implementation of the Storable trait.
#[derive(Default)]
pub struct Candid<T>(pub T)
where
    T: CandidType + for<'de> Deserialize<'de>;

impl<T> Storable for Candid<T>
where
    T: CandidType + for<'de> Deserialize<'de>,
{
    const BOUND: Bound = Bound::Unbounded;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(candid::encode_one(&self.0).expect("encoding should always succeed"))
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(candid::decode_one(bytes.as_ref()).expect("decoding should succeed"))
    }
}

impl<T> Deref for Candid<T>
where
    T: CandidType + for<'de> Deserialize<'de>,
{
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}


// the state struct. This struct contains all the data that needs to be stored in the canister.
pub struct State {
    pub user_data: UserMap,
}



// The StoredPrincipal struct is a wrapper around the Principal struct. It provides the implementation of the Storable trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoredPrincipal(pub Principal);

impl Storable for StoredPrincipal {
    const BOUND: Bound = Blob::<29>::BOUND;

    fn to_bytes(&self) -> Cow<'_, [u8]> {
        Cow::Owned(
            Blob::<29>::try_from(self.0.as_slice())
                .expect("principal length should not exceed 29 bytes")
                .to_bytes()
                .into_owned(),
        )
    }

    fn from_bytes(bytes: Cow<'_, [u8]>) -> Self {
        Self(Principal::from_slice(
            Blob::<29>::from_bytes(bytes).as_slice(),
        ))
    }
}
