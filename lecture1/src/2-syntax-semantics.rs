/*
    Lecture 1
    Part 2:
    More on Syntax and Semantics

    === Recap from last time ===

    - A programming language is defined by a **syntax**
      and a **semantics.**

    - Syntax: we can use a Rust enums to describe syntax.
    We used an example of a (silly/minimal) programming language with only three commands

    - Semantics: we made the following **main point:**
    describing the meaning
    of those commands in *English* can be confusing an ambiguous.
    In fact, even stuff like "Increment the program counter by n"
    and "Display a string" led to different possible interpretations!

    So what we want is a way to describe the semantics in a more
    formal, or unambiguous way.

    But first, let's generalize our language to fill in a few
    missing things.
*/

// Language: NQAS
#[derive(Debug, PartialEq, Eq)]
enum NotQuiteAsSilly {
    Increment(usize), // Increment the program counter by n
    Print(String),    // Display a string
    HaltIf(usize),    // Halt if program counter is at least n
}

/*
    What's missing from the above?

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

    Exercise: generalize our language to include
    a more realistic subset of programs.
*/

/*
    Semantics:

    How can we give a more formal semantics for our programs?
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
    === Program equivalence ===

    Once we have a semantics, we can talk about interesting things such as:

    - Two programs, P1 and P2 are **equivalent** if:

    Example of two programs that are equivalent?
*/

/*
    Some theory

    === Formal definitions ===

    **Syntax.**

    Syntax defines a *set* of valid programs.
    It's a formal language (a subset of all strings over an alphabet.)

        *Note:* what is the alphabet? traditionally ASCII. But modern programming
        languages usually allow Unicode :-)

    Syntax is almost always given as a formal grammar.

    Each program element is given in terms of

        <Number> ::= 0 | 1 | <Number> + <Number>

    What's the formal grammar for our language above?

    *Note:*
    We can think of our Rust enum as a formal grammar.
    It doesn't match exactly (in fact, we are assuming the program
    has already been *parsed* via that grammar -- more on this soon),
    but it is convenient to work with. We're not worried right now
    about how programs in our language are represented.
*/

/*
    *Semantics.*

    There are many ways to define semantics.

    Most common:
*/

/*
    Questions:

    1.

    2. What is semantics good for?

    3. How do syntax and semantics play into the development of...

        compilers?

        static analysis tools?
*/
