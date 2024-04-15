# crabby

Rustaceans

https://doc.rust-lang.org/book/ch01-00-getting-started.html

update

```
rustup update
```

format and compile

```
rustfmt foo.rs
rustc foo.rs
```

cargo for dependency management

```
cargo --version
cargo new hello_cargo
cd hello_cargo && cargo build
./hello_cargo/target/debug/hello_cargo
```

or just build and run with one command

```
cargo run
```

check for errors

```
cargo check
```

build for release

```
cargo build --release
```

generate local docs and open in browser

```
cargo doc --open 
```