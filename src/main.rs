mod model;

use model::*;

fn main() {
    let user = User::new(1, "a50c4df7-6c08-428d-ad5a-258a2fadfed7", "demo@example.com", "$ww6%234", 36).unwrap();
    println!("{}", serde_json::to_string(&user).unwrap());
}
