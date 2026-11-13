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

    ----- Picking up here for 11/13 -----

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

        1. x = 5
        2. WHILE x < 10
            3. x += 1
            4. y += 1

        // what do I know here?
        I know x is a number >= 10
        // How to determine x == 10?
        // ^^^ this requires an additional invariant!
        // we would need to determine that x <= 10 throughout
        // the program

        // This requires a static analysis!

        5. x += 1
        6. output/return (x, y)

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

        ----> We will often want to use a very specififc IR
              known as the control-flow graph (CFG)

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

use std::collections::HashMap;

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
    Extended exercise:
    write a constant propagation algorithm for this language.

    We will give some code, and a rough sketch of how this works
    for the rest.

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

#[derive(Debug, Clone)]
pub enum CFGNode {
    // Assignment: x := IntExpr
    // Include index of next CFG node to go to
    Assignment(Variable, IntExpr, usize),
    // Branch: includes indices of *both* other CFG nodes to go to
    Branch(BoolExpr, usize, usize),
    // Terminate program
    End,
}

#[derive(Debug, Default, Clone)]
pub struct CFG {
    // Graph data structure
    // Nodes: example [ node0, node1, node2, node3, node4 ]
    nodes: Vec<CFGNode>,
}

impl CFG {
    pub fn new() -> Self {
        Default::default()
    }
}

// Side note:
// to make the compiler more efficient we could use lifetimes
// and have CFG point back into the original WhileProg.
// pub struct CFG<'a> { ... }
// This is a bit harder to do... it reduces cloning
// it also helps with error messages,
// in case you want to report an error by reporting back a line of the original
// program.
// We will not worry about this

#[derive(Debug, Default)]
pub struct WhileProgCompiler {
    // CFG
    cfg: CFG,

    // Info for analyzed CFG
    // Backward edges: 1 <- 0, 2 <- 1, etc.
    edges_bck: Vec<Vec<usize>>,
    // Is constant:
    is_const: Vec<HashMap<Variable, bool>>,
}

impl WhileProgCompiler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn compile(&mut self, prog: &WhileProg) -> String {
        // Separated out into stages:
        self.compile_cfg(prog);
        self.analyze_cfg();
        self.optimize_cfg();
        self.compile_bin()
    }

    pub fn compile_cfg(&mut self, prog: &WhileProg) {
        self.compile_cfg_core(prog);
        self.cfg.nodes.push(CFGNode::End);
    }

    // Allow a Clippy lint
    #[allow(clippy::collapsible_if)]
    pub fn compile_cfg_core(&mut self, prog: &WhileProg) {
        match prog {
            WhileProg::Assignment(var, expr) => {
                let idx = self.cfg.nodes.len();
                // NOTE: why idx + 1?
                let node = CFGNode::Assignment(var.clone(), expr.clone(), idx + 1);
                self.cfg.nodes.push(node);
            }
            WhileProg::Seq(prog1, prog2) => {
                self.compile_cfg_core(prog1);
                self.compile_cfg_core(prog2);
            }
            WhileProg::If(cond, prog1, prog2) => {
                // *** Interesting case #1 ***

                // dummy node
                // will fill in values at the end
                let cond_idx = self.cfg.nodes.len();
                let cond_node = CFGNode::Branch(cond.clone(), 0, 0);

                // compile sub-programs
                self.compile_cfg_core(prog1);
                let mid_index = self.cfg.nodes.len();
                self.compile_cfg_core(prog2);
                let end_index = self.cfg.nodes.len();

                // In prog1: have to fill in any edges to the end of the program that point
                // to mid_index to point to end_index
                for node in self.cfg.nodes.iter_mut() {
                    // Some Rust pattern matching!
                    if let CFGNode::Assignment(_, _, idx) = node {
                        if *idx == mid_index {
                            *idx = end_index;
                        }
                    } else if let CFGNode::Branch(_, _, idx) = node {
                        if *idx == mid_index {
                            *idx = end_index;
                        }
                    }
                }

                // now, replace dummy node
                let new_node = CFGNode::Branch(cond.clone(), cond_idx + 1, mid_index);
                self.cfg.nodes[cond_idx] = new_node;
            }
            WhileProg::While(cond, prog1) => {
                // *** Interesting case #2 ***

                // dummy node
                let start_idx = self.cfg.nodes.len();
                let cond_node = CFGNode::Branch(cond.clone(), 0, 0);

                // compile sub-program
                self.compile_cfg_core(prog);
                let end_idx = self.cfg.nodes.len();

                // In prog1: have to fill in any edges to the end of the program that point
                // to end_index to point to start_index
                for node in self.cfg.nodes.iter_mut() {
                    // Rust pattern matching again
                    if let CFGNode::Assignment(_, _, idx) = node {
                        if *idx == end_idx {
                            *idx = start_idx;
                        }
                    } else if let CFGNode::Branch(_, _, idx) = node {
                        if *idx == end_idx {
                            *idx = start_idx;
                        }
                    }
                }

                // Replace dummy node
                let new_node = CFGNode::Branch(cond.clone(), start_idx + 1, end_idx);
                self.cfg.nodes[start_idx] = new_node;
            }
        }
    }

    pub fn analyze_cfg(&self) -> CFG {
        /*
            SKETCH:

            - First, compute back-edges for each forward edge in the CFG.
            So, if we have a transition
                node1 --> node2

            We should add an entry to self.bck_edges for
                node2 --> node1.

            - Second, we iterate over the nodes in the CFG:

                1. For each variable, if it is constant along all back edges, it remains constant

                2. After an assignment, if all variables involved in the integer expression
                   are constant, the result is constant.

                3. After a branch, in case of a condition of the form

                    Var == IntExpr

                    where IntExpr is Constant,

                    Var is now constant.
        */
        unimplemented!()
    }

    pub fn optimize_cfg(&self) {
        /*
            SKETCH:

            Now go through each node in the CFG and **replace** any occurrences
            of all variables in:

                Var := IntExpr

            or IntExpr inside Boolean expressions with their constant equivalents.
        */
        unimplemented!()
    }

    pub fn compile_bin(&self) -> String {
        /*
            Produce a working binary from the source program.

            Or, produce Rust code.
            SKIP in class.
        */
        unimplemented!()
    }
}

/*
    === A folk result about WHILE ===

    Theory exercise
    (SKIP/BRIEF):

    A "folk result": WHILE all programs can be reduced to a WHILE
    program with only a single WHILE loop.

    Why?
*/

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
    RECAP

    Concluding thoughts on optimizing compilers:

    - In order to build an optimizer, we want to generate an IR and operate on that!

        An incredibly important and fundamentally useful IRs: a control flow graph (CFG),
        store information at each node.

    - Example common optimizations:
        Constant propagation
        Loop unrolling
        Dead code elimination
        Others :-)

    - WHILE language

        Interesting fact: any WHILE program can be reduced to an equivalent program
        involving only a single WHILE loop.

    - Building an optimizer requires knowing information at each node -- I.E., static analysis!

        So --> Static analysis is useful/important for compilers.
*/
