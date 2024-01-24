fn main() {
    let mut s = String::from("Hello");

    s.push_str(", World!");
    println!("{s}");

    // move: only stack data of s1 is copied to s2 and s1 is not valid thereafter
    let s1 = String::from("Hi");
    let s2 = s1;

    // println!("{s1}");

    // deep copy with clone(): both stack and heap data are copied
    let st1 = String::from("Hola");
    let st2 = st1.clone();

    println!("st1 = {st1} st2 = {st2}");

    // Functions with ownership
    let stt = String::from("Passing");

    takes_ownership(stt);

    let x = 5;
    makes_copy(x);

    // Return values transfer ownership
    let stt1 = gives_ownership();
    println!("stt1={stt1}");

    let stt2 = String::from("hello");
    println!("stt2={stt2}");

    let stt3 = takes_and_gives(stt2);
    // println!("stt2={stt2}");
    println!("stt3={stt3}");

    // Multiple return values
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives(a_string: String) -> String {
    a_string
}

fn takes_ownership(some_string: String) {
    println!("some_string: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}
