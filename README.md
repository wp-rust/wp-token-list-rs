# wp-token-list-rs

Rust port of [`@wordpress/token-list`](https://github.com/WordPress/gutenberg/tree/trunk/packages/token-list) from the Gutenberg project.

A `DOMTokenList`-like CSS class token set for managing Gutenberg block `className` attributes. Provides ordered, deduplicated class name management with an API that mirrors the browser's `classList`.

## Installation

```toml
[dependencies]
wp-token-list-rs = "0.1"
```

## Usage

```rust
use wp_token_list_rs::TokenList;

let mut classes = TokenList::new("wp-block-group");
classes.add("alignwide");
classes.add("has-background");
classes.remove("alignwide");

assert!(classes.contains("wp-block-group"));
assert!(classes.contains("has-background"));
assert!(!classes.contains("alignwide"));

println!("{}", classes); // "wp-block-group has-background"
```

## Features

- `add(token)` / `remove(token)` / `toggle(token)` / `contains(token)`
- Preserves insertion order, no duplicates
- `Display` impl produces a space-separated class string
- Zero dependencies — pure `std`

## Related Crates

| Crate | Purpose |
|---|---|
| [`wp-escape-html-rs`](https://crates.io/crates/wp-escape-html-rs) | Sanitize class names and attribute values |
| [`wp-block-parser-rs`](https://crates.io/crates/wp-block-parser-rs) | Parse Gutenberg block markup |

## License

GPL-2.0-or-later — consistent with the [Gutenberg project](https://github.com/WordPress/gutenberg).
