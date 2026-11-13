# Lecture 2: Compilers and Optimization

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
After today, we have 3 weeks left (7-9), followed by in-class presentations (weeks 10-11).

Plan:
  + Roughly this week and next week (week 7) on compilers, optimization

  + Roughly weeks 8-9 on static analysis and dynamic analysis, advanced topics.

Reminders:

- Project proposal due **Friday, November 13**

  Some notes and changes to the project - after talking to some of you:

    + **New requirement:** I will require a basic interpreter as a minimum for your project
    (minimum goal), *plus* one of the other four tool options (compiler, static analysis tool,
    dynamic analysis tool, or type system).

    + These don't need to be built as part of your proposal.
      Proposal is a chance to get feedback

    + I will recommend steering away from the "type system" choice.

    + Reminder: main point of the project is to design your own language! (or subset of an existing
      language)

      * Get concrete as soon as possible, by drafting your grammar + example programs.
        I've also updated the project proposal description a little, in `project/` -
        Give us some example syntax!
        It doesn't have to be complete at this stage. It will help force you to make decisions
        about your language and what it will look like early on.

      * You will need to also define the formal semantics for your final report.

Plan:

- Poll

  Quick poll - feedback on the pacing of the class so far.

- Start Lecture 2

  Part 1:

  + Compilers; definition of correctness

  + Writing a compiler

  Part 2:

    + Syntactic sugar and platform specific code.

    + How this gives rise to intermediate representations (IR)

Coming up ahead:

  + Compiler optimizations.

Questions?

### Monday, November 2

Announcements/reminders:

- Project proposal due Friday, November 13

- HW2 release upcoming (probably sometime this week)

Plan:

- Continue on compilers:

  + Finish part 1: Compiler example and some discussion

  + Part 2: Compiler concepts

    * Poll

    * Syntactic sugar, platform specific code

    * Intermediate representations

    * Compiler optimizations.

Questions?

### Weds., Nov 4

Reminders:

- Project proposal due Friday, November 13

- HW2 WIP, release upcoming soon

Announcements:

- Remaining OH will alternate Zoom and in person

  + Today on Zoom, next week in person, etc.

Plan:

- Fixed compiler from last time.

- Part 2: Compiler concepts

  + Note on last time poll

  + Layer based compiler development; IRs

  + Syntactic sugar

  + Poll

  + Platform specific code

  + How IRs enable optimization passes.

Questions?

### Fri., Nov 6

Announcements:

- HW2 released! Due in 2 weeks, Friday November 20, 11:59pm.

Reminders:

- Project proposal due Friday, November 13

Plan:

- Finish Part 2:

  + Platform specific code

  + Poll and coding example

  + Recap and exercises (SKIP - these appear on your HW)

- Part 3 on Optimization:

  + Some history of program optimizations

  + The WHILE language, constant propagation

  + A proof exercise on the WHILE language.

Questions?

### Monday, Nov 9

Announcements:

- No class this Wednesday (Veteran's Day)

Reminders:

- HW2 due Friday November 20, 11:59pm.

- Project proposal due Friday, November 13

Plan:

- Go over poll from last time

- Part 2: recap/finishing thoughts

- Part 3 on Optimization:

  + Some history of program optimizations

  + The WHILE language, constant propagation

  + During class: poll on optimization.

  + A proof exercise on the WHILE language.

Questions?

### Friday, Nov 13

(Happy Friday the 13th!)

Announcements/reminders:

- Project proposal due Today!

- HW2 due Friday November 20, 11:59pm.

- HW1 grades released

- NB: project presentation slots will open up on Monday, along
  with more details about your presentations
  (# of presentations per slot, etc.)

Plan:

- Compiler optimizations:

  + Go over poll from last time

  + Challenges in compiler optimization
    Or, how to build your own optimizer

    * Motivation for static analysis.

  + Example: constant propagation in the WHILE language.

This will be a "very brief" introduction to compiler optimizations.
I hope to give you a sense of the main principles of how optimizers are built,
and how to build your own optimizer for your projects.

This will be the last lecture on compilers.
We will spend next week, the last week, on static analysis,
which is one of my favorite topics and which I hope to touch on
using our remaining time.
