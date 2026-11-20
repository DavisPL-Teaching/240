# Lecture 3: Static Analysis

### Monday, November 16

Last week of lectures!

Announcements:

- Project presentations

  Sign ups will open **after class today** and due by this Wednesday, 11:59pm.

  We are working on grading the projects! Feedback ASAP hopefully by Wed/Thu.

  Project presentation details: SEE `project/presentation.md`.
  Starting next Monday!

Reminders:

- HW2 due this Friday (Nov 20)

Looking ahead:

- Presentations: Nov 23 -- Dec 4

- Final exam: **Tuesday, Dec 8 3:30-5:30pm**

  Please make sure you have marked your calendar

- Final project reports due: last day of finals period (Dec 11, 11:59pm)

  by popular request (tally was: 4 votes to 3)

Plan:

(first ~25 mins of class)

- Presentation details

- Go over last time's poll

- Finished compiler implementation from last time, and a few concluding thoughts (`lecture2/src/3-optimization.rs`)

(last ~25 mins of class)

- Lecture 3, Part 1: Introduction to Static analysis:

  + Start with the poll

  + Motivation: Enabling compiler optimization, + Safety

  + Soundness and completeness
    (or: overapproximation and underapproximation, and why)

  + Interval analysis example

  + General def. of static analysis.

  + Rice's theorem and undecidability (aka "Compiler full employment theorem").

  + Types of static analysis

## Wednesday, November 18

(Hope everyone is staying dry!)

Second-to-last day of lectures! Your in-class presentations will start next week.

Reminders:

- Please sign up for a project presentation slot by today 11:59pm!

- HW2 due this Friday (Nov 20).

  Please come by OH for help (with HW2 or your project presentations).

  My OH: today 4:30pm on Zoom TA OH: Friday.

  Note:
    + P1 is the type of proof question I'd like for you to be able to do.
      Come by if you are stuck or need help!

     + P2, P3 are pretty involved coding questions (especially P2).
       For the eam you should understand some of those higher level concepts
       (IR, syntactic sugar, platform-specific code).

     + P5 difficult theory problem. Understand: Turing completeness

Plan:

- Continuing on Static Analysis

  + Start with the poll

  + Soundness and completeness
    (or: overapproximation and underapproximation, and why)

  + Interval analysis example

  + Rice's theorem and undecidability (aka "Compiler full employment theorem")
    (Recap/reminder)

  + Approaches to static analysis.

      Major approaches:
      Symbolic execution, abstract interpretation, type checking.

Questions?

## Friday, November 20

Last day of class prior to the presentations.

Announcements:

- Project proposals **preliminary grades** and feedback has been published for those of you presenting on Monday

  + These were primarily graded by the TA, I will grade the presentations + final report

  + Please monitor Gradescope for feedback! I will announce when more grades are available
    We will get the remainder of them graded ASAP or this weekend.

- Details about final project report: will be posted next week.

Reminders:

- HW2 due **today**

- Final presentations starting Monday

We will talk about:

- Compositionality in static analysis

  + Interval analysis from last time

- Final poll.

- Approaches to static analysis

  + Major approaches:
    Symbolic execution, abstract interpretation, type checking.

Note 1:
There will be in-class polls for the classes next week and the following week.
These polls will be a chance for you to give feedback on the final presentations.
If you can't attend class, I will ask the presenters to upload their slides to Piazza so that you can share feedback on the slides through the poll.

Note 2:
I am preparing a course review file and some additional content for helping you study for the final exam.
I will release this on Piazza when it is ready.

Questions?
