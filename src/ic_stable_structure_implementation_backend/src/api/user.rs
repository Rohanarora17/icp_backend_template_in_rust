use crate::declarations::pagination::{PaginationParams, PaginationResponse};
use crate::declarations::storable::{Candid, StoredPrincipal};
use crate::declarations::user::UserData;
use crate::guards::caller_is_not_anonymous;
use crate::utils::user::{mutate_state, read_state};
use ic_cdk::update;
use ic_cdk_macros::query;
// get_user_data is used to get the user data of the caller. It returns a vector of UserData. This function is annotated with the guard caller_is_not_anonymous which checks if the caller is not anonymous.
#[query(guard = "caller_is_not_anonymous")]
fn get_user_data() -> Vec<UserData> {
    let stored_principal = StoredPrincipal(ic_cdk::caller());
    read_state(|s| s.user_data.get(&stored_principal).unwrap_or_default().0)
}

// add_user_data is used to add user data to the canister state. It takes a UserData object as an argument and returns a string. This function is annotated with the guard caller_is_not_anonymous which checks if the caller is not anonymous.
#[update(guard = "caller_is_not_anonymous")]
fn add_user_data(user_details: UserData) -> String {
    let stored_principal = StoredPrincipal(ic_cdk::caller());

    mutate_state(|state| {
        let user_data = &mut state.user_data;
        let mut tokens = user_data.get(&stored_principal).unwrap_or_default().0;

        if !tokens.is_empty() {
            tokens.push(user_details);
            user_data.insert(stored_principal, Candid(tokens));
        } else {
            user_data.insert(stored_principal, Candid(vec![user_details]));
        }
    });
    "user data added successfully".to_string()
}

//example of Pagination
// list_all_users is used to list all the users in the canister state. It takes a PaginationParams object as an argument and returns a PaginationResponse of UserData. This function is annotated with the guard caller_is_not_anonymous which checks if the caller is not anonymous.
#[query(guard = "caller_is_not_anonymous")]
pub fn list_all_users(pagination_params: PaginationParams) -> PaginationResponse<Vec<UserData>> {
    let page = pagination_params.page;
    let page_size = pagination_params.page_size;

    read_state(|state| {
        let data = &state.user_data;

        let mut all_users: Vec<Vec<UserData>> = vec![];
        for (_, users) in data.iter() {
            all_users.push(users.0.clone());
        }

        let total_items = all_users.len();

        let start = (page - 1) * page_size;
        let end = (start + page_size).min(total_items);

        let items = if start < end {
            all_users[start..end].to_vec()
        } else {
            Vec::new()
        };

        PaginationResponse { items, total_items }
    })
}

// set_username is used to set the username of the caller. It takes a string as an argument and returns nothing. This function is annotated with the guard caller_is_not_anonymous which checks if the caller is not anonymous.
// This function first checks if the caller is already present in the user_data map. If the caller is present, it updates the username. If the caller is not present, it creates a new UserData object and inserts it into the user_data map.
#[update]
pub fn set_username(username: String) -> String {
    let caller_id = ic_cdk::caller();

    mutate_state(|state| {
        let user_data = &mut state.user_data;

        let mut user = user_data
            .get(&StoredPrincipal(caller_id))
            .unwrap_or_default()
            .0;

        if !user.is_empty() {
            for user_data in user.iter_mut() {
                user_data.set_username(username.clone());
            }
            "Username updated successfully".to_string()
        } else {
            "User not found.".to_string()
        }
    })
}
