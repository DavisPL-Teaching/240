//!
//! Lecture 2, Part 1:
//! Introduction.
//!

/*
    Compilers are like interpreters, but there are really 3 languages involved:
    - The source language (language in which we write the program)
    - The meta language (language in which we write the compiler)
    - The target language (language which we compile the program)

    Recall: In Lecture 1 we gave a definition of what it means for an interpreter to
        *satisfy* a formal syntax and semantics.

    Aside: this requires a programming language impl in order to write a new programming language!
        And the same is true for interpreters.
        So how do we get the "first" programming language impl?

    Aside: the Rust compiler is written in Rust. How is this possible? :-)
*/

/*
    Let's start by defining a compiler for RandProg.

    Here's RandProg again:
*/

enum RandProg {
    /* Some basic variable operations */
    SetVar(Variable, bool),      // x = true or x = false
    CopyVar(Variable, Variable), // x = y
    // Random number generation
    FlipCoin(Variable), // x = flip_coin()
    // Input and output
    Input(Variable),  // x = get_input()
    Output(Variable), // e.g., print to terminal print(x)
    // Control flow
    Seq(Box<RandProg>, Box<RandProg>),          // prog1; prog2;
    If(Variable, Box<RandProg>, Box<RandProg>), // if x { prog1 } else { prog2 }
}

struct Variable(String);

/*
    What should we compile RandProg to?

    Let's compile to Rust!
*/

struct RandProgCompiler {
    // TODO: fill in here
}

impl RandProgCompiler {
    fn new() -> Self {
        // TODO: initialize compiler
        unimplemented!()
    }
    fn compile(&mut self, prog: RandProg) -> String {
        // TODO: return a Rust program as a string.
        unimplemented!()
    }
}
