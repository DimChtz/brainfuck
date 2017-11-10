# rust-brainpreter (brainpreter) - v0.1.0
A simple and easy to use Brainfuck interpreter.

# How to use
#### Create a new brainpreter.
```rust
let mut bf = Inter::new();
```
#### Load brainfuck code from text string or file.
```rust
// For file use: .load_from_file()
match bf.load("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.") {

    Ok(_) => {}
	
    Err(e) => println!("{}", e),
	
}
```
#### Compile the bf code.
```rust
// For file use: .load_from_file()
match bf.parse() {
        
    Ok(_) => {}
	
    Err(e) => println!("{}", e),

}
```
#### Finally run it.
```rust
// For file use: .load_from_file()
match bf.run() {
        
    Ok(_) => {}
	
    Err(e) => println!("{}", e),

}
```
#### Result on the console.
```bash
Hello world!
```


# Installation

Add this line to your Cargo.toml:

```toml
[dependencies]
brainpreter = "0.1.0"
```

and then add this line to your main.rs:

```rust
extern crate brainpreter;
```
