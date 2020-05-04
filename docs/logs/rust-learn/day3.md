### Day 3: low-level programming is back!

#### Recap

> Duration: 6 hours
> Total projects: 3
> Total lines of code: ~155
> Total lines of wiki: ~190

**What I learned**

- [ ] "trivial" things like strings handling, iterating on strings chars
- [ ] strong reminders of the true complexity of handling UTF-8 strings correctly
- [ ] using map, collect (although I didn't use them in the end)
- [ ] input filtering and validation
- [ ] faced the reality that rust is low-level, it's not Golang, it's not Python

**What I need to improve**

- [ ] remember that rust is low-level and focused on "correctness"
- [ ] knowledge of language to be faster at coding "simple" things
- [ ] remember that debug is different than release!!! (panics may change a bit)
- [ ] iterate in extreme programmin style (small incremental steps, continuous testing)
- [ ] related to above, commit often (atomic commits) instead of only at the end
- [ ] frustration with standard library (kinda like a microkernel design choice) provides very little of the "day to day" ground work and feature set (feels terrible to rely on crates for "Tier 1" basics", the "batteries included" philosophy of Python is more prevalent these days for any modern language)

**Notes to self**

- [ ] unit tests would have been good for the fibonacci exercise (u128 integers overflow)
- [ ] rust really feels "robust", it's (very) hard to do "wrong" things
- [ ] ASCII is not UTF-8 (designing for the later is harder, but clean and more future-proof)

**Extras I read and useful stuff**

- https://doc.rust-lang.org/std/string/struct.String.html#method.matches
- https://www.reddit.com/r/rust/comments/30c6yb/why_is_accessing_a_char_from_a_str_so_annoying/
- https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_alphanumeric
- http://danielnill.com/rust_tip_compairing_strings
- 

#### Tasks / projects

##### Temperature converter

Convert temperatures from °C (Celsius) into °K (Kelvins) and vice-versa.

> Good
> - simple problem
> - very clean code with ```match```
> - happy that there is a conditional match possibility albeit weird (```match value { x if x < 30 => {/* conditional arm */}}```)
> - program is super safe

> Bad
> - had to resist the urge to rely on regex for validation and filtering (way overkill, but basic string operations made me very frustrated)

```rust
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Couldn't read user input!");

// clean user input
let mut value = String::new();
let mut unit = '°';

for c in input.chars() {// single-pass cleaning of all input chars
    match c {
        x if x.is_numeric() || x == '.' => {
            value.push(c);
        },
        'C' => {
            unit = 'C';
        },
        'K' => {
            unit = 'K';
        }
        _ => {/* invalid char, skip */}
    }
}
```

##### Fibonacci

Generate the Nth Fibonacci term

> Good
> - good exercise as in other languages to implement a "known" algorithm
> - good to verify proper types usage and optimizing
> - would have done a recursive function (I didn't haha)
> - refreshed a few things since I had never had to implement that in years haha
> - reminded me of the overflow corner-case

> Bad
> - not enough error management (overflow) knowledge yet to make robust code
> - had to ```println!``` a warning for N above 185 to warn of overflows

**TIL stuff like this:**

```rust
fn fibonacci(n: u128) -> u128 {
    let mut prev: u128 = 0;
    let mut next: u128 = 1;
    let mut current: u128 = 1;

    if n < 1 {
        return 0
    }
    else if n == 1 {
        return 1;
    }
    else {
        for _i in 2..=n {
            current = prev + next;
            prev = next;
            next = current;
        }
    }
    
    current
}
```

##### Twelve Days of Christmas

Generate the famous song's lyrics because and exploit the repetitiveness. Not as simple/smooth as I thought it would have been beforehand.

> Good
> - Great exercise to apply all concepts learned so far
> - learned to consider String as a StringBuffer from Java rather than regular String (much better for my brain)
> - feels like I'm a C programmer implementing everything from scratch (frustrating, but I always wanted to live my own "42sh" experience so I guess this is just karma at work haha - it's slow yet I actually enjoy having a C-like adventure in modern times quite a lot !)

> Bad
> - &str and String are needed to implement this well but these weren't covered enough in depth so far, lot of catch-up to do
> - having to implement a ```capitalize()``` function (see. code example below) feels strange (I don't know enough of the language internals as do all beginners so it's impossible to implement basic things like this correctly without too huge effort for the pay-off)

**TIL stuff like this:**

```rust
// could have never implemented that this well
// by: Shepmaster @ https://stackoverflow.com/questions/38406793/why-is-capitalizing-the-first-letter-of-a-string-so-convoluted-in-rust
fn uppercase_first_letter(string: &str) -> String {
    let mut letters = string.chars();
    match letters.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + letters.as_str(), // as_str() more optimal by using memcpy for remaining letters
    }
}

fn main() {
    let holiday = "christmas";
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "twelfth"];
    let counts = ["a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve"];
    let gifter = "my true love";
    let gifts = [
        "partridge in a pear tree", "turtle doves", "French hens", "calling birds",
        "gold rings", "geese a laying", "swans a swimming", "maids a milking",
        "ladies dancing", "lords a leaping", "pipers piping", "drummers drumming"];

    for day in 0..12 {
        println!("\nOn the {} day of {} {} gave to me", days[day], uppercase_first_letter(holiday), gifter);

        let mut chorus = String::new();
        for index in (0..=day).rev() {
            if day == 0 {
                // chorus of first day
                chorus.push_str(&format!("{} {}", counts[index], gifts[index]));
            }
            else if index == day {
                // first item: append to chorus (in reverse - e.g. prepend) without prefix
                chorus.push_str(&format!("{} {}", counts[index], gifts[index]));

                if day > 1 {
                    chorus.push_str(", ");
                }
            }
            else if index == 0 {
                // last item: prefix " and " then append to chorus (in reverse - e.g. prepend)
                chorus.push_str(&format!(" and {} {}", counts[index], gifts[index]));
            }
            else {
                // intermediary item: append to chorus (in reverse - e.g. prepend)
                if index % 2 == 0 {
                    // even items get a new line at the end
                    chorus.push_str(&format!("{} {},\n", counts[index], gifts[index]));
                }
                else {
                    // odd items are on new lines so capitalize them
                    chorus.push_str(&uppercase_first_letter(&format!("{} {}", counts[index], gifts[index])));

                    if index > 1 {// add trailing ", " before next item unless penultimate item
                        chorus.push_str(", ");
                    }
                }
            }
        }
        
        println!("{}.", uppercase_first_letter(&chorus.as_str()));
    }
}
```
