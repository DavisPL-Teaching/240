/*
    Part 2: Approaches
*/

/*
    === POLL ===

    1. Which of the following is true about soundness and sound applications of interval analysis?

    A) An interval analysis is sound if the interval inferred for a variable x is a *superset* of
       the actual values of that variable along any real program execution.

    B) An interval analysis is sound if the interval inferred for a variable x is a *subset* of
       the actual values of that variable along any real program execution.

    C) If an interval analysis is sound, then I can replace an instance of a variable x with a
       choice of a constant integer, chosen within the bounds of its interval.

    D) Interval analysis, without clamping, will always terminate in finite time.

    E) Interval analysis, with clamping, will always terminate in finite time.

    2. Suppose that a sound interval analysis determines that x lies in the interval
        [10, 100], immediately before some while loop is reached.

        Give an example of a while loop that can be optimized away (removed as dead code)
        given the result of this analysis.

    https://forms.gle/hcnJHyDzMJoHYMKs8
*/

/*
    === Approaches ===

    Major approaches to static analysis:

    I will illustrate each with an example.

    Example:

    x = INPUT
    if (x > 10) {
        x = x + 1
    } else {
        x = x - 1
    }
    OUTPUT x

    Q: what are all possible values of x after the program executes?

    A: How would you solve this problem?

    .
    .
    .

    - Symbolic execution

    - Abstract interpretation

    - Type checking.

*/

/*
    Recap

    .
*/
