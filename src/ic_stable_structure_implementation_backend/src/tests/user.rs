use crate::declarations::user::UserData;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_username() {
        let mut state = UserData::default();
        state.set_username("Rohan".to_string());
        assert_eq!(state.get_username(), "Rohan");
    }
}
