use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum JSON {
    Null,
    Boolean(bool),
    Number(i64),
    String(String),
    Array(Vec<JSON>),
    Object(Box<HashMap<String, JSON>>),
}

macro_rules! json {
    (null) => {
        JSON::Null
    };
}

fn main() {
    println!("Hello, world!");
    let students = JSON::Array(vec![JSON::Object(Box::new(
        vec![
            ("name".to_string(), JSON::String("Alex".to_string())),
            ("class".to_string(), JSON::Number(10)),
            (
                "major".to_string(),
                JSON::String("Computer Science".to_string()),
            ),
        ]
        .into_iter()
        .collect(),
    ))]);
    println!("json: {:?}", students);

    /*

    let students = json!([
        {
            "name": "Alex",
            "class": 10,
            "major": "Computer Scienece"
        }
    ])
    */

    // json null: Null
    println!("json null: {:?}", json!(null));
}
