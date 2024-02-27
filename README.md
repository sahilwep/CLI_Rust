# CLI_Rust

Rust Command line applications Walkthrough

[Resource](https://rust-cli.github.io/book/index.html)
[Git Repo](https://github.com/rust-cli/book)


## Rust Modules for CLI : 

* [clap](https://crates.io/crates/clap)
  * [docs](https://docs.rs/clap/latest/clap/)


## Project Setup : 

* We can use `cargo new grrs` command to setup new project.
  
## Parsing Command-line arguments:

* A typical invocation of our code tool will look like this:
```sh
$ grrs sahil names.txt
```
* we expect our program to look in `names.txt` and print out the lines that contains `sahil`.
* The text after the name of program is often called the `command-lien arguments` or `command-line flags` (specially when they look like `--this`).

### Getting the arguments: 

* The standard library contains the function `std::env::args()` that gives you an iterator of the given arguments. The first entry(at index `0`) will be the name your program was called as (eg `grrs`), the one that follow are what the user wrote afterwords.

* Getting the raw arguments this way is quit easy:

```rust
use std::env::args;

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    println!("pattern: {:?}, path: {:?}", pattern, path);

}
```
* output : 

```sh
sahilwep~$ cargo run -- some-pattern some-files
   Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/grrs some-pattern some-files`
pattern: "some-pattern", path: "some-files"
```


### CLI arguments as data type

* Instead of thinking about them as bunch of text, it often pays off to think to CLI arguments as a custom data type that represent the inputs to your program.

* In our program `grrs` we have two arguments, first is `pattern` and then `path`.

* While getting input from user:
  * The first argument is expected to be string.
  * The second argument is expected to be path.

* We can structure programs around the data they handle, so this way of looking at CLI arguments fits very well.

```rust
struct CLI {
  pattern: String,
  path: std::path::PathBuf,
}
```

* **NOTE :**  `PathBuf` is like a `String` but for the system path that work cross-platform.

```rust
struct CLI {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    let args = CLI {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
```
* This works, but not very convenient. We can't deal with the requirement to support `--pattern="foo"` or `--pattern "foo"` or `--help`.

### Parsing CLI arguments with Clap: