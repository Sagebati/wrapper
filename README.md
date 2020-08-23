Wrapper
==

Tired of `Some(if {} else {})` or `Some(match{})` or equivalent for Result. Try this trait.

## Exemple
```rust
use wapprer::Wrap;
fn foo() -> Option<u32> {
    let x = 5;
    match x {
        0..=2 => 1,
        3..=5 => 0,
       _ => 2
    }.wrap()
}
```