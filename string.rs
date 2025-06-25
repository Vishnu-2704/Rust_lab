fn main() {
    // Declaring a string literal (immutable)
    let s_literal = "Hello, Rust!";

    // Converting the string literal to a String object
    let s_object = String::from(s_literal);

    // Alternatively (also common in practice)
    let s_object2 = s_literal.to_string();

    // Print all strings
    println!("String Literal: {}", s_literal);
    println!("String Object (from): {}", s_object);
    println!("String Object (to_string): {}", s_object2);
}
