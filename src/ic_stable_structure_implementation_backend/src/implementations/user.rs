use crate::declarations::user::UserData;


// implementation for the user data
impl UserData {
    pub fn get_username(&self) -> String {
        self.name.clone()
    }

    pub fn set_username(&mut self, username: String) {
        self.name = username;
    }
}
