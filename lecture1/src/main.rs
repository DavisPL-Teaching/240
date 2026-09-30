/*
    ECS 240

    Lecture 1: Foundations of PL: Syntax and Semantics.
*/

#![allow(unused_variables)]
#![allow(dead_code)]

// Modules
// Note: the #[path =] annotation is not normally needed. It's needed here because
// I wanted to start the filenames with numbers, which is not otherwise allowed.
#[path = "3-imp.rs"]
mod imp;
#[path = "1-intro.rs"]
mod intro;
#[path = "2-syntax-semantics.rs"]
mod syntax_semantics;

fn main() {
    println!("Hello, ECS 240!");
    println!();
    println!("To follow along, find the specific parts in src/1-intro.rs, src/2-..., etc.");
    println!();

    // Uncomment to run Part 1
    // intro::main();
}
