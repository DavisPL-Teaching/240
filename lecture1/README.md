# Lecture 1: Foundations

### Monday, Sep 28

Announcements:

- HW0 now available: due in 1 week (Monday, Oct 5)

    + Rust installation/tutorial

    + For help: my OH Wed; TA OH Friday
      (no OH planned on Monday! but please use Piazza)

- Results of Poll (34 responses)

    28 Foundations of PL
    23 Compilers and static analysis
    23 Testing and verification
    21 Advanced topics: concurrent and parallel programming languages, etc.
    14 Type systems
    11 DSLs

Plan:

- Begin Lecture 1: Foundations of programming languages

    + Part 1: `1-intro.rs`: introduction to syntax and semantics

    + Along the way: learn a few things about Rust (where needed)

- How to follow along

- Poll.

### Wednesday, Sep 30

Reminders:

- HW0 due Monday (Oct 5)

    + My OH: today 4:15p, ASB 2085

    + TA OH: Friday 2pm (after class)

Plan:

- Questions about HW0

- Begin part 2: More on syntax and semantics.

    + starting with: a side note about Rust & Rust syntax, then, today's poll.

Questions about HW0 or Rust?

## Friday, Oct 2

Reminders:

- HW0 due Monday (Oct 5)

- TA OH after class

Plan:

- Questions about HW0

- Short addendum from last time (+ Piazza post)

- Start with the poll

- Continue Part 2

HW0 question:

    `-> ()`

If Clippy doesn't like your code after solving an exercise:
1. make the fix
2. Clippy will give you the code of the warning
    ```
    #[allow(clippy::warning_code)]
    fn () ...
    ```

## Monday, Oct 5

Announcements/reminders:

- HW0 due today

    + Please ignore autograder" output!
      We will configure the autograder following the assignment.

    + Check that `cargo build`, `cargo test`, `cargo clippy`, `cargo fmt` are happy with your code.
      Can also run: `cargo check`

      (No warnings! or errors)

- There will be **no class this Friday, Oct 9** (I will be away at a conference)

- Poll answers file now available: see `exam/poll_answers.md`

Plan:

- Continue syntax & semantics with our toy language

- Do the poll.

Questions?

## Wednesday, October 7

Reminders:

- No class on Friday

    Last time, I erroneously said I would not hold office hours,
    but my OH are on Wednesday! I will still hold OH.

    TA OH:
    Office hours remotely via Zoom (2-3pm Fri)

Plan:

    Semantics.

Questions?

## Monday, October 12

Announcements:

- HW0 grades are ready and will be released shortly

- I am working on HW1; I hope to release this soon

Plan for today:

- Continue semantics.

    + Determinism.
        * Examples
        * Why semantics may not be deterministic
        * Why we don't want semantics to be deterministic

    + Poll

    + Ambiguous vs. nondeterministic semantics

    + How semantics enables program properties: program equivalence, program optimization.

    + Revising the poll from Lecture 0.

## Wednesday, October 14

Announcements:

- HW0 graded

    Please take a look at your scores - regrade requests open for one week, until Tuesday

    Reminder to run `cargo clippy` and `cargo fmt` before submitting

- HW1:

    Still in progress! I am still working on it and will release soon, apologies for the delay.

Plan:
    + Poll

    + Finish discussion on nondeterminism: why nondeterministic semantics is useful

    + Ambiguity vs. nondeterminism

    + How semantics enables program properties: program equivalence, program optimization.

    + Revising the poll from Lecture 0.

If extra time: we can start Part 4.

## Friday, October 16

Announcements:

- HW1 available -- due Friday, October 23, 11:59pm

    This homework is a problem set

    May be challenging! Please start early

    https://github.com/DavisPL-Teaching/240-hw1

Plan:
    + Poll

    + Finish memory layout (allocation language) example

    + Ambiguity vs. nondeterminism

    + How semantics enables program properties: program equivalence, program optimization.

    + Revising the poll from Lecture 0.

If extra time: we can start Part 4.

Questions?
