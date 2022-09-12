/* Make it work by adding proper lifetime annotations */
struct ImportantExcerpt {
    part: &str,
}

impl ImportantExcerpt {
    fn level(&'a self) -> i32 {
        3
    }
}

fn main() {}
