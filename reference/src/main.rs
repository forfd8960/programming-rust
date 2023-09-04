use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn main() {
    let mut table = Table::new();
    table.insert(
        "ar1".to_string(),
        vec!["wk11".to_string(), "w12".to_string()],
    );
    table.insert(
        "ar2".to_string(),
        vec!["wk21".to_string(), "w22".to_string()],
    );
    table.insert(
        "ar3".to_string(),
        vec!["wk31".to_string(), "w32".to_string()],
    );

    // So when the program calls show(table), the whole structure gets moved to the function
    // leaving the variable table uninitialized
    show(table);

    // assert_eq!(table["ar1"][0], "wk11");
    /*
     assert_eq!(table["ar1"][0], "wk11");
    |                ^^^^^ value borrowed here after move
     */
}

fn show(tb: Table) {
    for (artist, works) in tb {
        println!("works by {}: ", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}
