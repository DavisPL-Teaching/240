/*
    Part 4:
    Theoretical foundations
    4-theory.rs

    === Poll ===

    Background for today's poll:
    Some languages, like C, Rust (for unsafe Rust), and LLVM define
    a notion of "Undefined Behavior".
    The meaning of **undefined behavior** is the following:

    If PROG invokes undefined behavior on input INPUT, then

        (INPUT, PROG, OUTPUT)

    is valid for *any* output OUTPUT.

    Examples of undefined behavior include, for instance:

    - Dereferencing a null pointer

    - Accessing memory that is out-of-bounds of an array

    - Reading or writing from a dangling pointer (pointer whose memory has been
      deallocated)

    - Violating a class invariant or type invariant, such as creating a String
      that is not valid UTF-8 (in Rust),
      or a null-terminated string not terminated by a null byte (in C)

    - Data races (accessing the same location in memory from two separate threads,
      where at least one access is a write)

    Give your best shot at answering the following
    (briefly, in 1 sentence each):

    1. Based on what we have learned in class, WHY do you think these languages
    allow undefined behavior in their semantics?

    2. What risk does this pose to programmers?

    https://forms.gle/FTDKrZWaLrNsiaAB6

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

    ------------------------------------------------------------

    Oct 21

    Some selected answers to last time's poll.

    1. WHY do some languages allow undefined behavior?

        Performance is one reason but it may also allow for compiler developers to make decisions and allow for more optimizations.

        I think some languages allow undefined behavior because it gives the compiler more freedom to make programs run faster.

        I think some languages allow undefined behaviors for the sake of efficiency and to offload the responsibility of unsafe code onto the code developers instead of the language developers.

        Undefined behavior lets the compiler assume these situations never happen, so it can skip runtime checks (null, bounds, races) and optimize aggressively.

        To let compiler skip safety checks and optimize the code faster

        ... giving the programmer the ability to do unforseen(and potentially destructive) things.

    2. What risk does this pose to programmers?

        It risks the developers making mistakes while programming/puts the burden on the developer (i.e. dereferencing a null pointer)

        UB is the worst case scenario. Your program *runs*, but can do *anything*, something you may never accounted for.

        It needs programmer to be extra cautious about the programs they wrote to avoid the undefined behavior

        It poses security vulnerabilities if, for example, undefined behavior allows a program to access and/or modify data it shouldn't.

        They may put security risks on accident into their code. It may also cause crashes, and any other unexpected behavior when it is not properly accounted for

        A program with undefined behavior may crash or silently produce wrong results, often without any warning.

    Q: Compiler bugs?

    Compiler bugs mean the compiler does NOT conform to the formal semantics

    Reading: "Reflections on Trusting Trust", Ken Thompson.

    === Theoretical foundations ===

    A programming language is given formally by:

    - A **syntax** given as a formal grammar

    - A **formal semantics.**

    ----------

    ===== Syntax =====

    Syntax defines a *set* of valid programs.
    Syntax is almost always given as a formal grammar.

    This will define a formal language (a subset of all strings over an alphabet.)

        *Note:* what is the alphabet? traditionally ASCII. But modern programming
        languages usually allow Unicode :-)

    Each syntax element is given in terms of finitely many
    other syntax elements.
    The cases can be recursive.

    Examples of formal grammar:

    - Formal grammar for mathematical integers:

        <Number> ::= 0 | 1 | <Number> + <Number>

        The grammar generates expressions such as:

        Example:
        8 can be written as:

            1 + 1 + 1 + 1 + 1 + 1 + 1 + 1

        or:

            1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 0 + 0

    - Formal grammar for Integer expressions:

        <IntExp> ::= 0 | 1 | <IntExp> + <IntExp> | <IntExp> * <IntExp> | <Variable>
        <Variable> ::= x | y | z

        The grammar generates expressions such as:

            x + 1 * x + y * x

        WHY is this expression in the grammar?

            <IntExp>
            <IntExp> + <IntExp>
            <IntExp> + <IntExp> * <IntExp>
            <IntExp> + <IntExp> * <IntExp> + <IntExp>
            <IntExp> + <IntExp> * <IntExp> + <IntExp> * <IntExp>
            x + <IntExp> * x + <IntExp> * x
            x + 1 * x + y * x

        This is like a Rust enum, each case is one of the cases of our Rust enum,
        but with a bit of a problem, we don't have parentheses.

    - Formal grammar for Integer expressions with parens:

        <IntExp> ::= 0 | 1 | <IntExp> + <IntExp> | <IntExp> * <IntExp> | <Variable> | (<IntExpr>)
        <Variable> ::= x | y | z

        to express:

            x + 3 * x + 4 * x

        we can generate:

            x + (1 + 1 + 1) * x + (1 + 1 + 1 + 1) * x

    - Formal grammar for Boolean expressions:
      (SKIP)

        <IntExp> ::=

        The grammar generates expressions such as:

    - Formal grammar for RandProg programs:

        (Copy paste the Rust enum from Part 3 here, and modify)

        <RandProg> ::=

            SetVar(<Variable>, <Bool>)
            | CopyVar(<Variable>, <Variable>)
            | FlipCoin(<Variable>)
            | Input(<Variable>)
            | Output(<Variable>)
            | Seq(<RandProg>, <RandProg>)
            | If(<Variable>, <RandProg>, <RandProg>)

        <Variable> ::= x | y | z

        <Bool> ::= True | False

        The grammar generates expressions such as:

    A grammar defines a **set of strings**
    (you can think of simply, strings in ASCII or Unicode) that match
    the grammar.

    A **formal language** is a set of strings.

    That's the formal def of syntax!

        Syntax = formal grammar = formal language given by a formal grammar

        earlier: Syntax = defines the set of valid program

    More specifically:

        A formal grammar gives rise to a context-free language

    Chomsky hierarchy:
    https://en.wikipedia.org/wiki/Chomsky_hierarchy

    Context free languages are rather poorly behaved in practice.
    (I can talk about this more if we have time.)
    Could be considered a historical mistake to view PL syntax this way.

        Traditional theory: a PL syntax defines a context free

    Programming languages are context-free languages.
    (But not only! -- that's only the syntax part.)

    ===== Parse trees and AST =====

    We have been thinking of our Rust enum as a formal grammar.
    It doesn't match exactly: in fact, we are assuming the program
    has already been *parsed* via that grammar.
    What is parsing?

    Example:

        x + y * x

    was that derived from the grammar as

        <IntExpr>
        <IntExpr> + <IntExpr>
        <IntExpr> + <IntExpr> * <IntExpr>
        x + y * x
    OR
        <IntExpr>
        <IntExpr> * <IntExpr>
        <IntExpr> + <IntExpr> * <IntExpr>
        x + y * x

    ?

    **Definition:**
    A **parse tree** or **abstract syntax tree** (AST) is

        a tree where each node in the tree is a partially evaluated expression
        which shows the formal derivation from the grammar

    Example:
    The second derivation above corresponds to
    the following parse tree:

                 *
              /      \
              +       x
           /   \
           x    y

    ===== Ambiguous grammars =====

    **Not to be confused with:** ambiguous semantics!

    A topic that came up on Piazza is that a formal grammar can be ambiguous.

    (https://piazza.com/class/ms5m7g95gq1tr/post/32)

    What does that mean?

        Exactly as in the example above

        There exists a string (expression)
        - that can be derived from the grammar in two inherently different ways
        - i.e. two different possible parse trees.

        This is kind of annoying for PL design

        The "parser" is a component of any PL that takes the source program
        (string written by the user)
        and converts into the formal grammar of that PL.

        If the grammar is ambiguous, there may be cases where
        multiple parsed programs are possible.

    Parse tree AKA "Abstract Syntax Tree" (AST).
*/

/*
    ===== Poll =====

    Which of the following formal grammars are ambiguous?

    1. <Int> ::= 0 | 1 | -1 | <Int> + <Int> | <Int> * <Int>

    2. <Int> ::= 0 | 1 | -1 | 1 + <Int> | -1 + <Int>

    3. <Int> ::= 1 + <Int> | -1 + <Int> | <Int> + <Int> | <Int> * <Int>

    4. <Int> ::= 0 | 1 | -1 | (<Int> + <Int>) | (<Int> * <Int>)

    5: <Int> ::= 1 | <Int>1 | <Int>0

    (Select all that apply)

    https://forms.gle/izvWYuq2bMW7AioB8

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

    --------------------------------

    Continuing here for Oct 23

    Poll results

    Correct answers:

        Grammar 1 only!

        Grammar 3 is actually unambiguous.
        It's a bit of a trick question.
        It does not generate any strings, therefore there can be no
        example with two different parse trees.

    Points:

    - Any grammar can be made unambiguous by simply adding parentheses

    - In practice ambiguous grammars are usually undesirable. So we
      prefer to just ensure our grammar is unambiguous.

    One solution that is a nice treatment of this issue:
    Visibly pushdown languages and nested words:
    - https://www.cis.upenn.edu/~alur/Stoc04.pdf
    - https://en.wikipedia.org/wiki/Nested_word

    In a "visibly pushdown" language, in contrast to context-free languages,
    the grammar has to explicitly indicate when it goes into a recursive
    case (for example by adding parentheses).
    This is typically done in practice anyway.

    We will probably not get to this in class, but we could discuss
    it as a potential advanced topic.
*/

/*
    ===== Relation to Interpreters and Compilers =====

    We've been hinting that formal semantics is the foundation of
    building interpreters and compilers
    ("everyone should agree on what programs should do")

    But we have not spelled out the relationship explicitly.

    How might we define the following?

    We need a meta-language: Let's say we define a meta-language,
    whose semantics M is a set of triples

        (INPUT, PROG, OUTPUT)

    *such that* our INPUT set allows for ordered pairs.

    An interpreter I will be a program in the meta-language (PROG above)
    that takes input pairs:

        (INPUT, P)

    Finally, let's say we have a langauge whose semantics we want to implement,
    where the semantics S is a set of triples

        (INPUT, PROG, OUTPUT)

    An interpreter I implements (or satisfies) the semantics S if

        for any INPUT and any program P, and any OUTPUT, if

            ((INPUT, P), I, OUTPUT) is in M

        THEN

            (INPUT, P, OUTPUT) is in S.

    (SKIP)
    We can give a similar definition for compilers.
    A compiler implements (or satisfies) the semantics if ...
*/

/*
    **Backwards compatibility:**
    Recall: a semantics S can be thought of as a **set** of triples

        (INPUT, PROG, OUTPUT)

    A semantics S2 is backwards compatible with S1 if

        S2 ⊆ S1

    i.e.: all behaviors allowed by the new semantics are also allowed by the old semantics.

    Let's do a proof.

    We will prove or disprove the following:

    - If S2 is backwards compatible with S1, and an interpreter I1 implements S1, then I1 implements S2.
    - If S2 is backwards compatible with S1, and an interpreter I2 implements S2, then I2 implements S1.

    Before we prove/disprove:
    Let's do these first as a True/False exercise.

    https://forms.gle/ny2HH6cV41qzFQMT8

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

    Prove or disprove.

    - If S2 is backwards compatible with S1, and a interpreter I1 implements S1, then I1 implements S2.

    False.
    DISPROOF: Skipped.

    - If S2 is backwards compatible with S1, and a interpreter I2 implements S2, then I2 implements S1.

    True.
    PROOF:
    See PROOF.md.

*/

/*
    ----- Picking up for Monday, Oct 26 -----

    ===== Semantics =====

    As we have defined:
    A **formal semantics** is a **set** of triples

        (INPUT, PROG, OUTPUT)

    where INPUT is an input behavior, OUTPUT is an example output behavior,
    and PROG is a program.

    We now know that PROG should be an element of the formal grammar G.

    INPUT and OUTPUT we have to define sets for.

        Easiest model:
        I = set of all sequences of input symbols or input actions
        O = set of all sequences of output symbols or output actions
                            (e.g., error, crash, etc.)

        INPUT ∈ I
        OUTPUT ∈ O.

    We know that syntax (denoting the set of valid programs)
    can be given more specifically as a grammar G.

    But how do we give the semantics in a more specific way?
*/

/*
    ===== Types of formal semantics =====

    There are many specific ways of giving formal semantics.
    We have seen: reference interpreters / reference compilers
    and that they are NOT really a good way.

    Here are some other ways to define semantics.

    Most common:

    - Def. **Operational semantics.**

        + Small-step semantics:

        + Big-step semantics:

    - Def. **Denotational semantics.**
*/

/*
    Selection of other topics
    (Or skip for time)

    ===== Undefined behavior =====

    Undefined behavior (UB) - see earlier poll

    How does UB show up in operational semantics?
    In denotational semantics?

    ===== Lexing and parsing =====

    We defined parsing and parse trees above.

    In traditional compiler parlance, parsing your source code (e.g., .c or .rs file)
    is split into two stages, lexing and parsing.
    The "lexing" part uses a regular finite-state machine rather than
    a grammar.
    The parsing part uses a grammar and converts your code into the
    parse tree (AST).
*/

/*
    === Exercises and discussion questions ===

    1. Give a small-step operational semantics and a big-step operational semantics
       for

        a) Mathematical integers

        b) Integer expressions

        c) Boolean expressions

    2. In a sentence, explain the difference between syntax and semantics.

    3. How do syntax and semantics play into the development of...

        compilers?

        interpreters?

        static analysis tools?
*/

// ----------------------------

fn main() {
    println!("This is a theory part. It does not have any code.");
}
