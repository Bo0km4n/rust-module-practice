mod todo;
mod user;
mod userrepo;

use user::UserAlias;
use todo::todo::Todo;
use userrepo::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let t: Todo = Todo{
        desc: "AAA".to_string(),
        name: "BBB".to_string(),
    };

    let u: UserAlias = UserAlias{
        id: 1,
        name: "Kawabe".to_string(),
    };

    let box_user = Box::new(UserAlias {
        id: 1,
        name: "Kawabe".to_string(),
    });
    let refc_box_user = Rc::new(RefCell::new(box_user));

    let new_u: UserAlias = new_user("Kawabe".to_string());
    println!("user = {:?}, todo = {:?}, new_user = {:?}", u, t, new_u);
    write_user_to_db(Rc::clone(&refc_box_user));
    println!("{:?}", Rc::clone(&refc_box_user).borrow_mut());   
}
