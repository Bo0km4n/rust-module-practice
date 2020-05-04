use super::user::User;
pub fn new_user(name: String) -> User {
    return User {
        name: name,
        id: 1,
    }
}