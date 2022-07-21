fn main(){
    // single line comments
    println!("commenting");
    /*
    multiple line commenting 
    line 2 
    line 3
    line 4
    */
    let x = 5;
    println!("This is another method of printing values {}", x);

    // printing more that two values in a single console 
    let (value1, value2);
    value1 = 10; value2 = "string value";
    println!("{1} {0}", value1, value2);
    println!("{} {}", value1, value2);
    
}
