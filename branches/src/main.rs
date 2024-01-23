fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if in a `let` statement
    // since if is an expression it can be used in statements
    let condition = true;
    let number = if condition {5} else {6};
    println!("The value of number is: {number}");

    let number = if condition {5} else {"string"};   // error
    println!("The value of number is {number}");
}
