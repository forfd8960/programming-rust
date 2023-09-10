# Match

## match

```
Use ..= to match a whole range of values.

'0'..='9'
'a'..='z'


Use `|` to match multiple values

match v {
    ' ' | '\n' | '\t' => println!("white space"),
    ...
}
```
