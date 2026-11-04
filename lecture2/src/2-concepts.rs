/*
    Lecture 2, part 2:
    Compiler concepts
*/

/*
    === Poll ===

    An interpreter has more information than a compiler: it has a program *and* an intended input on which that program is run. Therefore it could perform more optimizations than a compiler, including optimizing it for that specific input. Therefore, interpreters are more useful than compilers for optimization.

    What is wrong with this reasoning?

    https://forms.gle/bLGhNKSQUWxmRjdH7
*/

/*
    --- Nov 4

    Emphasize from last time's poll:
    Idea of compilers! ---> **allow** ourselves to spend more time at compile time,
    to optimize a program for many different inputs.
    Separate out compile time vs. runtime --
    we allow ourselves to **trade** additional time spent during compilation
    (compile-time cost) for better performance at runtime.

        (Code is written once -- run many times in production on different inputs)

    Otherwise there's no point!
    There would be no point in developing compilers without distinction.

    Some of your answers said that it would not be possible, or harder, to optimize
    a program with a specific input. That is not the case. We will actually see later
    this is involved in a very common and important program optimization, known
    as constant propagation.
    (Dates back to the very early days of the field, 1966 paper.)

    ===== Layered compiler development and IRs =====

    Last time: Complexity in compiler implementation leads to
    *layered* development of a compiler, with multiple passes.

    Basic compiler schematic:
        M, S, T syntax and semantics (metalanguage, source language, target language)

        Metalanguage : M

        Source language:                              Target language and platform:
        S                          ->                 T

    With layers:

        Metalanguage : M

        Source language:                              Target language and platform:
        S                 -> I1 -> I2 -> I3 -> I4 ->  T

    Abstraction:

        ^^^High-level                          ^^^^ Low-level

    Complexity:

        HIGH  .... decreases ....  LOW .... increases .... HIGH

    Each intermediate layer I1, I2, I3, is called an **Intermediate Representation** (IR).

    IR comes with its own syntax and semantics! Each of these is itself a language.

    ===== Syntactic sugar =====

    Def. Syntactic Sugar

        Syntactic sugar is source-level syntax that is immediately compiled out on the first
        pass, and "stands in" for some other language constructs with identical semantics.

    === Poll ===

    Which of the following is a correct example of syntactic sugar?
    (Select all that apply)

    <Options cut>

    https://forms.gle/1FLSE5AkG3KpGmMh8

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

    https://forms.gle/36KfMe2QNswg5atG8

    .
    .
    .

    Exercise: generalize our RandProg to have a new construct.
    Compile it out by treating it as syntactic sugar.

    Q: Can any existing constructs be viewed as syntactic sugar?

    Below:
    an example of adding syntactic sugar for a SwapVars construct.

        In Python: x, y = y, x.
*/

// Import
// TODO: remove after used
#[allow(unused_imports)]
use super::intro::{RandProg, RandProgCompiler, Variable};

#[derive(Debug)]
pub enum SyntacticSugar {
    SwapVars,
    Base(RandProg),
}

#[derive(Debug, Default)]
pub struct SyntacticSugarCompiler {
    // what should go here?
}

impl SyntacticSugarCompiler {
    pub fn new() -> Self {
        unimplemented!()
    }
    pub fn compile(&mut self, prog: &SyntacticSugar) {
        // TODO: what should be the output?
        // what should go here?
        // TODO: implement
        unimplemented!()
    }
}

/*
    What happened? Any interesting observations?

*/

/*
    ===== Platform-specific code =====

    Def.:
    Platform = OS, Architecture, and installed libraries that code depends on.

    Example:
    We could write different compilers, one which produces output
    to the terminal; and one which returns output by displaying it in a dialogue box to the
    user; and yet another which produces output by saving it to a file.

    OR:
    Logic for displaying output to a file changes depending on the platform
    (e.g.: use CRLF on Windows, LF on Linux).

    SKETCH:
*/

#[derive(Debug)]
pub enum PreRustIR {
    RustCode(String),
    OutputCommand(Variable),
    Seq(Box<PreRustIR>, Box<PreRustIR>),
}

#[derive(Debug, Default)]
pub struct PlatformSpecificCompiler {
    // what should go here?
}

impl PlatformSpecificCompiler {
    pub fn new() -> Self {
        // what should go here?
        unimplemented!()
    }
    pub fn compile(&mut self, prog: &PreRustIR) {
        // TODO: what should be the output?
        // what should go here?
        // TODO: implement
        unimplemented!()
    }
}

/*
    What happened? Any interesting observations?
*/

/*
    ===== IRs and Compiler Passes =====

    IR complexity:

        see extras/IR_picture.jpg

    Compiler designers know that such features occur frequently! And so they
    define compilers to work in a series of stages, or representations
    of the program.

    Rust has many IRs that it works with. AMong these are:

        Source code -> HIR -> MIR -> LLVM IR -> binary
        ^^^High-level                          ^^^^ Low-level

    If you want to optimize the program, or analyze to figure something out about it ...
    you want to work at the "lowest complexity" level that has the information you need!
    (NOT necessarily the lowest level or highest level in the layers.)

    Why?
*/

/*
    === Exercises ===
    (SKIP)

    1. Add syntactic sugar to our language for 4-bit integers.
       Adapt and structure the compiler in such a way that the 4-bit integer logic is
       compiled out.

    2. Add some "platform-specific code" for random number generation by
       allowing our language to be configured to different pseudorandom number generators
       at the back-end:
       - Rust thread RNG (default Rust random generator)
       - Always return Heads
       - Always return Tails
       - On Unix systems, use Unix's /dev/random for RNG rather than the built in Rust thread RNG.
*/
