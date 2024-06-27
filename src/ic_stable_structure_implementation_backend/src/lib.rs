use ic_cdk_macros::export_candid;
mod api;
mod constants;
mod declarations;
mod guards;
mod implementations;
mod memory;
mod state;
mod tests;
mod types;
mod utils;
use crate::declarations::pagination::*;
use crate::declarations::user::UserData;
//export_candid is used to export the canister interface.
export_candid!();
