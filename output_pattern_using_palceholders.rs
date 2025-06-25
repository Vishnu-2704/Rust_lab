fn main() {
    for i in 1..=5 {
        // The placeholder `{}` is used to print the value of `i`, repeated `i` times
        for _ in 0..i {
            print!("{}", i);
        }
        println!(); // Move to the next line after each row
    }
}
