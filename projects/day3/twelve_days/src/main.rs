// Twelve days of Christmas song generator

/* ORIGINAL LYRICS

On the first day of Christmas my true love gave to me
A partridge in a pear tree

On the second day of Christmas my true love gave to me
Two turtle doves and a partridge in a pear tree

On the third day of Christmas my true love gave to me
Three French hens, two turtle doves and a partridge in a pear tree

On the fourth day of Christmas my true love gave to me
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the fifth day of Christmas my true love gave to me
Five gold rings, four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the sixth day of Christmas my true love gave to me
Six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the seventh day of Christmas my true love gave to me
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the eighth day of Christmas my true love gave to me
Eight maids a milking, seven swans a swimming
Six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the ninth day of Christmas
Nine ladies dancing, eight maids a milking
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the tenth day of Christmas my true love gave to me
Ten lords a leaping, nine ladies dancing, eight maids a milking
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the eleventh day of Christmas my true love gave to me
Eleven pipers piping, ten lords a leaping
Nine ladies dancing, eight maids a milking
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

On the twelfth day of Christmas my true love gave to me
Twelve drummers drumming, eleven pipers piping
Ten lords a leaping, nine ladies dancing, eight maids a milking
Seven swans a swimming, six geese a laying, five gold rings
Four calling birds, three French hens
Two turtle doves and a partridge in a pear tree

*/

fn uppercase_first_letter(string: &str) -> String {
    let mut letters = string.chars();
    match letters.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + letters.as_str(), // as_str() more optimal by using memcpy for remaining letters
    }
}

fn main() {
    let holiday = "christmas";
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth", "eleventh", "thwelfth"];
    let counts = ["a", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve"];
    let gifter = "my true love";
    let gifts = [
        "partridge in a pear tree", "turtle doves", "French hens", "calling birds",
        "gold rings", "geese a laying", "swans a swimming", "maids a milking",
        "ladies dancing", "lords a leaping", "pipers piping", "drummers drumming"];

    println!("TWELVE DAYS OF {}", holiday.to_uppercase());
    
    // Reminder: full chorus for last day
    /*
    On the twelfth day of Christmas my true love gave to me
    Twelve drummers drumming, eleven pipers piping
    Ten lords a leaping, nine ladies dancing, eight maids a milking
    Seven swans a swimming, six geese a laying, five gold rings
    Four calling birds, three French hens
    Two turtle doves and a partridge in a pear tree
    */
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
