# Day 1: dived but wandered too deep

[Back to log main page](../rustlang-learn.md)

## Recap

> Duration: 8 hours
> Total projects: 3
> Total lines of code: ~43
> Total lines of wiki: ~160

**What I learned**

- [x] ```Option```, ```Result```, ```Ok```, ```Err```, ```match```, ```String```, ```loop```
- [x] shadowing of variables
- [x] random numbers generation
- [x] user input handling with ```io::stdin::read_line(&mut str_buffer)```
- [x] range matching and many corner cases for code sample in "guessing game" section

**What I need to improve**

- [x] code without Internet nor doc (because no completion)
- [ ] deeper knowledge about macros, types compiling, sizes, inference rules
- [ ] stop digging too deep (too time consuming), just follow the course!

**Notes to self**

- [ ] learn how to edit cargo default template (that default Hello World stub is sooo boring !)
- [ ] learn rustdoc to make clean doc
- [ ] learn unit tests


**Extras I read and useful stuff**

- https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
- https://doc.rust-lang.org/std/result/
- https://www.reddit.com/r/rust/comments/48fmta/seven_ways_to_concatenate_strings_in_rust_the/
- https://github.com/Vurich/const-concat
- https://github.com/hoodie/concatenation_benchmarks-rs
- https://users.rust-lang.org/t/concatenate-two-static-str/33993

<br>

## Tasks / projects

### Hello World !

Learn how to write a hello world rust program. Compile it with rustc.

> Good
> - super easy and straigth forward
> - I tried it for fun on Windows, Mac, Linux it's a breeze
> - compiler is super nice and informative (feels like meeting git "Did you mean?" for the first time again haha)

> Bad
> - nothing, but println! being a macro without further details or links for deeper infos makes me sad and uneasy


```bash
$ mkdir hello_world
$ cd hello_world

# this works on Windows in cmder o_O
$ echo fn main() { println!("Hello World"); } > main.rs

$ rustc main.rs
$ .\main.exe
```

### Hello cargo !

Learn the basics of cargo (package manager, dependencies and build tool swiss army knife for rust).

> Good
> - quickly learn how to use ```cargo``` (instead of rustc)
> - ready to scale-up to build bigger projects and dependencies
> - learn cargo TOML file syntax
> - ```cargo new <project>``` scaffolds clean git-ready dirs and files (love it !)

> Bad
> - still very basic example

**TIL stuff like this:**
```bash
$ cargo new hello_cargo
$ cd hello_cargo
$ cargo check
$ cargo build
$ cargo build --release
$ cargo run
```

### Guessing game

Quick summary of project or task

> Good
> - lots of new concepts and instructions learned
> - a lot of the things are functional / high-level thus unexpected for system programming
> - cargo and compiler makes everything more pleasant
> - tried to stress error handling and inputs and it felt strong
>
> Bad
> - not having full code completion feels like I'm back to the 90s (had forgotten the feeling haha)
> - tried to do more things than asked for and fell in many traps with compilator inferences, types casting...
> - couldn't concatenate ```const``` with ```format!()``` and ```concat!()``` so I felt stupid spending too much time figuring some workaround

**TIL stuff like this:**

```rust
// user safe input guarding and checking
use std::io;

loop {
    let mut input = String::new();
    println!("\nType a \"good\" value:");
    io::stdin().read_line(&mut input)
        .expect("Can't read your input");

    let _value = match input.trim().parse::<i32>() {
        Ok(v) => match v {
            0..=10 => {
                println!("{:?} is a correct value!", v);
            },
            _ => {
                println!("Not in 0 to 10 included range!");
                continue;
            }
        },
        Err(e) => {
            println!("Not a 32bits integer! Error -> {:?}", e);
            continue;
        }
    };

    println!("Got a good unsigned integer within expected range, exiting happily!");
    return;
}

```
```
$ cargo run

Type a "good" value:
qskudgqsd
Not a 32bits integer! Error -> ParseIntError { kind: InvalidDigit }

Type a "good" value:

Not a 32bits integer! Error -> ParseIntError { kind: Empty }

Type a "good" value:
123345679876856353576458759597898
Not a 32bits integer! Error -> ParseIntError { kind: Overflow }

Type a "good" value:
-2325835283523
Not a 32bits integer! Error -> ParseIntError { kind: Underflow }

Type a "good" value:
-2
Not in 0 to 10 included range!

Type a "good" value:
5
5 is a correct value!
Got a good unsigned integer within expected range, exiting happily!
```

[Back to log main page](../rustlang-learn.md)