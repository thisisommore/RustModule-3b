// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

use std::io::{self, Write};

#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let users = [String::from("Om"), String::from("Mayur")].map(|e| User { name: e });

    let mut user_name = String::new();
    print!("Enter user name - ");
    io::stdout().flush();
    io::stdin().read_line(&mut user_name);

    let res = find_user(user_name.trim(), &users);
    match res {
        Some(x) => {
            dbg!(x);
        }
        None => println!("User not found"),
    }
}

fn find_user<'a>(user_name: &str, users: &'a [User; 2]) -> Option<&'a User> {
    users.iter().find(|e| e.name == user_name)
}
