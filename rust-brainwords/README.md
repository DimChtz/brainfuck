
# rust-brainwords (brainwords) - v0.1.0
A simple and easy to use Brainfuck code generator in Rust.

# How to use (Hello world! Example)
```rust
println!("{}", brainwords::generate_bfcode("Hello world!"));
```

#### Result on the console.
```bash
++++++++[>++++++++<-]>++++++++.<+++++[>+++++<-]>++++.<++[>++<-]>+++.<[><-]>.<+[>+<-]>++.<++++++++[>--------<-]>---------------.<
+++++++++[>+++++++++<-]>++++++.<++[>--<-]>----.<+[>+<-]>++.<++[>--<-]>--.<++[>--<-]>----.<++++++++[>--------<-]>---.
```


# Installation

Add this line to your Cargo.toml:

```toml
[dependencies]
brainwords = "0.1.0"
```

and then add this line to your main.rs:

```rust
extern crate brainwords;
```
