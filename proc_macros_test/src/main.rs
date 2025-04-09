use penpot_macros::{do_nothing, ToJS};

#[derive(ToJS)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[derive(ToJS)]
struct Country {
    id: u32,
    name: String,
    population: i64,
}

fn main() {
    let result = do_nothing!();
    println!("{:?}", result);
    assert_eq!(result, ());
}
