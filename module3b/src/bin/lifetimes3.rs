fn invalid_output<'a>() -> &'a str {
    "foo"
}

// fix the error and print invalid_output()
fn main() {
    println!("Ye lo print {}", invalid_output());
}
