/*
    ECS 240

    Lecture 1: Foundations of PL: Syntax and Semantics.
*/

#![allow(unused_variables)]
#![allow(dead_code)]

// Modules
// Note: the #[path =] annotation is not normally needed. It's needed here because
// I wanted to start the filenames with numbers, which is not otherwise allowed.
#[rustfmt::skip]
#[path = "1-intro.rs"]
mod intro;
#[rustfmt::skip]
#[path = "2-syntax.rs"]
mod syntax;
#[rustfmt::skip]
#[path = "3-semantics.rs"]
mod semantics;
// #[rustfmt::skip]
// #[path = "4-imp.rs"]
// mod imp;

fn main() {
    println!("Hello, ECS 240!");
    println!();
    println!("To follow along, find the specific parts in src/1-intro.rs, src/2-..., etc.");
    println!();

    // Uncomment to run Part 1
    // intro::main();
}
