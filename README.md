# wrapping_macros

A macro for scoped wrapping arithmetic.

Any code within a `wrapping! { .. }` block will be transformed as follows:

* `a + b` becomes `a.wrapping_add(b)`. Similarly for `-`, `*`, `/`, `%`, `<<`, `>>`.
* `a += b` becomes `a = a.wrapping_add(b)`. Similarly for `-=`, `*=`, `/=`, `%=`, `<<=`, `>>=`.
* `-a` becomes `a.wrapping_neg()`.

## Cargo

Add this to your `Cargo.toml`:

```toml
wrapping_macros = "*"
```

## Example

```rust
use wrapping_macros::wrapping;

fn main() {
    let mut sum = 0u8;
    for x in 0u8..50 {
        wrapping! {
            sum += x;
        }
    }
}
```

*Inspired by [`wrapping_macros`](https://github.com/lambda-fairy/wrapping_macros) crate by lfairy*
