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

## Examples

```rust
let a = 128u8;
let b = wrapping! { a * 2 };

assert_eq!(b, 0);
```

```rust
let mut a = 250u8;
let mut b = 4u8;
let mut c = 100u8;
wrapping! {
    a += 10;
    b -= 10;
    c *= 10;
}

assert_eq!(a, 4);
assert_eq!(b, 250);
assert_eq!(c, 232);
```

```rust
use wrapping_macros::wrapping;

fn main() {
    let mut sum = 0u8;
    wrapping! {
        for x in 0u8..50 {
            sum += x;
        }
    }
}
assert_eq!(sum, 201);
```

## Caveat

You cannot nest another macro invocation within a `wrapping!` block. For example, this will not work:

```rust
let x = 128u8;
wrapping! {
    println!("The answer is {}", x + 128) // Error
}
```

Instead, move the macro call out of the `wrapping!` block:

```rust
let x = 128u8;
println!("The answer is {}", wrapping! { x + 128 })
````

*Inspired by [`wrapping_macros`]("https://github.com/lambda-fairy/wrapping_macros") crate by lfairy*
