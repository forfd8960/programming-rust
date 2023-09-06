use std::fmt;

#[derive(Debug, Clone)]
struct JsonError {
    message: String,
    line: usize,
    column: usize,
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}: {})", self.message, self.line, self.column)
    }
}

impl std::error::Error for JsonError {}

fn main() {
    /*
    json res: true
    json: true
    */
    let res = call_ret_json(false).expect("json is invalid");
    println!("json: {}", res);

    /*
    thread 'main' panicked at 'json is invalid: JsonError { message: "bad input", line: 1, column: 1 }', custom-error/main.rs:26:36
    */
    let res1 = call_ret_json(true);
    match res1 {
        Err(e) => {
            // error: bad input (1: 1)
            println!("error: {}", e);
        }
        Ok(v) => {
            println!("{}", v);
        }
    }
    // println!("json: {:?}", res1.err());
}

fn call_ret_json(is_error: bool) -> Result<bool, JsonError> {
    let success = ret_json_error(is_error);
    let res = success?;
    println!("json res: {}", res);
    Ok(res)
}

fn ret_json_error(is_err: bool) -> Result<bool, JsonError> {
    if is_err {
        return Err(JsonError {
            message: "bad input".to_string(),
            line: 1,
            column: 1,
        });
    }

    return Ok(true);
}
