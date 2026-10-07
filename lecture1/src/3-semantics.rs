/*
    Lecture 1
    Part 3: Formal Semantics

    We said last time we need a way for everyone to agree
    on what programs mean.
    This is formal semantics.

    Syntax is given as a **formal grammar** that tells us the set of valid programs.
    (for now, we think of formal grammar == Rust enums and structs defining an
    abstract data type)

    We may define formal semantics as an **oracle** that allows or does not allow any example
    of the form

        (INPUT, program, OUTPUT).

    "INPUT" and "OUTPUT" should be considered general here.
    They can be any input or any output mechanisms (user input, terminal or
    command line arguments, file access, and any other relevant "external"
    program behavior, etc.)

    Example:

        (INPUT, "print x", OUTPUT)

        is valid only if OUTPUT consists only of printing the contents of x to the terminal (and no other behaviors).

    Why "oracle"?

    Note that semantics is defined over **all** programs, not just one specific program.

        Aside: Semantics is like a true/false property over *all possible* programs and program behaviors.
        (Maybe helpful for those who took ECS 261)

        Yes/No -- did this program execute correctly?

    There are several approaches to define formal semantics.
    We noted last time that one simple approach is
    to just define a "bespoke" compiler or interpreter that defines
    the correct semantics.

    (This is actually not a good approach in general, but we will
     discuss the limitations with this later on.)

    Potentially relevant definitions:

        - An **interpreter** is ...

        - A **reference interpreter** is ...

        - A **compiler** is ...

        - A **reference compiler** is ...

    How do we derive a true/false "oracle" from these definitions?


*/
/*
    ===== Poll =====

    Last time, we defined semantics as an "oracle" that accepts
    or does not accept any example
        (INPUT, program, OUTPUT).

    Which of the following are valid semantics according to this definition?
    (For the purposes of this question, imagine your favorite programming language,
     say, Python or C++.)

    Select all that apply.

    A) All programs must take input 5 and return 7.
    B) Every program that is of the form "Print x; Print y;" should print x and then y to the terminal, separated by newlines.
    C) Every program that is of the form "Print x; Print y;" should print x and y to the terminal, in either order, and these may or may not be separated by newlines.
    D) Every program must run in at most 5 seconds.
    E) On input x = 5, if the program is a function of the form "f(x): x += 1; return x", it should return either 6 or "integer overflow error". (Similarly for any other program of this form for any other integer variable.)

    https://forms.gle/49jg6n7XxCCRrUBm8

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
    Let's implement a semantics for our language via a reference interpreter.

    This will also help us see how representing programs as an abstract data type is useful
    when writing compilers and interpreters.
*/
// We could import these ...
// use super::syntax::{ProgramV2, IntExpr, ...} etc.
// Redefining below, slightly cleaned up

#[derive(Debug, PartialEq, Eq)]
enum Program {
    Increment(usize),                             // Increment the program counter by n
    Print(String),                                // Display a string
    HaltIf(usize),                                // Halt if program counter is at least n
    Sequence(Box<Program>, Box<Program>),         // sequence of two programs - prog1; prog2
    IfThen(BoolExpr, Box<Program>, Box<Program>), // if condition then program1 else program2
    Loop(BoolExpr, Box<Program>),                 // loop
}

#[derive(Debug, PartialEq, Eq)]
enum BoolExpr {
    Eq(IntExpr, IntExpr),
    LessThan(IntExpr, IntExpr),
    LessThanEq(IntExpr, IntExpr),
    // Would usually be recursive
    // Or(Box<BooleanExp>, Box<BooleanExp>),
    // And(Box<BooleanExp>, Box<BooleanExp>),
}

#[derive(Debug, PartialEq, Eq)]
enum IntExpr {
    NumberLiteral(isize),
    Add(Box<IntExpr>, Box<IntExpr>), // Recursive!
    Mult(Box<IntExpr>, Box<IntExpr>),
    LeftShift(Box<IntExpr>, Box<IntExpr>),
}

// Uncomment to implement
// fn prog_interpeter() {
// }

// Uncomment to implement/run
// #[test]
fn test_prog_interpreter() {
    todo!()
}

/*
    Exercise: demonstrate one of the problems from one of the polls
    last week on ambiguous semantics.
*/

// fn interpreter_1() {}

// fn interpreter_2() {}

// with formal semantics: we can tell which of interpreter 1 or 2 is **wrong**, and which is right.

/*
    === Ambiguous versus nondeterministic semantics ===

    Ambiguity is not the same as nondeterminism!

    https://en.wikipedia.org/wiki/Nondeterministic_programming

    Here is an example language with two interpreters:
*/

#[derive(Debug, PartialEq, Eq)]
enum NondeterministicEx {
    Print(String),                                            // print a string
    StoreVar(String, usize), // store an integer value to a variable
    CopyVar(String, String), // copy one variable to another
    PrintVar(String),        // print the contents of a variable
    Choose(Box<NondeterministicEx>, Box<NondeterministicEx>), // ???
}

fn nondeterministic_interpreter(prog: &NondeterministicEx) {
    match prog {
        NondeterministicEx::Print(s) => {
            println!("{s}");
        }
        NondeterministicEx::StoreVar(s, n) => {
            // TODO: fill in a few of these
            todo!()
        }
        NondeterministicEx::CopyVar(s1, s2) => {
            todo!()
        }
        NondeterministicEx::PrintVar(s) => {
            todo!()
        }
        NondeterministicEx::Choose(p1, p2) => {
            // TODO
            // Different choices here
            // All allowed by the semantics!
            unimplemented!()
        }
    }
}

/*
    Key difference:
    - Ambiguous semantics:
      The language specification ...

    - Nondeterministic semantics:
      The language specification ...

    This is a valid semantics! But the tools we have so far aren't enough to describe
    its semantics.
    Formal semantics via a reference interpreter is not enough.

    .

    Other examples of ambiguity?

    We need to allow multiple ways of executing programs! Otherwise, often, program optimization wouldn't
    be possible.

    .
    .
    .
    .
    .

    === Revising the poll on first day of class: Is it a programming language? ===

    Expanding our intuition about what a programming language is

    Point: Syntax/semantics is very broad. It doesn't just include things like C and Python.
    Some examples:

    - Nondeterministic programming
      https://en.wikipedia.org/wiki/Nondeterministic_programming

    - Logic programming languages
      https://en.wikipedia.org/wiki/Logic_programming
      https://en.wikipedia.org/wiki/Datalog

    - Turing machines
      https://en.wikipedia.org/wiki/Turing_machine

    - Hardware description languages
      https://en.wikipedia.org/wiki/Verilog

    - Lambda calculus
      https://en.wikipedia.org/wiki/Lambda_calculus

    As far as we are concerned:
    - All of these are ways of giving instructions to computers to execute
    - All of these can be described by a formal syntax and formal semantics
    - All are valid programming languages.

    Revisiting the Lecture 0 poll:

    .
    .

    (Exercise: pick one and sketch below.)
*/

/*
    Things you can do with semantics.

    === Program equivalence ===

    Once we have a semantics, we can talk about interesting things such as:

    - Two programs, P1 and P2 are **equivalent** if:

    Example of two programs that are equivalent?

    This is the foundation of program optimization!

    This is why compilers are able to optimize your code.
    Without this, no optimization would ever be possible.
*/

/*
    === Theoretical foundations ===

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

    Reference interpreters or reference implementations are great, but they aren't
    fully general for all languages.

    There are many ways to define semantics.

    Most common:

    - Def. **Operational semantics.**

        + Small-step semantics:

        + Big-step semantics:

    - Def. **Denotational semantics.**
*/

/*
    === Discussion questions ===

    1. Why separate syntax and semantics?

    2. What is semantics good for?

    3. How do syntax and semantics play into the development of...

        compilers?

        interpreters?

        static analysis tools?
*/
