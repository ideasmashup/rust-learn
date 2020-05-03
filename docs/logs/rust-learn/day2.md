### Day 2: the usual bits are boring

[Back to log main page](../rustlang-learn.md)

#### Recap

> Duration: 16 hours
> Total projects: 5
> Total lines of code: ~383
> Total lines of wiki: ~140

**What I learned**

- [ ] literals, scalar types, compounds, tuples, arrays
- [ ] memory sizes of these types
- [ ] defaults types for typical init values infered by compiler
- [ ] personal unsupervised foray into basic usage of ```str```
- [ ] expressions, statements, functions, conditions, branching, loops
- [ ] expressions never end with a trailing ```;``` (it turns the line into a statement)

**What I need to improve**

- [ ] still veering-off too much delving into language details not covered in the course
- [ ] just follow the course blindly, do the exercises after as training (maybe better ?)

**Notes to self**

- [ ] UTF-8 native Rust support is amazing but no shell on Windows capable of displaying the characters correctly (maybe dev a cross-platform shell in rust ? like fish or zsh -- mmh: https://www.nushell.sh/blog/2019/08/23/introducing-nushell.html)
- [ ] type inference doesn't take values' encoding bits size into account (e.g. ```let value = 8;``` and ```let value = 65535;``` both produced a ```i32``` also ```let value = 170141183460469231731687303715884105727;``` doesn't compile unless you explicitly state ```let value: i128 = 170141183460469231731687303715884105727;``` or ```let value = 170141183460469231731687303715884105727i128;```)
- [ ] I want to make a cross-platform interactive rust learning app (like scrimba, but running locally and fully open-source to allow contributions, localization and comments from the whole community)
- [ ] unexpected variations when benchmarking loops, need to do more benchmarks and investigate causes of variation (Arch, OS, VM impact, ```std::time::{Instant, Duration}``` accuracy)
- [ ] don't forget to do the Chapter exercises!!

**Extras I read and useful stuff**

- https://doc.rust-lang.org/std/num/struct.Wrapping.html
- https://stackoverflow.com/questions/1186535/how-to-modify-a-specified-commit
- https://doc.rust-lang.org/book/appendix-02-operators.html
- 

#### Tasks / projects

##### Scalar types and compounds

Learn primitive types, integers, floats, boolean, char, tuples and arrays and basic operators.

> Good
> - covered most useful types
> - char type with native UTF-8 made me excited!

> Bad
> - no lower-level details (e.g. size of types in memory)
> - where are the strings?! why not say anything about these?! (I spend a heck of a time compensating this omission on my own)

```rust
let value = '好';

let value = 123_456_789u128;
let value: i32 = i32::MIN;

let value = [7;3];
let value = [i32; 7, 7, 7];
let value: i32 = [7, 7, 7];

let value = (1, 2, 3);
let (x, y, z) = value;
let value: (u8, u16, u32) = (1, 2, 3);

println!("x = {}, y = {}, z = {}", x, value.1, another.2);
```

##### Functions, statements

Super basic functions and gotchas.

> Good
> - very straightfoward

> Bad
> - super basic stuff
> - would have liked some error management in here

**TIL stuff like this:**

```rust
fn dad() {
    println!("I am your father!");
}

// works as intended
fn add(i32: a, i32: b) -> i32 {
    a + b
}

// wrong (missing return)
fn add(i32: a, i32: b) -> i32 {
    a + b;
}
```

##### Control Flow

Conditions and branching nothing else.

> Good
> - very quick !

> Bad
> - can't wait to get back to ```match``` instead of all that ```if else``` horror

**TIL stuff like this:**

```rust
// weird but possible
fn positive_additude(i32: a, i32: b) -> u32 {
    if a > 0 {a} else {a * -1} + if b > 0 {b} else {b * -1}
}

```

##### Loops

All the loop types: loop, while, for.

> Good
> - quick as usual

> Bad
> - not sure (skeptic about ```for``` only working on ranges and collections, it's misleading and should be introduced as ```for in``` instead)

**TIL stuff like this:**

```rust
let mut init = 0;

// loop that returns a result! o_O
let give_me_five = loop {
    init += 1;

    if init == 5 {
        break init;
    }
}
println!("This is a five: {}", give_me_five);
```

[Back to log main page](../rustlang-learn.md)