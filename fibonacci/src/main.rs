fn sing() {
    let ordinals = ["1st", "2nd", "3rd", "4th", "5th", "6th", "7th", "8th", "9th", "10th", "11th", "12th"];
    let lyrics = [
        "Twelve drummers drumming",
	"Eleven pipers piping",
	"Ten lords a-leaping",
	"Nine ladies dancing",
	"Eight maids a-milking",
	"Seven swans a-swimming",
	"Six geese a-laying",
	"Five golden rings",
	"Four calling birds",
	"Three French hens",
	"Two turtle doves",
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas,my true love sent to me", ordinals[i]);
	for j in (11-i)..11 {
	    println!("{}", lyrics[j]);
	}
	if i == 0 {
	    println!("a partridge in a pear tree");
	} else {
	    println!("and a partridge in a pear tree");
        }
    }
}

fn fibonacci(i: i32) -> i32 {
    if i == 0 {
        0
    } else if i<=2 {
        1
    } else {
        fibonacci(i-1) + fibonacci(i-2)
    }
}

fn main() {
    for i in 0..10 {
        println!("{}:{}", i, fibonacci(i));
    }

    sing()
}
