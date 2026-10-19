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

    === Theoretical foundations ===

    A programming language is given formally by:

    - A **syntax** given as a formal grammar

    - A **formal semantics.**

    ----------

    ===== Syntax =====

    Syntax defines a *set* of valid programs.
    It's a formal language (a subset of all strings over an alphabet.)

        *Note:* what is the alphabet? traditionally ASCII. But modern programming
        languages usually allow Unicode :-)

    Syntax is almost always given as a formal grammar.

    Each program element is given in terms of

    Examples of formal grammar:

    - Formal grammar for mathematical integers:

        <Number> ::= 0 | 1 | <Number> + <Number>

    - Formal grammar for Integer expressions:

        <IntExp> ::=

    - Formal grammar for Boolean expressions:

        <IntExp> ::=

    - Formal grammar for RandProg programs:

        (Copy paste the)


    Set of strings defined by a grammar:

    Formal language:

    Chomsky hierarchy:
    https://en.wikipedia.org/wiki/Chomsky_hierarchy

    Context free languages are rather poorly behaved in practice.
    (I can talk about this more if we have time.)
    Could be considered a mistake to view it this way.

    Programming languages are formal languages.
    (But not only -- that's only the syntax part.)

    ===== Parse trees and AST =====

    We have been thinking of our Rust enum as a formal grammar.
    It doesn't match exactly: in fact, we are assuming the program
    has already been *parsed* via that grammar.
    What is parsing?

    **Definition:**
    A **parse tree** or **abstract syntax tree** (AST) is ...

    Example:
*/

/*
    ===== Semantics =====

    A **formal semantics** for a language is a set of triples

        (INPUT, PROG, OUTPUT)

    (remember: it is a set! like an abstract oracle,
     not a program, can be defined purely mathematically)

    This is the same definition from before - but now we know what
    PROG is.

    - INPUT

    - PROG

    - OUTPUT

    There are many specific ways of giving formal semantics.
    We have seen: reference interpreters / reference compilers
    and that they are NOT really a good way

    More below.

*/

/*
    ===== Relation to Interpreters and Compilers =====

    We've been hinting all along that formal semantics is the foundation of
    building interpreters and compilers
    ("everyone should agree" on what programs should do!)

    But we have not spelled out the relationship explicitly.

    Some definitions.

    An interpreter implements (or satisfies) the semantics if ...

    A compiler implements (or satisfies) the semantics if ...

    **Backwards compatibility:**
    A semantics S2 is backwards compatible with S1 if ...


*/

/*
    ===== Types of formal semantics =====

    Reference interpreters or reference implementations are great, but they aren't
    fully general for all languages.

    There are many ways to define semantics.

    Most common:

    - Def. **Operational semantics.**

        + Small-step semantics:

        + Big-step semantics:

    - Def. **Denotational semantics.**
*/

/*
    Other topics
    (Selection of topics or skip for time)

    ===== Undefined behavior =====

    Undefined behavior (UB) - see today's poll

    How does UB show up in operational semantics?
    In denotational semantics?

    ===== Ambiguous grammars =====

    **Not to be confused with:** ambiguous semantics

    A topic that came up on Piazza is that a formal grammar can be ambiguous.

    Example?

    In practice this is usually undesirable.

    One solution that is a nice treatment of this issue:
    Visibly pushdown languages and nested words:
    - https://www.cis.upenn.edu/~alur/Stoc04.pdf
    - https://en.wikipedia.org/wiki/Nested_word

    ===== Lexing and parsing =====

    We defined parsing and parse trees above.

    In traditional compiler parlance, parsing your source code (e.g., .c or .rs file)
    is actually split into two stages:

    Lexing:

    Parsing:

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
