//!
//! Lecture 2, Part 1:
//! Introduction.
//!

/*
    Poll:
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
    ===== Compilers. =====

    Compilers are like interpreters, but there are really 3 languages involved:
    - The source language (language in which we write the program)
    - The meta language (language in which we write the compiler)
    - The target language (language to which we compile the program)

    Recall: In Lecture 1 we gave a definition of what it means for an interpreter to
        *satisfy* a formal syntax and semantics.
        What is the definition for compilers?
        (Work this out below)

    Aside: this requires a programming language impl in order to write a new programming language!
        And the same is true for interpreters.
        So how do we get the "first" programming language impl?

    Aside: the Rust compiler is written in Rust. How is this possible? :-)

Definition:

    Let M be the semantics (set of (INPUT, prog, OUTPUT)) for the metalanguage

    Let S be the semantics of the source language

    Let T be the semantics of the target language

    A compiler C is a program in the metalanguage.

    A compiler C implements T if...
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

    - we could compile to RandProg? (Not very useful)

    - ...
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
        // TODO: implement this function.
        unimplemented!()
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

/*
    We have a compiler!
    But we will likely notice two things:

    1. We often write a lot of repetitive code.

    2. The code we produce as output -- such as for random coin generation or user input
      -- may involve some "implementation details". Code that makes sense for one
      platform/implementation that may not make sense for another.

        ---> This is called platform specific code.

        ---> Most real world compilers support many platforms and architectures.
*/

/*
    ===== Syntactic sugar =====

    Exercise: generalize our RandProg to have a new construct.
    Compile it out by

    Q: Can any existing constructs be viewed as syntactic sugar?
*/

/*
    Def. An *intermediate representation* is ...

    ===== Platform-specific code =====

    Example:
    We could write different compilers, one which produces output
    to the terminal; and one which returns output by displaying it in a dialogue box to the
    user; and yet another which produces output by saving it to a file.
*/
