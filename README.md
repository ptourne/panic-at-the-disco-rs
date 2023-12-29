# `panic!("at the disco")` - Rust

This crate is a simple panic handler that plays the song "Hey Look Ma, I Made It!" and prints `🎩 Hey Look Ma, I Made It! 🕺` to the console before panicking.

This is a meme crate and should not be used in production.

# Usage

To use it, you can add it to your dependencies in your `Cargo.toml` file.

`Cargo.toml`

```toml
/* ... */

[dependencies]
panic-at-the-disco-rs = { path = "../panic-at-the-disco-rs/"}
```

`main.rs`

```
use panic_at_the_disco_rs::panic;
fn main() {
    panic!("Oh no!");
}
```

# Repo

This crate is available on Github at [https://github.com/ptourne/panic-at-the-disco-rs](https://github.com/ptourne/panic-at-the-disco-rs)

# Disclaimer

This crate is not associated by any means with the producers, publishing label,
writers, or artists of the song "Hey Look Ma, I Made It!" by Panic! at the Disco.
All rights to the song belong to their respective owners.
