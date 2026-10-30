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

struct Variable(String);

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
                let mut result = String::new();
                result.push_str("let ");
                result.push_str(&v.0);
                result.push_str(" = ");
                result.push_str(&b.to_string());
                result.push(';');
                result
                // NB:
                // Can probably do this in one line with the
                // format!() macro.
            }
            RandProg::CopyVar(v1, v2) => {
                unimplemented!()
            }
            RandProg::FlipCoin(v) => {
                unimplemented!()
            }
            RandProg::Input(v) => {
                unimplemented!()
            }
            RandProg::Output(v) => {
                unimplemented!()
            }
            RandProg::Seq(prog1, prog2) => {
                unimplemented!()
            }
            RandProg::If(b, prog1, prog2) => {
                unimplemented!()
            }
        }
        // Code from interpreter in Lecture 1. Can we adapt?
        // match prog {
        //     RandProg::SetVar(v, b) => {
        //         // Note the .clone() here
        //         // Makes Rust memory/borrow checker happy (why?)
        //         self.0.insert(v.0.clone(), *b);

        //         // ^^^ We could use the entry API here
        //     }
        //     RandProg::CopyVar(v1, v2) => {
        //         // ex.: programmer wrote x = y, y is not yet defined
        //         self.0
        //             .insert(v1.0.clone(), self.0.get(&v2.0).copied().unwrap_or(false));

        //         // CopyVar(v1, v2) is like a C program
        //         //     v1 := v2;
        //         // unwrap_or?
        //         //     we're saying that if v2 is not defined (uninitialized variable!)
        //         //     then, we are just going to set v1 = false.
        //         //     one reason is it false or is it missing?
        //         //     this is Python's reason!
        //         //     C's reason?
        //         //     C assumes you know what you're doing, and that you're doing some kind
        //         //     of clever memory manipulation
        //         //     C's main reason: efficiency!
        //         //     if every time v1 := v2, and v2 is not defined, I have to set v1 = false,
        //         //     then, in a C compiler, I would need to initialize all memory to false!
        //         //     and this is additional runtime overhead!
        //         //     According to C, unnecessary runtime overhead is always a bad thing.
        //     }
        //     RandProg::FlipCoin(v) => {
        //         // v = flip_coin()
        //         // TODO: Implement this case
        //         // util provides util::rand_bool()
        //         // set value of v in my variable store
        //         self.0.insert(v.0.clone(), util::rand_bool());
        //     }
        //     RandProg::Input(v) => {
        //         // Utility function
        //         // see util.rs
        //         self.0.insert(v.0.clone(), util::get_bool_input());
        //     }
        //     RandProg::Output(v) => {
        //         print!("{}", self.0.get(&v.0).copied().unwrap_or(false))
        //     }
        //     RandProg::Seq(prog1, prog2) => {
        //         // prog1; prog2
        //         // Run both programs in sequence
        //         self.interpret(prog1);
        //         self.interpret(prog2);
        //     }
        //     RandProg::If(b, prog1, prog2) => {
        //         // if b then prog1 else prog2
        //         // get the value of b from the store
        //         let b_val = self.0.get(&b.0).cloned().unwrap_or(false);

        //         // .get() returns us a reference: Option<&Value>
        //         // no entry in variable store => None
        //         // is an entry in the variable store => Some(&t)
        //         // Some(t)
        //         // .copied(): Some(&t) => Some(t)
        //         // .cloned(): same thing
        //         // .unwrap(): Some(x) => x, None => crash your program (panic)
        //         // .unwrap_or(y): Some(x) => x, None => y

        //         // .clone() copy => ways to turn a pointer into a value
        //         // .copied() .cloned() => special operations on Option<&T>

        //         /*
        //             Simple rule:
        //             If you have an Option<&T> => use .cloned() or .copied()
        //             If you have a &T => use .clone()

        //             null ptr from other languages => None
        //             regular (non-null) pointer => &T and &mut T
        //         */

        //         if b_val {
        //             self.interpret(prog1);
        //         } else {
        //             self.interpret(prog2);
        //         }
        //     }
        // }
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

    - We talked about circular dependencies in how compilers are written:
        + there must have been a "first" compiler (the first compiler = hardware)

        + writing a compiler in its own language: a process called Bootstrapping

        + Advantages/drawbacks of bootstrapping

    - We started implementing a compiler for RandProg in Rust and going through the cases

    - We'll see a couple other cases and what problems that emerge as a result of this next time.

    ----------------------------------------
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
