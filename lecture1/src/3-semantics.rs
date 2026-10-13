/*
    Lecture 1
    Part 3: Formal Semantics

    Syntax is given as a **formal grammar** that tells us the set of valid programs.
    (for now, we think of formal grammar == Rust enums and structs defining an
    abstract data type)

    We said last time we need a way for everyone to agree
    on what programs mean.
    This is formal semantics.

    We may define formal semantics as an **oracle** that allows or does not allow any example
    of the form

        (INPUT, program, OUTPUT).

    "INPUT" and "OUTPUT" should be considered general here.
    They can be any input or any output mechanisms (user input, terminal or
    command line arguments, file access, and any other relevant "external"
    program behavior, etc.)

    Example:

        (INPUT, "print <expr>", OUTPUT)

        is valid only if OUTPUT consists only of printing the contents of <expr> to the terminal (and no other behaviors).

    Note that semantics is defined over **all** programs, not just one specific program.

        Aside: Semantics is like a true/false property over *all possible* programs and program behaviors.
        (Maybe helpful for those who took ECS 261)

        Yes/No -- did this program execute correctly?

    Why "oracle"?
    - Spoiler: in general, we won't want this oracle to
      necessarily be an executable program
      More commonly: it will be some mathematical definition
      of whether a particular program behavior is consistent
      with the formal semantics.

    There are several approaches to define formal semantics.
    We noted last time that one simple approach is
    to just define a "bespoke" correct version of a
    compiler or interpreter that defines
    the correct semantics.

    (This is actually not a good approach in general, but we will
     discuss the limitations with this later on. Why?
        1. compiler version could change!
        2. who watches the watcher?
        3. could be platform-dependent behavior
        4. compiler could have bugs
        5. ...
     )

    That being said, giving a correct/bespoke correct version
    of a compiler/interpreter is a valid way of defining
    formal semantics, because it gives one possible definition
    of the "oracle" I mentioned above.

    Potentially relevant definitions:

        - An **interpreter** is
            a program that takes as input source code,
            (perhaps producing some intermediate byte code or other
            intermediate representation),
            and an input,
            and directly evaluates it producing output.

        - A **compiler** is a program
            which takes as input source code and produces
            output executable machine
            code which can be run on a target platform
            on any input(s).

        - A **reference interpreter** is
            an interpreter which is taken as the "oracle"
            to define the formal semantics for some language.

        - A **reference compiler** is
            a compiler which is taken as the "oracle" to
            define the formal semantics for some language.

    How do we derive a true/false "oracle" from these definitions?
    (Revisit)
*/

/*
    ===== Poll =====

    Last time, we defined formal semantics as an
    "oracle" that accepts
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
fn prog_interpreter(prog: &Program, prog_counter: usize) {
    let mut prog_counter = prog_counter;
    // use Program::*;
    match prog {
        Program::Increment(u) => {
            prog_counter += u;
        }
        Program::Print(s) => {
            print!("{}", s);
        }
        Program::HaltIf(u) => {
            if prog_counter >= *u {
                // TBD:
                // Clippy wants us to do nothing
                // return;
            }
        }
        Program::Sequence(prog1, prog2) => {
            prog_interpreter(prog1, prog_counter);
            prog_interpreter(prog2, prog_counter);
            // ^^ TBD: how to update prog_counter
        }
        Program::IfThen(b, prog1, prog2) => {
            todo!();
        }
        Program::Loop(b, prog) => {
            todo!();
        }
    }
}

// Uncomment to implement/run
// #[test]
fn test_prog_interpreter() {
    todo!()
}

/*
    Quick recap:

    *Formal* program semantics may be given by defining **any**
    yes/no oracle for whether any example program and example behavior
    (INPUT, prog, OUTPUT) is allowed

    Program semantics need not be deterministic - there may be
    multiple possible outputs for the same input

    The oracle need not be executable

    - However, if we want to give an executable oracle, there
      are ways to do this, for example
      define a reference interpreter or a reference compiler
      for the language

    - We started defining a reference interpreter for our
      simple toy language, which worked by recursively
      evaluating on the syntax of toy programs, and evaluating
      each case.

      ----------------------------------------
*/

/*
    Oct 12

    Recall:

        *Formal* program semantics may be given by defining **any**
        yes/no oracle (or *set*) for whether any example program and example behavior
        (INPUT, prog, OUTPUT) is allowed.

        Often will be given mathematically, not necessarily an executable function.

    === Determinism ===

    Def. A semantics -- set of

        (INPUT, prog, OUTPUT)

    is

        - **deterministic** if for every INPUT and prog, there is at most one possible OUTPUT.

    we will not use the following definition much, but it is worth knowing:

        - **total** if for every INPUT and prog, there is at least one possible output.

    Why is semantics not always deterministic?

    We will start with a trivial/toy example of one "obvious" reason,
    then less obvious reasons.

    Example 1:
    Here is a language that now incorporates variables (but only boolean variables):
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

// To model variables, we can use Rust strings
struct Variable(String);
// ^^^^^ This is called a newtype pattern
// var = Variable("hello".to_string())
// var.0 // get the string inside

// To evaluate programs, we need some kind of variable store.
// We can use a HashMap in Rust
use std::collections::HashMap;
struct VariableStore(HashMap<String, bool>);

// Useful utility functions
use super::util;

// Reference interpreter
// Implemented as a method on Variable Store
// (Methods are like classes / object oriented programming style)
// Impl block syntax:
impl VariableStore {
    fn new() -> Self {
        // Self = VariableStore
        VariableStore(HashMap::new())
    }
    // Syntax for a Rust method
    // let mut store = VariableStore::new();
    // store.interpret(program);
    fn interpret(&mut self, prog: &RandProg) {
        match prog {
            RandProg::SetVar(v, b) => {
                // Note the .clone() here
                // Makes Rust memory/borrow checker happy (why?)
                self.0.insert(v.0.clone(), *b);

                // ^^^ We could use the entry API here
            }
            RandProg::CopyVar(v1, v2) => {
                // ex.: programmer wrote x = y, y is not yet defined
                self.0
                    .insert(v1.0.clone(), self.0.get(&v2.0).copied().unwrap_or(false));
            }
            RandProg::FlipCoin(v) => {
                // v = flip_coin()
                // TODO: Implement this case
                // util provides util::rand_bool()
                // set value of v in my variable store
                self.0.insert(v.0.clone(), util::rand_bool());
            }
            RandProg::Input(v) => {
                // Utility function
                // see util.rs
                self.0.insert(v.0.clone(), util::get_bool_input());
            }
            RandProg::Output(v) => {
                print!("{}", self.0.get(&v.0).copied().unwrap_or(false))
            }
            RandProg::Seq(prog1, prog2) => {
                // prog1; prog2
                // Run both programs in sequence
                self.interpret(prog1);
                self.interpret(prog2);
            }
            RandProg::If(b, prog1, prog2) => {
                // TODO: implement this case
                // get the value of b from the store
                let b_val = self.0.get(&b.0).copied().unwrap_or(false);
                if b_val {
                    self.interpret(prog1);
                } else {
                    self.interpret(prog2);
                }
            }
        }
    }
}

// ex: how to write an AND statement in this language?
// Get two user inputs and AND them?
// Output the result

fn and_program() -> RandProg {
    // take two inputs
    // Input(x)
    // Input(y)
    // check if both are true
    // if yes, output a variable corresponding to true
    // if not, output a variable corresponding to false
    // If(x,
    //      If(y, Output(x), Output(y)),
    //      Output(x)
    // )
    unimplemented!()
}

// (SKIP: Try running our interpreter on this example)

/*
    ===== Poll =====

    Consider the definition of semantics, and the program in this RandProg language

        FlipCoin(x);
        FlipCoin(y);
        Output(x);
        Output(y)

    Note: to define it in our Rust grammar it would be

        let x = Variable("x".to_string());
        let y = Variable("y".to_string());
        Seq(
            FlipCoin(x),
            Seq(
                FlipCoin(y),
                Seq(
                    Output(x),
                    Output(y)
                )
            )
        )

    You may assume our formal semantics requires that each Output() statement corresponds to exactly one output
    Boolean to the terminal (t or f).

    1. What are all possible outputs of this program that are possible according to our reference interpreter?

    2. Can we use our reference interpreter to define the formal semantics of RandProg programs?

    3. Can we use any **deterministic** reference interpreter to define the semantics of RandProg programs?

    Briefly - Why or why not? (For Questions 2 and 3)

    https://forms.gle/P6wSBogP1JukWE2d7

    .
    .
    .
    .
    .

    1. tt, tf, ft, ff

    2. we can define ANY possible (INPUT, prog, OUTPUT) ...
    ...
    branch on all possible program behaviors?

    How would you define the possible (INPUT, prog, OUTPUT) triples?

    Let's define that

        (INPUT, prog, OUTPUT)

    is valid if there is **at least one* execution of our reference interpreter
    on input INPUT that returns OUTPUT.
    ^^^

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

    But this is not the only reason we might not want our semantics to be deterministic!

    Even if we remove FlipCoin from our language, I argue that we *still* may benefit from
    allowing our semantics to be deterministic.
    We will discuss this more next time.

    ------------------------------

*/

/*
    Oct 14

    Poll:
    Which of the following is possible for RandProg programs:
    (Select all that apply)
    A) Two different inputs, for the same program, produce the same output
    B) Two different programs are equivalent (produce the exact same output
       or set of possible outputs on every input)
    C) A program is deterministic for all inputs: for all inputs, for that particular
       program, there is exactly one output
    D) A program is nondeterministic for all inputs: for all inputs, for that particular
       program, there are at least two possible outputs

    https://forms.gle/ZFQK3un1WM72axPe8
*/

/*
    Last time, we defined a grammar for a language called RandProg.
    We also gave it a full formal semantics in the following sense:

        (INPUT, prog, OUTPUT) is allowed if and only if OUTPUT is the output
        of the reference interpreter on **at least** one program execution.

    So for the first time in the class, we have defined a programming language!

        PL = Syntax + Semantics

    We have given it a formal syntax (via Rust enum) and a formal semantics (via the above definition).

    Questions:
    Q1. Why is the semantics not deterministic?

    A: There are four possible outputs in last time's poll.
*/

/*
    Q2. Let's say we remove the FlipCoin case. *then* do we want the semantics to be
    deterministic?
*/

/*
    Example 2:
    (We will not completely fill out this one)

    Here is a more complicated toy language with a heap:
*/

enum HeapProg {
    /* Some basic variable operations */
    // Now int instead of boolean
    SetVar(Variable, usize),
    CopyVar(Variable, Variable),
    // More variable operations
    IncVar(Variable),
    DecVar(Variable),
    // Input and output
    Input(Variable),
    Output(Variable),
    /* Memory operations */
    // Allocate a byte on the heap, store a pointer to it in the variable
    NewAlloc(Variable),
    // Read a byte from memory
    // x = *y;
    ReadMem(Variable, Variable),
    // Write a byte to memory
    // *x = y;
    WriteMem(Variable, Variable),
    // Control flow
    Seq(Box<HeapProg>, Box<HeapProg>),
    IfEq(Variable, Variable, Box<HeapProg>, Box<HeapProg>),
}

// // We can model a memory location as a Rust usize
struct Location(usize);

/*
    Exercise:
    What is the state of our program?
    What do we need instead of a memory store?
*/

// TODO
// struct HeapState {
// }

// Reference interpreter
// You could imagine defining an interpreter for our heap language.
// You can do that if you like on your own time.
// (SKIP)
// impl HeapState {
//     fn new() -> Self {
//         unimplemented!()
//     }
//     fn interpret(&mut self, prog: &HeapProg) {
//         unimplemented!()
//     }
// }

/*
    EXAMPLE:

    NewAlloc(x)
    NewAlloc(y)

    SetVar()


    Another example in a more C-like setting:

        int *x = new pointer;
        int *y = new pointer
        *x = 3;
        // some operations on *y
        *y = *y + 1
        // back to x
        print(x)

        Can we optimize this to this?

        int *y = new pointer
        *y = *y + 1
        print(3)

        (This is known as constant propagation)

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

    Some overall points:

    - Nondeterminism = more than one output per input

    - Any semantics given by a deterministic reference interpreter
      is deterministic. (why?)

    - Nondeterminism is useful for modeling inherently nondeterministic programs

    - AND nondeterminism is useful for leaving semantics under-specified, even
      when programs are inherently deterministic, in order to allow for program
      optimization, efficient execution and memory layout, etc.

    - Finally, nondeterminism is useful in cases where the language designers
      want to allow different interpretations to exist and leave some choices
      up to compiler/interpreter designers or particular platforms.

        ==> Example: ending print statements with a LF vs. CR;LF.
*/

/*
    === Ambiguous versus nondeterministic semantics ===

    Ambiguity is not the same as nondeterminism!

    https://en.wikipedia.org/wiki/Nondeterministic_programming

    Key definitions:
    - Ambiguous semantics:
      The language semantics ...

    - Nondeterministic semantics:
      The language semantics ...

    Exercise: demonstrate one of the problems from one of the polls
    last week on ambiguous semantics.
*/

// fn interpreter_1() {}

// fn interpreter_2() {}

/*
    Another example of nondeterministic semantics:
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
    This is a valid semantics! But the tools we have so far aren't enough to describe
    its semantics.
    Formal semantics via a reference interpreter is not enough.

    .

    Other examples of nondeterminism?

    We need to allow multiple ways of executing programs! Otherwise, often, program optimization wouldn't
    be possible.

    .
    .
    .
    .
    .
*/

/*
    Things you can do with semantics.

    === Program equivalence ===

    Once we have a semantics, we can talk about interesting things such as:

    - Two programs, P1 and P2 are **equivalent** if:

    Example of two programs that are equivalent?

    In our RandProg language?

    This is the foundation of program optimization!

    This is why compilers are able to optimize your code.
    Without this, no optimization would ever be possible.

    Another definition:

    - Program P1 is a **refinement** of program P2 if:
*/

/*
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

    From the Lecture 0 poll:

    .
    .

    (Exercise: pick one and sketch below.)
*/
