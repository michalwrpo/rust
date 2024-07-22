fn main() {
    let lyrics = ["A patridge in a pear tree", "Two turtle doves and", "Three french hens", "Four calling birds", 
                            "Five golden rings", "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", 
                            "Nine ladies dancing", "Ten lords a-leaping", "Eleven pipers piping", "Twelve drummers drumming"];

    let numbers = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let mut ind = 0;

    while ind < 12 {
        let number = numbers[ind];
        println!("On the {number} day of Christmas, my true love sent to me");
        
        for verse_number in (0..(ind+1)).rev() {
            let verse = lyrics[verse_number];
            println!("{verse}");
        }

        println!("");

        ind += 1;
    }
}
