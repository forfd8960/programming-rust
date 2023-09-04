#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let p1 = &Person {
        name: "alice".to_string(),
        age: 18,
    };
    let p2: &Person = &Person {
        name: "alex".to_string(),
        age: 22,
    };
    let p3: &Person = &Person {
        name: "bob".to_string(),
        age: 16,
    };

    let persons = vec![p1, p2, p3];
    let p = yongest_personal(&persons);

    println!("the yongest person: {:?}", p);
    assert_eq!(p.age, 16);
    assert_eq!(p.name, "bob".to_string());

    let p1 = oldest_personal(&persons);
    println!("the yongest person: {:?}", p1);
    assert_eq!(p1.age, 22);
    assert_eq!(p1.name, "alex".to_string());
}

fn yongest_personal<'a>(persons: &'a [&'a Person]) -> &'a Person {
    let mut yonger = persons[0];
    if persons.len() <= 1 {
        return yonger;
    }

    for p in &persons[1..] {
        if p.age < yonger.age {
            yonger = *p;
        }
    }

    return yonger;
}

fn oldest_personal<'a>(persons: &'a [&'a Person]) -> &'a Person {
    let mut older = persons[0];
    if persons.len() <= 1 {
        return older;
    }

    for p in &persons[1..] {
        if p.age > older.age {
            older = *p;
        }
    }

    return older;
}
