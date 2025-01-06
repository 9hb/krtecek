# HTMoLe (formerly known as Krtek)

Library for searching HTML elements using ID, tags, and class in HTML pages.

## Usage

```rust
use krtek::Krtecek;

let krtecek = Krtecek::new("https://example.com")
    .find_id("username")
    .find_tag("h1")
    .find_class("container");
```

## Example

![Code example](https://raw.githubusercontent.com/9hb/krtecek/refs/heads/main/assets/showcase-code-2.png)

![Compilation result](https://raw.githubusercontent.com/9hb/krtecek/refs/heads/main/assets/showcase-terminal-2.png)
