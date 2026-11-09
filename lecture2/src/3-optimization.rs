//!
//! Part 3: Compiler optimizations.
//!

/*
    Let's talk about compiler optimizations!
*/

/*
    Some classical compiler optimizations

    (A bit of historical context -
        "Program Optimization," Frances Allen 1966

        Some of the first few pages are in extras/Allen_1966_Program_Optimization.pdf

        I can't find these on the internet, I had to track
        them down by checking out a book from the library
        and taking pictures.

        The optimizations Fran Allen mentions are:

        1. Eliminating redundant instructions
        2. Replacing variables with values (now known as constant propagation)
            (-- related to Nov 2 poll!)
        3. Reordering instructions
        4. Renaming/replacing definitions
        5. Replacing sequences of instructions with a single instruction
        6. Eliminating unused definitions and code (now known as dead code removal)

        Allen won the 2006 Turing Award
        "For pioneering contributions to the theory and practice of optimizing compiler techniques that laid the foundation for modern optimizing compilers and automatic parallel execution."
    )
*/

/*
    === The WHILE language ===

    The following language, WHILE, is introduced on your HW Problem 5.

    WHILE syntax:
        Var ::= x | y | z
        IntExpr ::= 0 | 1 | Var
                    | (IntExpr + IntExpr) | (IntExpr * IntExpr)
        BoolExpr ::= IntExpr < IntExpr | IntExpr == IntExpr
        Prog ::= Var := IntExpr
                | Prog ; Prog
                | If { BoolExpr } Then { Prog } Else { Prog }
                | While { BoolExpr } Do { Prog }

    Some Rust code:
*/


// re-use
pub use super::intro::Variable;

// Rust enums

#[derive(Debug, Clone)]
pub enum IntExpr {
    Zero,
    One,
    Var(Variable),
    Plus(Box<IntExpr>, Box<IntExpr>),
    Times(Box<IntExpr>, Box<IntExpr>),
}

#[derive(Debug, Clone)]
pub enum BoolExpr {
    LessThan(IntExpr, IntExpr),
    Equal(IntExpr, IntExpr),
}

#[derive(Debug, Clone)]
pub enum WhileProg {
    Assignment(Variable, IntExpr),
    Seq(Box<WhileProg>, Box<WhileProg>),
    If(BoolExpr, Box<WhileProg>, Box<WhileProg>),
}

/*
    Goal: write a constant propagation algorithm for this language.

    Example:

    How do we do this?
*/

/*
    POLL
    https://forms.gle/7H7rQrUfZkPVQxjZA

    Consider the following WHILE program:
        1. x = 5
        2. WHILE x != 10
            3. x += 1
            4. y += 1
        5. x += 1

    At which program points is x known to be constant?
    At which program points is y known to be constant?

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
    Programming exercise: write a compiler for constant propagation
*/

#[derive(Debug, Default)]
pub struct WhileProgCompiler {
    // todo
}

impl WhileProgCompiler {
    pub fn new() -> Self {
        Self {}
    }
    pub fn compile(&mut self, prog: &WhileProg) -> String {
        // TODO
        // Separate into two stages.
        unimplemented!()
    }
}

/*
    === Poll ===

    TBD

    https://forms.gle/ZxSeKa3cxytTxE2X9
*/

/*
    Proof exercise:

    A "folk result": all programs can be reduced to a WHILE
    program with only a single WHILE loop.
*/
