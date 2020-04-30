# Day 1 : this day in 5 words

[Back to log main page](../rustlang-learn.md)

## Recap

- **Goal**
    - ...
- **Results**
    - ...

> Duration: 0 hours
> Total projects: 0
> Total lines of code: 0

**What I learned**

- [ ] ...
- [ ]

**What I need to improve**

- [ ] ...
- [ ]

**Notes to self**

- ...

**Useful stuff**

- ...

## Tasks / projects

### Task / project 1

Learn how to write a hello world rust program. Compile it with rustc.

> Good :D
> - super easy and straigth forward
> - I tried it for fun on Windows, Mac, Linux it's a breeze
> - compiler is super nice and informative (feels like meeting git "Did you mean?" for the first time again haha)

> Bad :/
> - nothing


```bash
$ mkdir hello_world
$ cd hello_world

# this works on Windows in cmder o_O
$ echo fn main() { println!("Hello World"); } > main.rs

$ rustc main.rs
$ .\main.exe
```

### Task / project 2

Learn the basics of cargo (package manager, dependencies and build tool swiss army knife for rust).

> Good :D
> - quickly learn how to use ```cargo``` (instead of rustc)
> - ready to scale-up to build bigger projects and dependencies
> - learn cargo TOML file syntax
> - ```cargo new <project>``` scaffolds clean git-ready dirs and files (love it !)

> Bad :/
> - still very basic example

> Notes to self:
> - learn how to edit cargo default template (that default Hello World stub is sooo boring !)
> - learn rustdoc to make clean doc
> - learn unit tests

```bash
$ cargo new hello_cargo
$ cd hello_cargo
$ cargo check
$ cargo build
$ cargo build --release
$ cargo run
```

[Back to log main page](../rustlang-learn.md)