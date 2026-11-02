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
    ===== Syntactic sugar =====

    Exercise: generalize our RandProg to have a new construct.
    Compile it out by treating it as syntacitc sugar.

    Q: Can any existing constructs be viewed as syntactic sugar?
*/

// Import
// TODO: remove after used
#[allow(unused_imports)]
use super::intro::{RandProg as RandProgOld, RandProgCompiler as CompilerOld, Variable};

pub enum SyntacticSugar {
    // what should go here?
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

    Q: Can anything that our compiler does be viewed as platform-specific code?
*/

/*
    What happened? Any interesting observations?
*/

/*
    ===== IRs, Layers, and Compiler Passes =====

    A picture emerges...

    see extras/IR_picture.jpg

    Compiler designers know that such features occur frequently! And so they
    define compilers to work in a series of stages, or representations
    of the program.

    Def. An *intermediate representation* (IR) is ...

    Rust has many IRs that it works with.

    Most important:
        Source code -> HIR -> MIR -> LLVM IR -> binary

    Qs:

    - What are the advantages of this approach?
      Why is it more convenient to work with an IR
      than the original source language or target code?

    - Is this really necessary?
*/

/*
    === Exercises ===

    1. Add syntactic sugar to our language for 4-bit integers.
       Adapt and structure the compiler in such a way that the 4-bit integer logic is
       compiled out.

    2. Allow our language to be configured to different pseudorandom number generators
       at the back-end:
       - Rust thread RNG (default Rust random generator)
       - Always return Heads
       - Always return Tails.
*/
