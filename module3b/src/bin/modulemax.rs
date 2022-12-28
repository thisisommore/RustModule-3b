mod max {
    struct Wow {
        name: String,
    }

    pub fn max_pro() {
        println!("Max");
    }

    pub(self) fn public_function_in_nested() {
        println!("called `my_mod::nested::public_function_in_nested()`");
    }

    pub mod nested {
        pub fn what() {
            println!("What");
        }
    }
}

use max::nested;

macro_rules! max_macro {
    () => {
        println!("We are in max_macro");
    };
}
fn main() {
    max_macro!();
    max::max_pro();
    nested::what();
}
