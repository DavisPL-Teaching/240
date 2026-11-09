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

                                    C: M
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

        see extras/IR_picture.jpg

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
    .
    .
    .

    Exercise: generalize our RandProg to have a new construct.
    Compile it out by treating it as syntactic sugar.

    Q: Can any existing constructs be viewed as syntactic sugar?

    A: Yes,
            CopyVar(x, y) // x = y
            could be syntactic sugar for

            If(y, SetVar(x, true), SetVar(x, false))

    Below:
    an example of adding syntactic sugar for a SwapVars construct.

        In Python: x, y = y, x.
*/

// Import
// TODO: remove after used
#[allow(unused_imports)]
use super::intro::{RandProg, RandProgCompiler, Variable};

// NEW LANGUAGE: RandProg + some Syntactic Sugar
#[derive(Debug)]
pub enum SyntacticSugar {
    SwapVars(Variable, Variable), // in Python: x, y = y, x
    Base(RandProg),
    // TODO add: Seq, If, etc.
}

#[derive(Debug, Default)]
pub struct SyntacticSugarCompiler {
    core_compiler: RandProgCompiler,
}

impl SyntacticSugarCompiler {
    pub fn new() -> Self {
        Self {
            core_compiler: RandProgCompiler::new(),
        }
    }
    pub fn compile(&mut self, prog: &SyntacticSugar) -> String {
        // TODO: what should be the output?
        // what should go here?
        // TODO: implement
        let rand_prog = match prog {
            SyntacticSugar::SwapVars(v1, v2) => {
                // Idea: compile to RandProg first
                // Write a RandProg program for swap
                // idea: if ( v2 ) ( v2 = v1; v1 = true ) else ( v2 = v1; v1 = false )
                // let v1 = v1.clone();
                // let v2 = v2.clone();
                RandProg::If(
                    v2.clone(),
                    Box::new(RandProg::Seq(
                        Box::new(RandProg::CopyVar(v2.clone(), v1.clone())),
                        Box::new(RandProg::SetVar(v1.clone(), true)),
                    )),
                    Box::new(RandProg::Seq(
                        Box::new(RandProg::CopyVar(v2.clone(), v1.clone())),
                        Box::new(RandProg::SetVar(v1.clone(), false)),
                    )),
                )
            }
            SyntacticSugar::Base(b) => b.clone(),
        }; // return a RandProg
        self.core_compiler.compile(&rand_prog)
    }
}

/*
    It works! (Hopefully)

    Compiler now has three layers:

        SyntacticSugar --> RandProg --> Rust

    What happened? Any interesting observations?

    Recap:

    - We defined layered compiler development - and
        intermediate representations (IR), which are a type of language
        with their own syntax and semantics and each layer

    - Syntactic sugar = syntax that is immediately compiled out at the very first
      layer and replaced with identical semantics

    - Other end: Platform-specific code which will increase complexity at the back end,
      or lowest level

    - GOAL: simplify as much as possible the complexity going into the inner layers,
      so that we can more easily write optimization and analysis passes.
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
*/

/*
    Coding example --

    SKETCH:

        SyntacticSugar -> RandProg -> PreRustIR -> Rust
*/

#[derive(Debug)]
pub enum PreRustIR {
    RustCode(String),
    OutputCommand(Variable),
    Seq(Box<PreRustIR>, Box<PreRustIR>),
}

// placeholders

#[derive(Debug, Default)]
pub struct SyntacticSugarCompiler2();

#[derive(Debug, Default)]
pub struct RandProgCompiler2();

#[derive(Debug, Default)]
pub struct CRLFCodeGenerator();

#[derive(Debug, Default)]
pub struct PlatformSpecificCompiler {
    // Compiler from SyntacticSugar -> RandProg
    syntactic_sugar_compiler: SyntacticSugarCompiler2,
    // Compiler from RandProg -> PreRustIR
    rand_prog_compiler: RandProgCompiler2,
    // Compiler from PreRustIR -> CRLFCodeGenerator
    crlf_code_generator: CRLFCodeGenerator,
}

impl PlatformSpecificCompiler {
    pub fn new() -> Self {
        // what should go here?
        unimplemented!()
    }
    pub fn compile(&mut self, prog: &PreRustIR) -> String {
        // TODO: what should be the output?
        // what should go here?
        // TODO: implement
        unimplemented!()
        // prog1 = self.syntactic_sugar_compiler.compile(prog);
        // prog2 = self.rand_prog_compiler(prog);
        // ---- can do optimization passes here
        // prog3 = self.crlf_code_generator(prog2);
        // prog3
    }
}

/*
    What happened? Any interesting observations?
*/

/*
    ===== POLL =====

    In some low-level languages, like C and Rust,
    the number of bits of an integer or pointer can be platform-specific.
    For example, in Rust, the number of bits in

        usize
        isize

    is equal to the number of bits needed to index memory addresses on the
    particular platform.

    Consider the following program (pseudocode):

        x = 0
        While x < 2^32 :
            x += 1; // wrapping add
        print "Done";

    Suppose that we generate platform-specific code -- optimizing the program
    in a platform-specific way -- where the number of bits
    that is used to store x is either 32-bit or 64-bit, depending on the platform.

    a) Give a shorter, simpler program (without loops) that this program could
       be optimized to on 64-bit platforms.

    b) Give a shorter, simpler program (involving at most one, simpler loop) that
       this program could be optimized to on 32-bit platforms.

    c) (If your answers to part a and b are different)
       The behavior of the code depends on the platform!
       Yet, in the source language semantics, we probably need to state what the
       program should do.
       How is this not a problem for program semantics?
       :-)

       How do you think programming languages like C and Rust define the semantics
       to avoid this issue?

    Form link:
    https://forms.gle/pYp3QdYkp3YfrkWZ6

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

    Answers:
    a) print "Done"
    b) while True

    Tried it?
        - gcc with sufficient optimizations removes the loop entirely
          for both 32 bit and 64 bit platforms
        - rustc: try with --release and report back.

    Q: why does gcc compile the second one by removing the loop?

        x is not read/written to

        but removing the loop changes the semantics!

        how is this allowed?

        Hint: C semantics is nondeterministic...

        A: The program has undefined behavior
        So, C is allowed to remove the infinite loop.

        - Integer overflow is UB

        - In some languages, infinite loops are UB.

        (In C, I can actually optimize the loop in both cases.)

        Rust cannot do this because safe Rust is not allowed to have UB.

    c)
        A: nondeterminism!

        More generally if you see a scenario where:
        here's a program P and it can be validly compiled in two different ways,
        probably the answer as to why this is happening is either
        nondeterminism or UB (a form of nondeterminism).
*/

/*
    ===== IRs and Compiler Passes (Recap) =====

    For all of the above reasons of a "refactorable"/modular design and to manage complexity -
    compiler designers define compilers to work in a series of stages,
    or representations of the program.

    Rust has many IRs that it works with. Among these are:

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
    (These will appear on the HW2 problem set.)

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
