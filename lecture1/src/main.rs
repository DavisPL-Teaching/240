/*
    ECS 240

    Lecture 1: Foundations of PL: Syntax and Semantics.
*/

// Modules
// Note: the #[path =] annotation is not normally needed. It's needed here because
// I wanted to start the filenames with numbers, which is not otherwise allowed.
#[path = "1-intro.rs"]
mod intro;

fn main() {
    println!("Hello, ECS 240!");
    println!();
    println!("To follow along, find the specific parts in src/1-intro.rs, src/2-..., etc.");
    println!();

    // Uncomment to run Part 1
    intro::main();
}
