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

use std::collections::{HashMap, HashSet};

// re-use
pub use super::intro::Variable;

// Rust enums

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntExpr {
    // Zero,
    // One,
    Const(usize),
    Var(Variable),
    Plus(Box<IntExpr>, Box<IntExpr>),
    Times(Box<IntExpr>, Box<IntExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoolExpr {
    LessThan(IntExpr, IntExpr),
    Equal(IntExpr, IntExpr),
    // Add others: Not, And, Or, etc.
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
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

    /* Info for analyzed CFG */
    // Backward edges: 1 <- 0, 2 <- 1, etc.
    back_edges: Vec<Vec<usize>>,

    // vars
    vars: HashSet<Variable>,

    // Is constant: on all executions, before and after each CFG node
    is_const_pre: Vec<HashMap<Variable, usize>>,
    is_const_post: Vec<HashMap<Variable, usize>>,
}

impl WhileProgCompiler {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn compile(&mut self, prog: &WhileProg) -> CFG {
        // Separated out into stages:
        self.compile_cfg(prog);
        self.analyze_cfg();
        self.optimize_cfg();

        // TBD: produce binary or Rust code.
        // For now, just return the CFG
        // self.compile_bin()
        self.cfg.clone()
    }

    pub fn compile_cfg(&mut self, prog: &WhileProg) {
        self.compile_cfg_core(prog);
        // Finally: push end node
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
                // (NB: this can be done more efficiently)
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
                // to end_index to point back around to start_index
                // (NB: this can be done more efficiently)
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

    #[allow(clippy::collapsible_if)]
    pub fn analyze_cfg(&mut self) {
        /*
            SKETCH:

            - First, compute back-edges for each forward edge in the CFG.
            So, if we have a transition
                node1 --> node2

            We should add an entry to self.bck_edges for
                node2 --> node1.

            - Also, collect the set of all variables. We store only vars that are modified during the program.
        */

        // initialize
        self.back_edges = vec![vec![]; self.cfg.nodes.len()];

        for (i, node) in self.cfg.nodes.iter().enumerate() {
            match node {
                CFGNode::Assignment(var, _expr, j) => {
                    self.back_edges[*j].push(i);
                    self.vars.insert(var.clone());
                }
                CFGNode::Branch(_expr, j1, j2) => {
                    self.back_edges[*j1].push(i);
                    self.back_edges[*j2].push(i);
                }
                CFGNode::End => {}
            }
        }

        /*
            - Second, we iterate over the nodes in the CFG:

                1. For each variable, if it is constant along all back edges, it remains constant

                2. After an assignment, if all variables involved in the integer expression
                   are constant, the result is constant.

                3. After a branch, in case of a condition of the form

                    Var == IntExpr

                    where IntExpr is Constant,

                    Var is now constant.

            For loops, we can apply the above steps until reaching a fixpoint.
            Below we will just do it in two passes.
        */

        self.is_const_pre = vec![HashMap::new(); self.cfg.nodes.len()];
        self.is_const_post = vec![HashMap::new(); self.cfg.nodes.len()];

        const FIXPOINT_ITERS: usize = 2;
        for _ in 0..FIXPOINT_ITERS {
            for (i, node) in self.cfg.nodes.iter().enumerate() {
                // *** check cond 1 ***
                // add set of vars that is constant on all entries to the node
                // uses helper function below
                for var in &self.vars {
                    if let Some(val) = self.is_constant_along_all_backedges(var, i) {
                        if !self.is_const_pre[i].contains_key(var) {
                            self.is_const_pre[i].insert(var.clone(), val);
                        }
                    }
                }

                // *** check cond 3 ***
                for var in &self.vars {
                    if let Some(val) = self.is_forced_by_backedges(var, i) {
                        if !self.is_const_pre[i].contains_key(var) {
                            self.is_const_pre[i].insert(var.clone(), val);
                        }
                    }
                }

                // cases based on node
                match node {
                    CFGNode::Assignment(var, expr, j) => {
                        self.is_const_post[i] = self.is_const_pre[i].clone();
                        self.is_const_post[i].remove(var);
                        // *** check cond 2 ***
                        if let Some(result) = self.is_constant_expression(expr, i) {
                            self.is_const_post[i].insert(var.clone(), result);
                        }
                    }
                    _ => {
                        self.is_const_post[i] = self.is_const_pre[i].clone();
                    }
                }
            }
        }
    }

    // helper functions
    fn is_constant_along_all_backedges(&self, var: &Variable, node_i: usize) -> Option<usize> {
        let mut result = None;
        for &node_prev in &self.back_edges[node_i] {
            let val = self.is_const_post[node_prev].get(var).copied()?;
            if result.is_some() && result != Some(val) {
                // inconsistent constant along two edges
                return None;
            }
            result = Some(val);
        }
        result
    }
    fn is_forced_by_backedges(&self, var: &Variable, node_i: usize) -> Option<usize> {
        if self.back_edges[node_i].len() != 1 {
            return None;
        }
        let node_prev = self.back_edges[node_i][0];
        let CFGNode::Branch(expr, i1, i2) = &self.cfg.nodes[node_prev] else {
            return None;
        };
        let BoolExpr::Equal(expr1, expr2) = expr else {
            return None;
        };
        // Not-equal case: can't deduce anything
        // (if adding BoolExpr::Not, would need a case for this also)
        if !i1 != node_i {
            debug_assert_eq!(*i2, node_i);
            return None;
        };
        // Equal case
        let var = IntExpr::Var(var.clone());
        if expr1 == &var {
            self.is_constant_expression(expr2, node_prev)
        } else if expr2 == &var {
            self.is_constant_expression(expr1, node_prev)
        } else {
            None
        }
    }
    fn is_constant_expression(&self, expr: &IntExpr, node_i: usize) -> Option<usize> {
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

// Unit test to see if it worked
#[test]
fn test_optimizer() {
    // Example:
    // x := 5
    // x := x + 1

    let mut compiler = WhileProgCompiler::new();

    // Construct the program
    let x = Variable::new("x".to_string());
    let stmt1 = WhileProg::Assignment(x.clone(), IntExpr::Const(5));
    let stmt2 = WhileProg::Assignment(
        x.clone(),
        IntExpr::Plus(
            Box::new(IntExpr::Var(x.clone())),
            Box::new(IntExpr::Const(5)),
        ),
    );
    let prog = WhileProg::Seq(Box::new(stmt1), Box::new(stmt2));

    let cfg = compiler.compile(&prog);
    let expected_l0 = CFGNode::Assignment(x, IntExpr::Const(6), 1);
    let expected_l1 = CFGNode::End;
    let expected = vec![expected_l0, expected_l1];

    assert_eq!(cfg.nodes, expected);
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
