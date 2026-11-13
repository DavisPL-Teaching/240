//!
//! Part 3: Compiler optimizations.
//!

/*
    Let's talk about compiler optimizations!

    In the poll:

        - maybe we want to optimize out code that involves only a single
          variable (x) and x is not later modified

        - maybe we want to replace a loop with its overall effect.

            x = 0
            WHILE x < 10:
                x := x + 1

            ^^ may want to replace this with

            x = 10.

    More abstractly:

        - dependencies between pieces of my program:

            How can I tell that one piece of my program, say P1,
            has no effect on another piece of my program, P2?

        - equivalences between programs:

            How can I replace a part of my program, P1,
            with another equivalent program P2?
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
    POLL
    https://forms.gle/7H7rQrUfZkPVQxjZA

    Consider the following WHILE program:
        1. x = 5
        2. WHILE x != 10
            3. x += 1
            4. y += 1
        5. x += 1
        6. output/return (x, y)

    Consider each program point to be **before** that instruction
    is run.

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

    ----- Picking up here for 11/3 -----

    Answers to last time's poll:

    program point: 5, 6

    program point: None

    (Questions? Many of you got the second one right! but I saw less correct answers
    on the first one)

    This brings up two important challenges!

    1) To build a compiler optimization, we need to figure out
        **what is true** at each program point.

    2) This will sometimes require complex reasoning about the program!

    EXAMPLE: replace x != 10 with x < 10. What happens?

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

        + A naive compiler will simply not notice the optimization.

        + Better compiler: requires static analysis!!

    So we will see that static analysis is the foundation which enables
    many program optimizations
*/

/*
    ===== A very brief introduction compiler optimization =====

    Remember: a compiler optimization is carried out on one of the intermediate representation
    layers (one of the IRs)
        ---> usually at the middle point, or simplest layers

    It should replace code for equivalent code!

    Recipe to build your own compiler optimization:

    0. Construct the control flow graph (CFG) (see HW2 and below!)

        -- convenient way to divide program into relevant blocks of program points

        -- can do this one of two ways: one node per line, or one node per basic block
            (for definition of basic blocks - see HW.)
            (we will just do one node per line of source code below.)

    1. Figure out **what information is needed** at each program point

    2. Figure out how to write an algorithm to compute that information

    3. Use the additional information to replace a part of your program
       with a different program --
       **ensuring that it preserves the semantics** of the original program.

    === Example: Constant Propagation for the WHILE language ===

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
    While(BoolExpr, Box<WhileProg>),
}

/*
    Goal: write a constant propagation algorithm for this language.

    What is constant propagation?

    Example:

        x := 5
        x := x + 1

    Want to optimize to:

        x := 6.
*/

/*
    Sketch:
*/

#[derive(Debug, Default)]
pub struct WhileProgCompiler {
    // todo
}

#[derive(Debug, Clone)]
pub enum CFGNode {
    // Assignment: x := IntExpr
    // Include index of next CFG node to go to
    Assignment(Variable, IntExpr, usize),
    // Branch: includes indices of other CFG nodes to go to
    Branch(BoolExpr, usize, usize),
    // Terminate program
    End,
}

#[derive(Debug, Clone)]
pub struct CFG {
    nodes: Vec<CFGNode>,
}

// Side note:
// to make the compiler more efficient we could use lifetimes
// and have CFG point back into the original WhileProg.
// pub struct CFG<'a> { ... }
// This is a bit harder to do... it reduces cloning
// it also helps with error messages,
// in case you want to report an error by reporting back a line of the original
// program.

impl WhileProgCompiler {
    pub fn new() -> Self {
        Self {}
    }
    pub fn compile(&mut self, prog: &WhileProg) -> String {
        // Separated out into stages:
        let cfg = self.compile_cfg(prog);
        let mut analyzed = self.analyze_cfg(&cfg);
        self.optimize_cfg(&mut analyzed);
        self.compile_bin(&analyzed)
    }

    pub fn compile_cfg(&self, prog: &WhileProg) -> CFG {
        let mut nodes = Vec::new();
        self.compile_cfg_core(prog, &mut nodes);
        nodes.push(CFGNode::End);
        CFG { nodes }
    }
    pub fn compile_cfg_core(&self, prog: &WhileProg, nodes: &mut Vec<CFGNode>) {
        match prog {
            WhileProg::Assignment(var, expr) => {
                // TODO
                unimplemented!()
                // let node = CFGNode::Assignment(var.clone(), expr.clone());
                // nodes.push(node);
            }
            WhileProg::Seq(prog1, prog2) => {
                self.compile_cfg_core(prog1, nodes);
                self.compile_cfg_core(prog2, nodes);
            }
            WhileProg::If(cond, prog1, prog2) => {
                // Interesting case #1

                // dummy node
                // will fill in values at the end
                let cond_idx = nodes.len();
                let cond_node = CFGNode::Branch(cond.clone(), 0, 0);

                // compile sub-programs
                self.compile_cfg_core(prog1, nodes);
                let mid_index = nodes.len();
                self.compile_cfg_core(prog2, nodes);

                // now, replace dummy node
                nodes[cond_idx] = CFGNode::Branch(cond.clone(), cond_idx + 1, mid_index);
            }
            WhileProg::While(cond, prog1) => {
                // Interesting case #2

                // TODO: complete
                unimplemented!()
            }
        }
    }

    pub fn analyze_cfg(&self, cfg: &CFG) -> CFG {
        unimplemented!()
    }

    pub fn optimize_cfg(&self, cfg: &mut CFG) {
        unimplemented!()
    }

    pub fn compile_bin(&self, cfg: &CFG) -> String {
        unimplemented!()
    }
}

/*
    === Poll ===

    Consider the following WHILE program
    (Same program from last time poll)

    Which of the following potential program optimizations is valid, i.e.,
    preserves the semantics of the original program? (Select all that apply)

    <Options cut>

    https://forms.gle/yh8tGG68x2YEJ44m7
*/

/*
    Proof exercise:

    A "folk result": all programs can be reduced to a WHILE
    program with only a single WHILE loop.
*/
