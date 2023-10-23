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
    ([$ ($element:tt),*]) => {
        JSON::Array(vec![$ ( json!($element) ),*])
    };
    ({$ ($key:tt : $value:tt),*}) => {
        JSON::Object(Box::new(vec![
            $( ($key.to_string(), json!($value)) ),*
        ].into_iter().collect()))
    };
    ( $other:tt )  => {
        JSON::from($other)
    };
}

impl From<bool> for JSON {
    fn from(value: bool) -> Self {
        JSON::Boolean(value)
    }
}

impl From<i32> for JSON {
    fn from(value: i32) -> Self {
        JSON::Number(value as i64)
    }
}

impl From<String> for JSON {
    fn from(value: String) -> Self {
        JSON::String(value)
    }
}

impl<'a> From<&'a str> for JSON {
    fn from(value: &'a str) -> Self {
        JSON::String(value.to_string())
    }
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::JSON;

    #[test]
    fn json_array_with_element() {
        let macro_value = json!([{"pitch": 440}]);

        let mut obj = HashMap::new();
        obj.insert("pitch".to_string(), JSON::Number(440));
        let expect = JSON::Array(vec![JSON::Object(Box::new(obj))]);
        assert_eq!(macro_value, expect);
    }

    #[test]
    fn json_array_with_element1() {
        let macro_value = json!(["abc", 123, true, false, null]);

        let expect = JSON::Array(vec![
            JSON::String("abc".to_string()),
            JSON::Number(123),
            JSON::Boolean(true),
            JSON::Boolean(false),
            JSON::Null,
        ]);
        assert_eq!(macro_value, expect);
    }
}
