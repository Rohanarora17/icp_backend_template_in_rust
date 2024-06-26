use ic_cdk_macros::export_candid;
mod guards;
mod state_handler;
mod user;
use crate::user::*;

//export_candid is used to export the canister interface.
export_candid!();
