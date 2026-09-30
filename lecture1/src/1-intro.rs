/*
    Lecture 1: Foundations

    Part 1: Introduction to Syntax and Semantics

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

    === Introduction ===

    This lecture serves as an introduction to the rest of the class!

    We'll cover topics that are foundational to the study of programming languages:
    starting from syntax and semantics.

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
        Fred <n> # takes an integer

        George <s> # take a string

    In Rust:
*/

#[derive(Debug, PartialEq, Eq)]
enum SillyLang {
    Fred(usize),    // Fred command: provided with an integer
    George(String), // George command: provided with a String
}

// ^^ This is a Rust enum.
// It means anything in my language is either a Fred or a George.
// We access the enum fields with SillyLang::Fred, SillyLang::George.

/*
    Example:
*/

pub fn main() {
    let prog1 = SillyLang::Fred(3);

    let prog2 = SillyLang::George("George".to_string());

    // We need to do something above to get this to work
    println!("{:?}", prog1);
    println!("{:?}", prog2);

    // This is a valid syntax! Syntactically we have two valid types of programs, Fred(..) and George(..)

    // Check whether programs are (syntactically) equal

    // ... and for this to work:
    let prog3 = SillyLang::Fred(3);
    assert_eq!(prog1, prog3);
    assert_ne!(prog2, prog3);

    // To run the code: uncomment in main.rs, then cargo run
}

/*
    This is a valid syntax!

    - We have described a syntax for programs
    - We have **not** described any semantics. If we were to define what
        Fred(3)

        and

        George("George")

        *mean,* then we would have a valid programming language!

    But maybe this doesn't seem very realistic. It doesn't "feel" like a programming language.

    Another language:
*/

// Language: NQAS
#[derive(Debug, PartialEq, Eq)]
enum NotQuiteAsSilly {
    Increment(usize), // Increment the program counter by n
    Print(String),    // Display a string
    HaltIf(usize),    // Halt if program counter is at least n
}

// Let's write a test for this one, this time as a unit test.

// How to write a unit test in Rust:
// test annotation
#[test]
fn test_syntax() {
    // Test for equality
    assert_eq!(NotQuiteAsSilly::Increment(3), NotQuiteAsSilly::Increment(3));
    // (Define debug, equality for this to work)
}
// Run with `cargo test`

/*
    So far: we have a **syntax** that defines the set of valid programs

    We know that we can write programs (they are just values in our custom data type),
    we can print out those programs, and we can compare them for equality.

    Okay... but we don't have a programming language just yet.

    Semantics:

    We've said what the programs are...

    - but we haven't said anything about what the programs *mean.*

    - We don't have a way of running the programs.

    Recall:

        __instructions__ = Syntax
        __convention__ = Semantics

    Semantics (**informal:**)

        - Increment

            Increment 3

            Increment 4

            Increment 5

            Program semantics?

            - Take whatever is given to it, and add one

            - Assign the resulting value to whatever is given to it

                Increment(VarName, usize)

            - Given a program counter, add the increment (integer value) to the program counter.

        - Print

            Print "Hello"

            Prints the string to the output buffer

        - HaltIf:

            If the program counter reaches the value given to the HaltIf, halt the program

    ===== Poll =====

    Semantics is about agreeing -- between different implementations, interpreters, or compilers --
    on what the programs mean. If we don't agree, we aren't writing in the same language!

    Given our informal description of the semantics above, can you think of an example of how
    two implementations might disagree on the semantics of NQAS programs?

    https://forms.gle/mpxWpmwj5qTW1Z8u9

    === Some recap of points ===

    1. A programming language is defined by a **syntax** and a **semantics.**

    2. very broad! We can define **any** syntax for
       valid programs, and then define what those programs mean

    3. Writing down the semantics of what the programs mean is necessary so that we can all agree

    4. It's very easy to imagine scenarios where if the semantics is written just in English
       there's a lot of misinterpretation about what the programs could mean and how to implement
       the language.

       (**informal** semantics)

    As a result, we'll investigate next how to define a **formal** semantics.

    ***** Where we ended for Monday *****
    ----------
*/

/*
    Semantics (a little more formally):

    There are many ways to define formal semantics.

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
    .
    .
    .
    .
    .

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
    Can we demonstrate the problem from the poll above?
*/

// fn interpreter_1() {}

// fn interpreter_2() {}

// with formal semantics: we can tell which of interpreter 1 or 2 is **wrong**, and which is right.

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
