//!
//! Introduction to static analysis:
//!
//! Part 1: Introduction and main concepts.
//!

/*
    POLL

    (This will tie into today's lecture)

    Which of the following is true about optimization for WHILE programs? (Select all that apply)

    <Options cut>

    https://forms.gle/qrdUHMGXKnzwFfsMA

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
    === What is static analysis? ===

    Static analysis is determining semantic properties
    of a program (or a program's behavior), without running it.

    There are two main motivations for static analysis:

    1. We saw in last lecture: that in order to perform optimization, we need information about the program!

        ---> Typically of the form: given a program (say in WHILE), P, and a point in that program (say, line n),
             what properties/information are true at line n?

    This is what static analysis gives us!

    2. The second motivation is less about compiler optimization, and more about safety and correctness:

        we want our programs to be safe to execute! (Not raise errors, not have bad pointer access or
        undefined behavior, not have crashes or privacy/security bugs, etc.)

        We can use static analysis to try to determine these properties of the programs before running them

        (this is natural! - as deploying and running them directly might be dangerous.)

    However, generally solving a static analysis problem --
    for **all** possible programs -- is often impossible. (!)
    We saw examples of this on the poll - more on this below.

    So, we often resort to an approximate answer -- we either
    **overapproximate** or **underapproximate** the set of actual program behaviors.

    ---------- where we ended for 11/16 ----------

    Last time:

    - Static analysis: determine some properties of a program (often, at a specific program point), without running it

    - Motivation for static analysis

    - A little about undecidability: no "perfect" algorithm for most static analysis problems!

    === POLL ===

    Practice about understanding: Turing completeness + static analysis.

    Consider the following language, FooLang, that has the following grammar.

    (We will consider some static analysis problems for FooProg.)

    FooProg ::= LoopForever { FooProg } | LoopTenTimes { FooProg } | Output <Var> | Input <Var> | Set <Var> := <Int> | FooProg; FooProg

    Assume the semantics of these commands is as you would expect (e.g., looping means to repeat the loop body either forever or repeat it 10 times.)

    1. Is FooLang Turing-complete?

    2. Which of the following static analysis problems do you think are decidable (that is: there exists an algorithm to solve the problem completely & precisely) for FooLang?

    a) Dead code elimiination: given `prog` and a line number, is that line number reachable on any execution?
    b) Constant propagation: given `prog`, `Var`, and a line number, is the variable always equal to the same value at that line number?

    https://forms.gle/XGdChvVqVMwBWBy9A
*/

/*
    Some terminology

    Many static analyses output a **set** of results that are actual possible program behaviors.

        Constant propagation: set of variables that are constant at a point (program behavior = variable is constant)

        Dead code elim: set of program points that are reached (program behavior = reaching that point)

    For a program P, let 𝓕(P) be the set of results that the static analysis
    is supposed to compute.

    A static analysis is **sound** if it returns an answer

        Y ⊇ 𝓕(P)

    and **complete** if it returns an answer

        Y ⊆ 𝓕(P).

    It is generally impossible for a static analysis to be both sound and complete! (!)

    We thus resort to either **overapproximation** (sound, but not complete) or
    **underapproximation** (complete, but unsound) of the possible program behaviors.

    Anders Møller refers to this as giving "Approximative Answers".
    The idea is that a static analysis can give us useful behavior by either underapproximating
    or overapproximating the set of bugs we are interested in.
*/

/*
    Example: interval analysis

    Task:

        Given a program, and a program point in that program (line number),
        and a variable (say, x)
        determine the interval of values that are possible values of x
        at that program point.

    To each variable, we associate: an interval of values

        [min, max]

    that it might take on at a specific point in the program.
    These can be ± ∞.

    We want a sound analysis!

        --> Actual values of the variable should be a subset of [min, max]

    But! We will expect that having a complete analysis is not possible in general.
    So we want to report an **overapproximation** of the true interval.

    One thing we could do is go through our program and
    iteratively update these intervals [min, max]
    based on what values each variable seems to be able to take on
    based on the surrounding context.

    But! We may get stuck forever. Example:

    i = 0
    WHILE (i >= 0):
        [0, 3]
        i := i + 1
        ^^^^ what are possible values of i at this point
    return i

    Analysis: maybe we try running the loop and updating possible intervals
    as we go along ...

    BUT we might not terminate.

    Q: How to fix this problem?

    ---> Look at any branch conditions to see what interval they imply,
         if no branch conditions that create a smaller value, stop updating
         the interval?

         SUGGESTION: Modify to positive infinity
            I.e.: want to return [0, + ∞]

        Q: When?

            Idea: identify an overflow value!

                +100
                and
                -100

            If any of our intervals reach > 100 or < -100, replace them with ±∞

    (spoiler wall)
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

        IDEA: limit min and max to at most ± 10, OR ± ∞.

    EXERCISE:

    Write an interval analysis that gives sound over-approximations
    for integer expressions and Boolean expressions from WHILE.

    We will do this exercise below.
*/

// Similar Variable, IntExpr, BoolExpr from Lecture 2

use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Variable(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntExpr {
    // N.B.: I changed this to isize to allow any positive/negative integer literals
    Const(isize),
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

// Stores the relevant state for interval analysis!
// For simplicity, we'll use isize::MIN and isize::MAX for ± ∞.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct IntervalMap(HashMap<Variable, (isize, isize)>);

impl IntervalMap {
    pub fn new() -> Self {
        Default::default()
    }
    // Analyze int expr
    // TODO:
    // INPUT:
    // - self, an interval map (map from each var to an interval [min, max])
    // - iexp, an integer expression
    // OUTPUT:
    // - an interval [min, max]
    pub fn analyze_int_expr(&self, iexp: &IntExpr) -> (isize, isize) {
        // NB: We should be using saturating add/multiply in the below,
        // or special case the values isize::MIN and isize::MAX for this
        // to work.
        match iexp {
            IntExpr::Const(i) => (*i, *i),
            IntExpr::Var(v) => *self.0.get(v).unwrap(),
            IntExpr::Plus(iexp1, iexp2) => {
                let (min1, max1) = self.analyze_int_expr(iexp1);
                let (min2, max2) = self.analyze_int_expr(iexp2);
                (min1 + min2, max1 + max2)
            }
            IntExpr::Times(iexp1, iexp2) => {
                let (min1, max1) = self.analyze_int_expr(iexp1);
                let (min2, max2) = self.analyze_int_expr(iexp2);
                // this is a little more interesting
                let endpoints = [min1 * min2, min1 * max2, max1 * min2, max1 * max2];
                let min = *endpoints.iter().min().unwrap();
                let max = *endpoints.iter().max().unwrap();
                (min, max)
            }
        }
    }

    // Clamp function
    // This would be used to overapproximate the set to the full interval [-∞, +∞] after computing
    const INTERVAL_MIN: isize = -100;
    const INTERVAL_MAX: isize = 100;
    fn clamp(&self, (mut min, mut max): (isize, isize)) -> (isize, isize) {
        if min < Self::INTERVAL_MIN {
            min = isize::MIN;
        }
        if max > Self::INTERVAL_MAX {
            max = isize::MAX;
        }
        (min, max)
    }

    // Analyze bool expr
    // RETURN: two bools, could_be_true, could_be_false
    // TODO
    pub fn analyze_bool_expr(&self, i: &BoolExpr) -> (bool, bool) {
        unimplemented!()
    }
}

// write some unit tests here
// #[test]
// fn test_interval_analysis() {
//     unimplemented!()
// }

/*
    How is this useful to build an actual static analysis pass?

    Well...
*/

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhileProg {
    Assignment(Variable, IntExpr),
    Seq(Box<WhileProg>, Box<WhileProg>),
    If(BoolExpr, Box<WhileProg>, Box<WhileProg>),
    While(BoolExpr, Box<WhileProg>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct IntervalAnalysis {
    map: IntervalMap,
}

impl IntervalAnalysis {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // initialize all variables to [-∞, +∞] (or other values as desired to test other executions)
        unimplemented!()
    }
    pub fn eval(&mut self, prog: &WhileProg) {
        match prog {
            WhileProg::Assignment(v, e) => {
                // Analyze the expression, getting an interval
                let res = self.map.analyze_int_expr(e);
                // Clamp the result
                let res = self.map.clamp(res);
                // Insert it into the map
                self.map.0.insert(v.clone(), res);
            }
            WhileProg::Seq(prog1, prog2) => {
                self.eval(prog1);
                self.eval(prog2);
            }
            WhileProg::If(b, prog1, prog2) => {
                // Here we will need to fork the state
                // evaluate both branches, combine the results.

                // EXAMPLE
                // x: [15, 25]
                // if x > 20 {
                //
                //     x: [5, 17]
                // } else {
                //
                //     x: [0, 3]
                // }
                // x: [0, 17]

                // sketch
                let (maybe_true, maybe_false) = self.map.analyze_bool_expr(b);

                if maybe_true && maybe_false {
                    // clone the state
                    let mut other = self.clone();

                    // TODO: can also update the interval maps based on true and false case
                    // For now we ignore this

                    self.eval(prog1);
                    other.eval(prog2);

                    // Combine the states at the end
                    self.merge(other);
                } else if maybe_true {
                    // TODO: update interval maps based on true case
                    self.eval(prog1);
                } else if maybe_false {
                    // TODO: update interval maps based on false case
                    self.eval(prog2);
                } else {
                    // Neither branch reachable
                    // This may or may not be a bug, maybe the entire branch of code is dead.
                    eprintln!("warning: static analysis encountered unreachable branch");
                    self.set_unreachable();
                }
            }
            WhileProg::While(b, prog1) => {
                // This is the interesting case
                // It turns out that this will not necessarily terminate without clamping.

                // The code is something like this:

                let mut modified = true;
                while modified {
                    let (maybe_true, maybe_false) = self.map.analyze_bool_expr(b);
                    if maybe_true {
                        let self_before = self.clone();
                        self.eval(prog1);
                        self.merge(self_before.clone());
                        // check if reached fixpoint
                        modified = self != &self_before;
                    }
                }

                // EXAMPLE
                // x: [0, 10]
                // while x < 10 {
                //     x: [0, 9]
                //     x := x + 1
                //     x: [1, 10]
                // }
                // x: [10, 10]
                // x must be 10 here.

                // !
                // x: [1, ∞]
                // while x > 0 {
                //     x: [1, ∞]
                //     x += 1
                //     x: [1, ∞]
                // }
            }
        }
    }

    pub fn merge(&mut self, mut other: Self) {
        // Merges the two states, unioning all intervals.

        // drain = consume all values (producing them with ownership) from the hashmap
        for (var, (min2, max2)) in other.map.0.drain() {
            let (min1, max1) = self.map.0.get_mut(&var).unwrap();
            *min1 = std::cmp::min(*min1, min2);
            *max1 = std::cmp::max(*max1, max2);
        }
    }

    pub fn set_unreachable(&mut self) {
        // Set self to an unreachable state (all intervals empty)
        unimplemented!()
    }
}

/*
    Exercise: (SKIP)

    Add Not to our grammar above for BoolExpr, and adapt the static analysis.
*/

/*
    === Compiler full employment theorem ===
    (SKIP/BRIEF - We discussed this a bit on 11/16)

    **Rice's Theorem (informal):**
    For any nontrivial property of a program in a
    Turing-complete programming language,
    it is not possible to write an algorithm to determine
    that property perfectly.

    You don't need to know the formal statement of Rice's theorem or its proof.
    Please know its intuitive meaning.
    (See today and last time poll for practice!)

    Sometimes, this is humorously called the "compiler full employment theorem" since it means
    that if you work in compilers (or static analysis), you will never be out of a job.
    (I got that from Andrew Appel, but Appel wasn't the first to state this idea.)

    A little more seriously:
        undecidable doesn't mean unsolvable!

        Undecidable problems means we can't write a **general** algorithm which solves the problem
        on **all inputs**

        Well: who cares, right?
        Most programs that occur in practice are not difficult/impossible to solve. And,
        programs that occur in practice are not unbounded memory, and are not unbounded in size.
        There are algorithms that solve important problems for these specific "small" programs.
        And we can hope to build them.

        It is sometimes joked that in programming languages research, we
        solve undecidable problems all the time.
*/

/*
    === A general view of static analysis ===
    (BRIEF, time permitting)

    More generally, static analysis is intended to solve a task of the following form:

        Given a program `prog`, and a property of programs 𝓕, determine property 𝓕 for the program,
        without running it.

        (Often, but not always, a True-or-False property.)

        (NOTE: For those of you who have taken or are interested in taking ECS 261:
         this is exactly the same as a program specification.
         Program verification exists to prove that programs satisfy certain properties, it is related to static
         analysis in that static analysis aims to *automatically* verify certain specific classes of properties.)

    Example:

        Constant propagation
        𝓕 = which variables are constant in prog at line N?

        Dead code elimination
        𝓕 = is line N reached in `prog`?

    Simpler examples:

        𝓕 = Does program P terminate on all inputs?

        𝓕 = Does program P terminate on input (x, y, z) = (0, 0, 0)?

    And more...

        𝓕 = Does program P raise an exception or error?

        𝓕 = Does program P return an integer that is out of bounds?

        𝓕 = Does program P return a block of bytes that represents a valid C++ struct in memory?

        𝓕 = Does program P violate user's security/access control constraints,
                 or leak private user data?
*/

/*
    Recap and key points about static analysis:

    Static analysis = determine a property of the program without running it.

        1. **Without running it:** contrasts with dynamic analysis.

        Does not mean we can't consider what would happen if we did run it! But we don't want to run
        it directly (would result in performance loss + possible infinite loop)

        2. May be a property of the program **on all inputs**

            Not just on some!

            Again contrasts with dynamic analysis + unit tests + fuzzing -- always test a property on
            one input at a time only.

            If testing behavior on one input: dynamic analysis is often more effective

        3. These problems are hard (typically undecidable) to solve in general,
            as we have seen above.
            So we often resort to an approximation.
*/
