//!
//! Part 3: Compiler optimizations.
//!

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
    Exercise: write a constant propagation algorithm for RandProg.
*/
