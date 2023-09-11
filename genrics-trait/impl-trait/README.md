## Implment Trait

```rust
use std::io::{self, Write};

/// Trait for values to which you can send HTML.
trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}


/// You can write HTML to any std::io writer.
impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        ...
    }
}

```

## Type-Associated Functions

```rust
trait StringSet<'a> {
    fn new() -> Self;
    fn from_slice(strings: &'a [&str]) -> Self;
    ...
}

// S::new is a associated function
fn unknow_words<'a, S: StringSet<'a>>(document: &'a [String], word_list: &S) -> S {
    let mut unknows = S::new();
    for word in document {
        if !word_list.contains(word) {
            unknows.add(word);
        }
    }
    unknows
}

```
