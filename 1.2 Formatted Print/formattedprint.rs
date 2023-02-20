fn main() {

    // print formatted string
    println!("{} days", 31);

    // print without new line
    print!("this is some text");
    println!(", this is text with new line");

    // formatted string
    //format!("");

    // print error without new line
    eprint!("this is some error text");
    eprintln!(", this is error text with a new line");

    // positional formatted print
    println!("{0}, this is {1}. {1}, this is {0}", "alice", "bob");

    // named formatted print
    println!("{subject} {verb} {object}",
        object="the lazy dog",
        subject="the quick brown fox",
        verb="jumps over");

    // number formatting
    println!("base 10 (decimal):    {}", 69420);
    println!("base  2 (binary):     {:b}", 69420);
    println!("base  8 (octal):      {:o}", 69420);
    println!("base 16 (hex):        {:x}", 69420);
    println!("base 16 (hex):        {:X}", 69420);

    // right justify
    println!("{number:>8}", number=5);

    // right justify with zeros
    println!("{number:0>3}", number=6);
    println!("{number:0>3}", number=60);

    // left justify with zeros
    println!("{number:0<3}", number=1);

    // named arguments for format specifier (requires $)
    println!("{number:0<width$}", number=1, width=3);

    // just use arguments from existing variables
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    // specify decimal places
    let pi = 3.141592;
    println!("Pi is roughly {pi:1.3}");
}