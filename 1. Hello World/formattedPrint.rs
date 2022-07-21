fn main(){
    // automatically replace with arguments 
    println!("Hello {}", "world");

    // positional arguments can be used in rust programming 
    println!("Hello {0}, {1}, {2} and {0} , {2}, {1}", "one", "two", "three");

    // different formattings can be invoked using :
    println!("number {}", 3445);
    println!("number {:b}", 3445);
    println!("number {:o}", 3445);
    println!("number {:x}", 3445);
    println!("number {:X}", 3445);

    // right aligning letters with space
    println!("{number:>5}", number = 5);

    // padded with zeros 
    println!("{number:0>5}", number = 5);

    // can name the arguments
    println!("{number:0>width$}", number = 12, width = 8);

    // fixme 01
    println!("my name is {1} {0}", "Bond", "James");

    // fixme 02
    println!("my name is {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.

    // formatted print
    format!("this is about formatted print");
    print!("this will be not new lined. ");
    println!("this will be comprised with other lines ");
    eprint!("This will be io:stderr type print");
    eprintln!("This will be io:stderr type print with a new line along with the previous line ");
    eprintln!("This will be io:stderr type print creates a new line afterwords");
    println!("check weather there is a statement in a new line");

    // activity 
    let pi = 3.141592;
    println!("{}", pi); // usual printing
    println!("{Pi:.*}", 3, Pi = pi); // with precision

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
}