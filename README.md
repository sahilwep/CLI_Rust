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

* We can use one of the many available library to parse our CLI arguments. One of the popular is `clap`. It has all the functionality including support of `sub-commands`, shell completions, and great help message.

* For importing `clap` we need to add `clap = {version = "4.0", features = ["derive"]}` to the dependencies section of our `Cargo.toml` file.
* Now, we can write `use clap::Parser` in our code, and `#[derive(parser)]` right above our `struct CLI`

```rust
use clap::Parser;

// Search for a pattern in a file and display the lines that contains it.
struct CLI {
    // pattern to look for
    pattern: String,
    // The path to the file to read.
    path: std::path::PathBuf,
}
```
* **Note :** There are a lot custom attributes you can add to field. For example, to say you want to use this field for the argument after `-o` or `--output`, you'd add `#[arg(short = 'o', long = "output")]`. [clap Documentation](https://docs.rs/clap/)

* Right below the `CLI` struct our template contains its `main` function. WHen the program starts, it will call this function.

```rust
fn main(){
  let args = Cli::parse();

  println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
```

* This will try to parse the arguments into our `CLI` struct.
* But what if that fails? That's the beauty of this approach: Clap knows which field to expect, and what their expected format is. It can automatically generate a nice `--help` message, as well as give some great errors to suggest you pass `--output` when you wrote `--putput`.

* **NOTE :** The `parse` method is meant to be used in your `main` function. When it fails, it will print out an error or help message and immediately exit the program. Don't user it in other places!.

* Our code will look something like this : 

```rust
use clap::Parser;

// Search for a pattern in a file and display the lines that contains it.
#[derive(Parser)]
struct CLI {
    // pattern to look for
    pattern: String,
    // The path to the file to read.
    path: std::path::PathBuf,
}

fn main() {
    let args = CLI::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
}
```

* Output : 

```sh
sahilwep~$ cargo run -- pattern-some path-some
   Compiling grrs v0.1.0 (/Users/sahilwep/Developer/Development/Rust/CLI_Rust/grrs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/grrs pattern-some path-some`
pattern: "pattern-some", path: "path-some"
```

## First implementation of *grrs*

