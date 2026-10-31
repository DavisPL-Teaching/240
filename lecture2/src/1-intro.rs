//!
//! Lecture 2, Part 1:
//! Introduction.
//!

/*
    Poll (Oct 28)

    For these questions, consider a language consisting only of one command: Foo.
*/

// Rust grammar:
enum FooLang {
    Foo,
}

// Formal grammar:
// <Program> ::= Foo

/*
    1. Suppose that a semantics (set S of valid triples) is equal to the empty set (∅). Which of the following is true for this semantics?
    <Options cut>

    2. Consider the following semantics S1 and S2. Which S2 are backwards compatible with S1?
    <Options cut>

    https://forms.gle/Vp7a5wrextN3vfEt8

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
*/

/*
    Poll (Oct 30)

    Today's short poll is for you to give me a bit of feedback on the pacing of the class so far.

    https://forms.gle/2bfhJpJDAAsT9nFx7

    .
    .
    .
    .
    .
*/

/*
    ===== Compilers. =====

    Compilers vs. interpreters

    Input and output
    Interpreter: (Input, Prog) -> Output
    Compiler: Prog1 -> Prog2
                (Input, Prog2) -> Output

    There are really 3 languages involved:
    - The source language (language in which we write the program)
    - The meta language (language in which we write the compiler)
    - The target language (language to which we compile the program)

    Recall: In Lecture 1 we gave a definition of what it means for an interpreter to
        *satisfy* a formal syntax and semantics.
        What is the definition for compilers?
        (Work this out below)

Definition:
(SKIP - this will be on your HW HW2)

    Let M be the semantics (set of (INPUT, prog, OUTPUT)) for the metalanguage

    Let S be the semantics of the source language

    Let T be the semantics of the target language

    A compiler C is a program in the metalanguage.

    A compiler C implements S if...

Aside: this requires a programming language impl in order to write a new programming language!
    And the same is true for interpreters.
    So how do we get the "first" programming language impl?

        A: "In the beginning, was the hardware"

            Once you have hardware, you can write machine code

            Once you have machine code, you can write an interpreter for
                other instructions (assembly instructions, etc.)

Aside: the Rust compiler is written in Rust. How is this possible? :-)

    "Bootstrapping"

        1. Write a Rust compiler in OCaml or C

            I now have a compiler, C1: written in OCaml, takes Rust, produces machine code

        2. Write a Rust compiler in Rust

            Now I have a compiler, C2: written in Rust, take Rust, produce machine code

        3. Compile C2 using C1

            I get some machine code M2 for the compiler

        4. **Extra validation steps:**

            + Compile C2 using C2
                    --> check that I get the same machine code as with C1.

            + Check the existing machine code for C1
                    --> check that this is the same as M2

            + Run random programs through C1 and C2, see that they agree.

        I can now run M2 on any Rust program to produce machine code.

    Q: What is the advantage of bootstrapping?

        A: If you're writing the Rust compiler, you get access to Rust features
                --> Memory safety in your code
                --> Want access to Rust `enum`s

        A: Once you have the new compiler, you don't need the old one anymore

        A: "Dogfooding": It allows you to stress test your own features.

            Proves that your language is good or worth using along the way.
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

#[derive(Debug)]
struct Variable(String);

// NB: This saves us some boilerplate below
use std::fmt::{self, Display};
impl Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/*
    M: Rust
    S: RandProg
    T: Rust

    What should we compile RandProg to?

    - we could compile to RandProg? (Not very useful)

    - Let's compile RandProg to Rust.
*/

struct RandProgCompiler {
    // Do we need any fields?
}

impl RandProgCompiler {
    fn new() -> Self {
        Self {}
    }
    fn compile(&mut self, prog: &RandProg) -> String {
        // TODO: implement this function.
        match prog {
            RandProg::SetVar(v, b) => {
                // let v = true;
                // let v = false;
                // let mut result = String::new();
                // result.push_str("let ");
                // result.push_str(&v.0);
                // result.push_str(" = ");
                // result.push_str(&b.to_string());
                // result.push(';');
                // result
                // NB:
                // Can probably do this in one line with the
                // format!() macro.
                format!("let {} = {};", v, b)
            }
            RandProg::CopyVar(v1, v2) => {
                // Example: with format macro
                format!("let {} = {};", v1, v2)
                // ^^ Problem with this?
            }
            RandProg::FlipCoin(v) => {
                // Using util.rs
                format!("let {} = util::rand_bool();", v)
                // ^^ Problem with this?
            }
            RandProg::Input(v) => {
                // Using util.rs
                format!("let {} = util::get_bool_input();", v)
                // ^^ Problem with this?
            }
            RandProg::Output(v) => {
                // This one is a little weird:
                // we need to escape the strings
                // EXAMPLE: want println!("{}", myvar);
                format!("println!({{}}, {});", v)
            }
            RandProg::Seq(prog1, prog2) => {
                // Recursive case
                let rs1 = self.compile(prog1);
                let rs2 = self.compile(prog2);
                format!("{}\n{}", rs1, rs2)
            }
            RandProg::If(b, prog1, prog2) => {
                // Recursive case
                let rs1 = self.compile(prog1);
                let rs2 = self.compile(prog2);
                format!("if ( {} ) {{ {} }} else {{ {} }}", b, rs1, rs2)
            }
        }
    }
}

#[test]
fn test_compile() {
    let prog = RandProg::SetVar(Variable("x".to_string()), true);
    let mut compiler = RandProgCompiler {};
    let compiled_prog = compiler.compile(&prog);
    assert_eq!(compiled_prog, "let x = true;");
}

/*
    Recap:

    - To write a compiler, we need 3 languages: a source language, metalanguage, and a target language
        (Unlike an interpreter, which only has 2 languages)

    - Each has its own semantics that we have to worry about!

    - We talked about dependencies in how compilers are written:
        + there must have been a "first" compiler or interpreter (the first compiler = hardware)

        + writing a compiler in its own language: Bootstrapping

        + Advantages/drawbacks of bootstrapping

    - We started implementing a compiler for RandProg in Rust (one case)

    - Next time: other cases + problems that emerge.

    ----------------------------------------

    Questions:
    Problems with the above?
*/

/*
    We have a compiler!
    But we will likely notice two things:

    1. We often write a lot of repetitive code.

    2. The code we produce as output -- such as for random coin generation or user input
      -- may involve some "implementation details". In this case, a lot of Rust syntax/details
      that may be tedious or low-level.
      Also, code that makes sense for one
      platform/implementation that may not make sense for another.

        ---> This is called platform specific code.

        ---> Most real world compilers support many platforms and architectures.
*/
