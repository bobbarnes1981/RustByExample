
// define a struct called Structure containing a 32 bit int
#[derive(Debug)]
struct Structure(i32);

// define a struct called Deep containing a 'Structure'
#[derive(Debug)]
struct Deep(Structure);

// define a struct
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    // structure is printable
    println!("'Structure is printable {:?}", Structure(3));

    // 'Deep' is printable but we cannot control the way it is printed
    println!("'Deep' is prinbtable {:?}", Deep(Structure(4)));

    // pretty print
    let name = "Bob";
    let age = 41;
    let bob = Person { name, age };
    println!("{:#?}", bob);
}