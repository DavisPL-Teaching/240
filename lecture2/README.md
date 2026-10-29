# Lecture 2: Compilers and Static Analysis

### Wednesday, October 28

Reminders:

- Project proposal due **Friday, November 13**

  Please se materials in `project/` !

Plan:

- Some points from last time / Things I would like you to know
  (Included below)

- Answer to poll from last time
  (Included below)

- Today's poll at the start of class.
  --> This will revisit "backwards compatibility" from last Friday

Then:

- Start Lecture 2

  + Compilers; definition of correctness

  + Writing a compiler

  + Syntactic sugar and platform specific code.

  Further topics:

  + Intermediate representation (IR)

  + Compiler optimizations.

Questions?

##### Points from last time

- Apologies that last time was rushed!

  My hope: you were able to get a small taste or "sneak peak"  about some topics
  in how to define formal semantics.

  Some points / things I would like you to know:

    + The point of formal semantics: that just like we are able to instantiate the set
      of valid programs with a formal grammar G, we can instantiate the set of valid triples
      (INPUT, prog, OUTPUT) by giving a formal semantics relation

    + Small-step semantics: We define a relation called "evaluates to": when one program/context
      evaluates to another in a single step.

            e1 ---> e2
            ^^^^^ e1 evaluates to e2 in one step

        EXAMPLE: Evaluation rules for IntExp
        https://piazza.com/class/ms5m7g95gq1tr/post/42

    + Why do we define this "small step" relation, how is this different than an interpreter?

      More flexible ---> introduce nondeterminism
                         by leaving multiple possible rules to
                         evaluate.

                         flip_coin ---> True
                         flip_coin ---> False

    + How this gives rise to a set of (INPUT, prog, OUTPUT)!

    + Different styles of formal semantics:

        Operational, axiomatic, and denotational.

        Operational "feels" the most like how real code works, and often
        the most straightforward to define for your own languages.

##### Answer to poll

  75% of you have formed your project groups

  2: A, B, C, E (all except "developers of the underlying hardware")

    --> usually not! usually the other way around.

### Friday, October 30

Today: end of week 6!
After today, we have 3 weeks left (7-9), followed by project presentations (weeks 10-11).

Reminders:

- Project proposal due **Friday, November 13**

Plan:

- Poll

  This poll is for you to give me a bit of feedback on the pacing of the class so far.

- Start Lecture 2

  + Compilers; definition of correctness

  + Writing a compiler

  + Syntactic sugar and platform specific code.

Further topics:

  + Intermediate representation (IR)

  + Compiler optimizations.
