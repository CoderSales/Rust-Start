
```bash
cargo new hello_world
cd hello_world
tree .
tree /f
cargo build
./target/debug/hello_world
cargo run
cd ..
cargo new hello
cd hello
```

New file: /hello/src/hello.rs

```bash
cargo build
rustc src/hello.rs
./hello
cd ..
cargo new hello_world2 --bin --vcs none
cd hello_world2
tree /f
cargo build
./target/debug/hello_world2
cargo run
cargo build --release
cd ..
git clone https://github.com/rust-lang/regex.git
git remote -v
cd regex
git remote -v
cargo build
```
