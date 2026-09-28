// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {//也可以用&y
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        //ref 就是“按引用绑定”的意思，有一个就行
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
