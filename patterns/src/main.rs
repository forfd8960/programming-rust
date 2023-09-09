fn main() {
    println!("Hello, world!");
    match_literal();

    match_array([1, 2, 3]);
    match_array([0, 0, 0]);

    match_slice(vec!["a", "B", "C"].as_slice());
    match_slice(vec![].as_slice());
    match_slice(vec!["B", "C"].as_slice());

    match_tuple((10, 10));

    match_struct(MyData {
        name: "HiHiHI".to_string(),
        lines: vec![],
    });
    match_struct_ref(MyData {
        name: "HiHiHI".to_string(),
        lines: vec!["ABC".to_string()],
    });

    let mut d = MyData {
        name: "HiHiHI".to_string(),
        lines: vec!["ABC".to_string()],
    };
    match_struct_ref_mut(&mut d);
}

fn match_literal() {
    let n = 10;
    match n {
        0 => println!("zero"),
        1 => println!("one"),
        n => println!("get: {}", n),
    }
}

fn match_array(array: [u8; 3]) {
    match array {
        [_, _, 0] => println!("ends with zero"),
        [_, _, 255] => println!("Hello: 255"),
        [a, b, c] => println!("Hello: {}, {}, {}", a, b, c),
    }
}

fn match_slice(names: &[&str]) {
    match names {
        [] => println!("Hello no body"),
        [a] => println!("Hello: {}", a),
        [a, b] => println!("Hello: {}, {}", a, b),
        [a, .., b] => println!("Hello from: {}, to: {}", a, b),
    }
}

fn match_tuple(tp: (u8, u8)) {
    match tp {
        (0, 0) => println!("tuple(0, 0)"),
        (x, y) => println!("tuple({}, {})", x, y),
    }
}

struct MyData {
    name: String,
    lines: Vec<String>,
}

fn match_struct(d: MyData) {
    match d {
        MyData { name, .. } => println!("name: {}", name),
    }
}

fn match_struct_ref(d: MyData) {
    match d {
        MyData {
            name, ref lines, ..
        } => {
            println!("name: {}", name);
            println_lines(lines);
            println!("d.lines: {:?}", d.lines);
        }
    }
}

fn match_struct_ref_mut(d: &mut MyData) {
    match d {
        MyData {
            name,
            ref mut lines,
            ..
        } => {
            println!("name: {}", name);
            lines.push("DEF".to_string());
            println!("d.lines: {:?}", d.lines);
        }
    }
}

fn println_lines(lines: &Vec<String>) {
    println!("{:?}", lines);
}
