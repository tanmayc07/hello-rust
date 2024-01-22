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
}
