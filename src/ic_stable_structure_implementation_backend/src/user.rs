use crate::guards::caller_is_not_anonymous;
use crate::state_handler::{mutate_state, read_state, Candid, StoredPrincipal};
use candid::{CandidType, Deserialize};
use ic_cdk::update;
use ic_cdk_macros::query;

// UserData is a struct that represents the user data. It has three fields: name, email, and profile_pic. The profile_pic field is an optional field that stores the profile picture of the user.
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UserData {
    pub name: String,
    pub email: String,
    pub profile_pic: Option<Vec<u8>>,
}

// PaginationParams is a struct that represents the pagination parameters. It has two fields: page and page_size. The page field represents the page number and the page_size field represents the number of items per page.
#[derive(CandidType, Deserialize)]
pub struct PaginationParams {
    pub page: usize,
    pub page_size: usize,
}

// PaginationResponse is a struct that represents the pagination response. It has two fields: items and total_items. The items field is a vector of items and the total_items field represents the total number of items.
#[derive(CandidType, Deserialize)]
pub struct PaginationResponse<T> {
    items: Vec<T>,
    total_items: usize,
}

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
