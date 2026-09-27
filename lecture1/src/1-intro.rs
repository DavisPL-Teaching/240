/*
    Lecture 1: Foundations

    Part 1: Introduction to Syntax and Semantics

    This lecture serves as an introduction to the rest of the class!

    We'll cover topics that are foundational to the study of programming languages:
    starting from syntax and semantics.

    === Following along: ===

    - Clone this repository:

        git clone https://github.com/DavisPL-Teaching/240.git

        OR

        git@github.com:DavisPL-Teaching/240.git

    - Go into the current lecture:

        cd lecture1

    - If you haven't installed Rust yet: follow the instructions at HW0:

        https://github.com/DavisPL-Teaching/240-hw0

        (Should be very quick, a single command to install.)

    - Run the code: `cargo run`

    You should see a message "Hello, ECS 240" with some other info.

    === Note on pacing ===

    Note on pacing: please bear with us if you've seen Rust or some of the other concepts before!
    I will get some feedback from the class about how things are going and speed up/slow down as necessary.

    === Clippy ===

    Clippy is the Rust linter. (`cargo clippy`)

    Some clippy allows below.
    I often do this for work-in-progress projects.
    We don't want to turn off *all* warnings, just the ones that are not
    useful during development. (We should turn these back on before shipping
    production code.)
*/

#![allow(unused_variables)]
#![allow(dead_code)]

/*
    === The Basics ===

    In the first week, we discussed some motivation (why do programming languages matter? why study programming languages?)

    What is a programming language? Last time, we saw the following definition:

    A programming language is any __convention__ that is used for describing __instructions__ for computers to execute.
                                  ^^^^^^^^^^^^^^                             ^^^^^^^^^^^^^^^^

    Very broad! More specifically, any language is given by:

    - Instructions: this is the **syntax**

    - Convention: this is the **semantics** (everyone must agree on what the instructions *mean.*)

    Other than that, we can define things however we want.

    Let's define our own programming language.

    Syntax:

    Two commands:
        Fred <n>

        George <s>
*/

enum SillyLang {
    Fred(usize),
    George(String),
}

// ^^ This is a Rust enum.
// It means anything in MyLanguage is either a Fred or a George.
// We access the enum fields with SillyLang::Fred, SillyLang::George.

/*
    Example:
*/

pub fn main() {
    let prog1 = SillyLang::Fred(3);

    let prog2 = SillyLang::George("George".to_string());

    // We need to do something above to get this to work
    // println!("{:?}", prog1);
    // println!("{:?}", prog2);

    // ... and for this to work:
    // let prog3 = SillyLang::Fred(3);
    // assert_eq!(prog1, prog3);
    // assert_neq!(prog2, prog3);

    // To run the code: uncomment in main.rs, then cargo run
}

/*
    This is a valid programming language.

    But maybe this doesn't seem very realistic. It doesn't "feel" like a programming language.

    Another language:
*/

// Language: NQAS
enum NotQuiteAsSilly {
    Increment(usize), // Increment the program counter
    Print(String),    // Display a string
    HaltIf(usize),    // Halt if program counter is at least n
}

// Let's write a test for this one, this time as a unit test.

// How to write a unit test in Rust:
// test annotation
#[test]
fn test_syntax() {
    // ^^ name can be anything
    todo!()
}
// Run with `cargo test`

/*
    Okay... but we don't have a programming language just yet.

    Semantics:

    We've said what the programs are...

    - but we haven't said anything about what the programs *mean.*

    - We don't have a way of running the programs.

    There are many ways to define semantics.
    One way is to give a reference interpreter for the language.

    Recall:

        __instructions__ = Syntax
        __convention__ = Semantics

    Def:

        A syntax defines a **set of valid programs.**

    Def:

        - An **interpreter** is ...

        - A **compiler** is ...
*/

// Uncomment to implement
// fn nqas_interpeter() {
// }

// Uncomment to implement/run
// #[test]
fn test_nqas_interpreter() {
    todo!()
}

/*
    ===== Poll =====

    Semantics is about agreeing -- between different implementations, interpreters, or compilers --
    on what the programs mean. If we don't agree, we aren't writing in the same language!

    Can you think of an example of how two implementations might disagree on the semantics of
    NQAS programs?

    https://forms.gle/mpxWpmwj5qTW1Z8u9

    Can we demonstrate this?
*/

// fn interpreter_2() {}

/*
    === Some additional questions ===

    Q1: How many valid programs are there in this language?

    Q2: Ignoring different literals (integer and strings), how many valid programs are there?

*/

/*
    === Extending the language ===

    Is NQAS a "real" programming language? Sure!

    But it doesn't seem that useful yet.

    Q: What's missing from the language so far?

    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .
    .

    Several possible answers

    Updating our interpreter:
*/

/*
    ===== Adding features =====

    What else is missing here?

    Features we could add:
*/

/*
    === Program equivalence ===

    Once we have a semantics, we can talk about interesting things such as:

    - Two programs, P1 and P2 are **equivalent** if:

    Example of two programs that are equivalent?
*/

/*
    === Syntax and semantics, more formally ===


    Syntax is usually given as a formal grammar.

    You know like:

        <Number> ::= 0 | 1 | <Number> + <Number>

    What's the formal grammar for our language above?




*/

/*
    === Recap ===

    Even though we haven't done anything fancy, we already have all the pieces
    we need for a general programming language:
    a grammar, a way of writing programs in that grammar, and
    some semantics which tells us what programs mean.

    We will go into more detail on these topics in the following lectures.
*/
