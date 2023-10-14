// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref mut p) => {
            println!("Co-ordinates are {},{} ", p.x, p.y);
            p.x = 20;
        },
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
