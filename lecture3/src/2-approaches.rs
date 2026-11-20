/*
    Part 2: Approaches
*/

/*
    === POLL ===

    1. Which of the following is true about soundness and sound applications of interval analysis?
    (Select all that apply)

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

        Give an example of a while loop that can be optimized away (removed from the program)
        given the result of this analysis.

    https://forms.gle/hcnJHyDzMJoHYMKs8
*/

/*
    === Approaches ===

    Major approaches to static analysis:

    x = INPUT
    if (x > 10) {
        x = x + 1
    } else {
        x = x - 1
    }
    OUTPUT x

    What are all possible values of x?

    .
    .
    .

    - Abstract interpretation

        In interval analysis: we assigned to each point in the program an *interval*
        of possible values.

        We also needed:
            - a way of combining the two sets of intervals at different program points (for example, after
              an if-else block)
              This is a "merge" function
            - a "clamp" function that allowed the whole thing to terminate

        This generalizes!

        We define some "abstract domain" that represents the *set* of possible values of x,
        for each variable x

        In this case: abstract domain = all possible intervals [min, max]

            With clamping: set of intervals [min, max] where min, max are either ± ∞ or between
                -100 and 100.
                (finite!)

            Abstract domain = A

            need: a merge function A x A -> A
            need: a clamp function: A -> A
                                    A -> FiniteA

        We track, for each variable x, a set in the abstract domain.
        We iterate the program until these abstract values stabilize and reach
        an equilibrium.

        To allow the analysis to terminate: we use clamping to reduce the abstract set
        to a finite set.

    - Symbolic execution

    - Type checking.
*/

/*
    (SKIP)
    Illustrate each with an example.

    Example:


    Q: what are all possible values of x after the program executes?

*/

/*
    Recap

    .
*/
