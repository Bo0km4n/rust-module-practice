mod todo;
mod user;
mod userrepo;

use user::UserAlias;
use todo::todo::Todo;
use userrepo::new_user;
fn main() {
    let t: Todo = Todo{
        desc: "AAA".to_string(),
        name: "BBB".to_string(),
    };

    let u: UserAlias = UserAlias{
        id: 1,
        name: "Kawabe".to_string(),
    };

    let new_u: UserAlias = new_user("Kawabe".to_string());
    println!("user = {:?}, todo = {:?}, new_user = {:?}", u, t, new_u);
}
