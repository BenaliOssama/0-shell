

### ** Environment variables**

* Create a file at the root, e.g., `.env` or `config.rs` inside a crate.
* For Rust, using [`dotenv`](https://docs.rs/dotenv/latest/dotenv/) works nicely:

```rust
// in shell/src/main.rs
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok(); // load .env
    let path = env::var("PATH").unwrap_or_default();
    println!("PATH = {}", path);
}
```

* `.env` can store dev-specific variables, like paths to executables or temp directories.

---

✅ **Summary for your workflow**

* Keep one top-level `Cargo.lock`.
* Allow `main.rs` in each crate for dev testing.
* Separate builtins and standalone binaries inside `commands`.
* Use `.env` or a `config.rs` for environment variables.

