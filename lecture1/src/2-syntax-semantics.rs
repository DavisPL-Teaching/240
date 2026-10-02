/*
    Lecture 1
    Part 2:
    More on Syntax and Semantics

    === Side note: a Rust & Rust syntax. ===

    Rust syntax takes some getting used to!

    - Rust has a higher learning curve than some other languages.

    - Please stop me or raise your hand if there's something you don't understand!

    - Some things to keep in mind about Rust philosophy:

        + Rust syntax is often similar to (inspired by) C/C++ syntax.

          Rust was targeted at "frustrated C/C++ developers". If that applies to you,
          or if you have some experience writing code in C/C++, the language will often
          make a lot more sense.

            (During an internship in 2020, I used to spend 48+ hours at a time
             debugging memory errors in C/C++ code)

        + Rust wants to be *explicit.* Remember coming from C-land: we want to support pointers,
          access to raw memory, allocation/deallocation. Rust will make us be explicit about things
          like memory allocation and copying.

            "Zero-cost abstractions" = no hidden operations!

            But -- if we don't care, there are some tricks to get around this.

                e.g.: .clone()

        + Rust is "strongly statically typed." That means that the type of every variable
            (string, int, etc.) is known at compile time.
            Very useful for us when designing a language! For example we can exhaustively match
            on Rust enums
            Here is the syntax:

            match my_prog {
                Increment(_) => { do_something() }
                Print(_) => { do_another_thing() }
                HaltIf(_) => { do_something_else() }
            }

            and we know that our cases are exhaustive. (Rust will complain if we don't handle every case.)

        + Rust is most interested in providing two things: the code should be *fast*, and the code should be *safe.*
        Safe means that some bad thing (e.g., "Segmentation fault") doesn't occur when running the code.

            What this means when learning Rust is that the compiler will often complain about your code.
            :-)
            Don't worry! It takes a few weeks before most programmers are able to write Rust code that
            passes the compiler.

            Rust always offers a way to "opt out" of safety abstractions

                e.g.: clone()
                e.g.: Rc in the standard library
                    &usize -> Rc::RefCell<usize>

            If you know the proper incantation, you can sidestep Rust
            safety features that you're not interested in.

    - We're using Rust mainly for some of its features (especially static types), which are useful when
    building programming language tools.
    However, along the way, I hope that some of the features we see in Rust will also give us
    some opportunities to learn about programming language design as a case study in their own right.

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
    (What we will do today.)
*/

/*
    === Poll (Sep 30) ===

    Building on last time's poll.

    Recall (informal): **Syntax** defines the set of valid programs. (We used a Rust enum)
    **Semantics** is a convention where we all agree on what those programs mean and how they should
    be executed.

    Which of the following is an example of ambiguous semantics?

    * In one compiler, the "print" command is case-sensitive. In another, it is case-insensitive ("print" or "Print" or "PRINT" is allowed).

    * It is not specified whether "print" should display a line ending in a newline, or not.

    * When incrementing the program counter, it is not specified whether we return an integer or a floating point.

    * It is not specified whether the command "LoopUntilHalting", which repeats another command until the program terminates, is a valid program.

    * It is not clear whether a compiler or interpreter for our language should be written in C, Python, Rust.

    https://forms.gle/aDFwbPPszPyHTcUM8

    .
    .
    .
    .
    .

    --------------------------------------------------

    (Continuing here for Oct 2)

    === Addendum from last time ===

    Last time, there was a lot of discussion about, "why Rust?"
    (Apologies for taking a lot of class time on this!)
    Just want to add a few things:

    - Google news: https://www.reddit.com/r/rust/comments/1wukyos/google_is_doing_large_scale_codebase_migrations/

    - Piazza post: https://piazza.com/class/ms5m7g95gq1tr/post/16

    TL;DR:

    - Increasingly used for safe and secure systems software (including tools relevant to
      this class like compilers & optimizers!)

    - Ties into a point from Lecture 0 (last Wed.):
    Recall Python/C++ examples:

        + default list example in Python
        + vector push_back example in C++ (vector moves, causing read/write to arbitrary memory)

    **Programming language design decisions* affect whether or not it is easy to accidentally
    write bugs.

    === Poll (Oct 2) ===

    (Note: From this point on:
    - polls will generally have a "right" answer, and can be used to help study for the exams
    - poll answers will be shared with the lecture notes! I plan to keep this updated after the end of each week.
    )

    Which of the following most likely reflects the design philosophy of Rust, from a programming language design standpoint?
    (Select all that apply)

    A. Make syntax unnecessarily difficult, so that it takes a long time to write programs

    B. Restrict the syntax of the language so that it is more difficult to write erroneous programs

    C. Allow many different syntaxes for the same thing ("There's more than one way to do it")

    D. Restrict the semantics of the language as much as possible, so that it is not ambiguous

    E. Adopt a semantics that tightly couples the syntax of a program with its performance and memory usage characteristics

    https://forms.gle/7BWi5EDCzAhec6s89
*/

/*
    === Continuing our example language ===

    Before we describe the semantics in a more formal way,
    let's generalize/fix our language to fill in a few missing things.
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
    - Ways to execute the code?
    - No syntax for variables!
    - Control flow -- jump or goto? loops?
        - Return statement?
    - Operations? Such as addition, ors, nots (basic ops on basic data types like integers/booleans)
    - Primitive data types
    - Expressions?
    - Entrypoint to the program?
    - What is the program counter?
    - Way of compiling the code?
    - Semantics unclear

    A way of combining programs?
    - If? sequencing programs -- loops came up

    could we:
        - one command --> newline or semicolon --> another command
        --> end of program token

    infamously bad one? every time the font changes, that's a new statement

    sequencing the program: how things are structured in memory?

    Can we have a pipe | which is accepting an input and giving output
*/

// // Language: NQAS
// #[derive(Debug, PartialEq, Eq)]
// enum ProgramV2 {
//     Increment(usize), // Increment the program counter by n
//     Print(String),    // Display a string
//     HaltIf(usize),    // Halt if program counter is at least n
//     Sequence(Box<ProgramV2>, Box<ProgramV2>), // sequence of two programs!
//     IfThen(Condition, Box<ProgramV2>, Box<ProgramV2>), // if condition then program1 else program2
//     Loop(Condition, Box<ProgramV2>), // loop
// }
// Box? Just a pointer to the heap.
// // Rust will give us 2 errors here:
// 1. we haven't defined Condition
// struct Condition {
//     BooleanExpression,
//     Operator,
//     BooleanExpression,
// }
// struct BooleanExpression {

// }

/*
    Quick Recap:

    - We talked a little more about Rust motivation (Rust language design and
      why it is the way it is), including in today's poll, that also ties back
      into Lecture 0 and syntax + semantics
    - We brainstormed how to generalize our language -- we need at least, a way
      of *combining* programs, and maybe some other features such as variables,
      basic data types, ...
    - We saw one way to allow combining programs: giving a recursive data types
        spoiler: this is what's called an Abstract Syntax Tree (AST)
    - We'll see at least one other way to model programs where you can combine
      multiple statements next time.

      -----------------------------------
*/

/*
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

    Exercise:
    - generalize our language to include a more realistic subset of programs.
    - (you may fix/modify the existing syntax if needed)
*/

// #[derive(Debug, PartialEq, Eq)]
// enum NotQuiteAsSillyV2 {
//     Increment(usize), // Increment the program counter by n
//     Print(String),    // Display a string
//     HaltIf(usize),    // Halt if program counter is at least n
//                       // add here ...
// }

// TODO: Let's write a simple example program using the new syntax

// fn example_program() -> NotQuiteAsSillyV2 {
//     unimplemented!()
// }

/*
    Semantics:

    Recall: We saw that the semantics is so far described "informally" in English, but this can lead to
    ambiguity (previous polls)
    We need *formal semantics* for our programs.

    So, someone has written a program in our language. We're getting
    some adoption, great! After a while, we have a small handful of users.

    But a debate begins between two of our users about whether a particular program
    is correct (or how it behaves).

    How would you settle the debate?

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

    Potentially relevant definitions:

        - An **interpreter** is ...

        - A **reference interpreter** is ...

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
    Exercise: demonstrate one of the problems from last time's (or today's) poll.
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
    === Discussion questions ===

    1. Why separate syntax and semantics?

    2. What is semantics good for?

    3. How do syntax and semantics play into the development of...

        compilers?

        interpreters?

        static analysis tools?
*/
