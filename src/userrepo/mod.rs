use super::user::User;
use std::rc::Rc;
use std::cell::RefCell;
pub fn new_user(name: String) -> User {
    return User {
        name: name,
        id: 1,
    }
}

pub fn write_user_to_db(req: Rc<RefCell<Box<User>>>) {
    println!("write user data to db = {:?}", req.borrow_mut());
}