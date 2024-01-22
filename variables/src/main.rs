fn main() {
    // Mutability and Immutability
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Variable Shadowing
    let y = 5;

    let y = y+1;

    {
        let y = y*2;
        println!("The value of y inside scope is {y}");
    }
    println!("The value of y outside scope is {y}");

    let spaces = "   ";
    let spaces = spaces.len();

    // Type annotations
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("Value of guess is {guess}");

    // integers
    let val1: u8 = 6;

    // floating-point
    let float1 = 2.0; // f64

    let float2: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;
    println!("value of sum is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("value of difference is {difference}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("quotient = {quotient} truncated = {truncated}");

    // booleans
    let f: bool = false; // with explicit type annotation

    // char type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c={c} z={z} heart_eyed_cat={heart_eyed_cat}");

    // tuples
    let tup: (f64, i32, char) = (1.1, 10, 'x');
    println!("tuple = {tup:?}");
    let (t1, t2, t3) = tup;
    println!("t1={t1} t2={t2} t3={t3}");
    let tup1 = tup.0;
    let tup2 = tup.1;
    println!("tup1={tup1} tup2={tup2}");

    // Empty tuple is called as unit - its type & value is () 

    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a_init: [i32; 5] = [3; 5]; // initialized with default value
    println!("a={a:?} a_init={a_init:?}");

    let first = a[0];
    let second = a[1];
    println!("a[0]={first}");
}
