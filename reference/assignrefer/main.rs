#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let x = 10;
    let y = 20;
    let r = &x;
    let s = &y;

    assert_eq!(*r, 10);
    assert_eq!(*s, 20);

    let point = Point { x: 1000, y: 100 };
    let p = &point;
    let pp = &p;
    let ppp = &pp;
    println!("ppp: {:?}", ppp);
    println!("x: {}, y: {}", ppp.x, ppp.y);
}
