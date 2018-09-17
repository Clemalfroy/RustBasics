fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let lyrics = [
        "a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-layin'",
        "Seven swans a-swimmin'",
        "Eight maids a-milkin'",
        "Nine lords a-leapin'",
        "Ten ladies dancin'",
        "Eleven pipers pipin'",
        "Twelve drummers drummin'"
        ];

    for index in 0..days.len() {
        println!("On the {} of Christmas, my true love gave to me", days[index]);
        for nb in (0..index+1).rev() {
            if nb == 0 && index == days.len() - 1 {
                println!("And {}", lyrics[nb]);
            } else if index != 0 && nb == 0 {
                println!("And {},\n", lyrics[nb]);
            }
            else {
                println!("{},", lyrics[nb]);
            }
        }
    }
}