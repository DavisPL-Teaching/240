# ECS 240: Program Verification - Fall Quarter 2026

## Contents

1. [Welcome and Course Information](#welcome-and-course-information)
2. [Lectures](#lectures)
3. [Course Description](#course-description)
4. [Schedule](#schedule)
5. [Grade Breakdown](#grade-breakdown)
6. [Policies](#policies)
7. [Contact and Office Hours](#contact-and-office-hours)

## Welcome and Course Information

Welcome!

- **Instructor:** [Caleb Stanford](https://web.cs.ucdavis.edu/~cdstanford/)
- **TA:** TBD
- **CRN:** TBD
- **Units:** 4
- **Lectures:** TBD
- **Final exam:** TBD
- **Piazza:** TBD (for class Q+A, announcements, and office hours)

## Lectures

I often lecture via live coding. I will post the code and lecture materials in this repository.
To follow along with the lectures, clone the repository:
```shell
git clone git@github.com:DavisPL-Teaching/240.git
```

If you make changes to the code after each lecture, you will need to discard them before pulling again.
For example, you can run:
```shell
git stash
git pull
```

### Lecture Recordings and Zoom

All lectures are broadcast and recorded on Zoom, and the recordings are made available on Canvas.
Please note that this is a best-effort process, technical difficulties do occur occasionally
(such as lost video recordings or poor audio quality).
If you miss class, you can make up the lectures and class polls by viewing the recording
at any time prior to the last day of class.

## Course Description

Topics in programming languages at the graduate level.
This course will survey theoretical and applied topics, including: formal syntax and semantics, relation between formal semantics and verification, static analysis including abstract interpretation and symbolic execution, program correctness properties and program logics, functional programming languages, and type theory. We will study these topics through a hands-on perspective (working with programming languages such as Dafny, Haskell, Rocq, etc. and building tools that can be used for static analysis and verification of programs).

### Prerequisites

An undergraduate course in programming languages (e.g., ECS 140A) is helpful, but not always required. Some prior background in mathematical reasoning (writing proofs, e.g. ECS 20 or ECS 120) at the undergraduate level is required and will be assumed.
As a graduate course, homeworks are a combination of theory and programming assignments, and there is a project component.
Please ignore the prerequisite listed on [the department website](https://cs.ucdavis.edu/schedules-classes/ecs-240-programming-languages).

This course is appropriate for graduate students or advanced undergraduates.
If you are an undergraduate, you should most likely take 140A first.

### Textbook

TBD

### Learning Objectives

By the end of the course, students will be able to:

- Understand the important definitions of programming languages; what makes a programming language including formal syntax and semantics, interpreted and compiled languages, type systems, etc.;
- Understand how program semantics relates to other topics including the correctness and verification of software;
- Know how to define and apply static analyses for the automated analysis of programs;
- Understand the functional foundations of programming languages including formalizing programs in the lambda calculus;
- Apply these techniques in the context of specific programming languages and program analysis and verification tools.

## Schedule

See `schedule.md`.

## Grade Breakdown

Your grade is evaluated based on:

- **Participation (10%):** via in-class polls
- **Homeworks (10%):** about 3 problem sets planned, plus homework 0
- **Final Exam (40%):** I am planning to do one final exam, either in-class or during finals period
- **Project (40%):** Final project; details TBD

### Attendance and Participation

To encourage class attendance, there are in-class polls that are worth a small amount of credit.
If you miss class, you may make up the polls at any time (up to and including the last day of class) by watching the lectures and filling out your responses offline. Please note that your poll scores will be automatically updated on the next Canvas sync and you do not need to notify me of make-up poll submissions.

You will also give a short presentation on your final project (likely 10-20 minutes).
More details on the project presentation will be announced later on in the class.

### Homeworks

Homeworks will consist of problem sets which are a combination of programming assignments and pen-and-paper exercises -- solutions must be written up in LaTeX.
I plan to assign homework assignments roughly bi-weekly (about 4 total), plus homework 0, which is designed to help you install the relevant software for the course.

**Important: your code must run to get credit!**
Frequently running and testing your code during development is an essential part of computer programming -- and a very important skill in the real world! This includes making sure that your code works on someone else's machine. The graders won't be able to debug your submission, and code that does not run may receive a 0 or at most 50% of partial credit.

For programming assignments, you should put thought into producing a solution that is not only correct but well written, high-quality code.
That is: is it readable, shareable, well-documented, well-commented, logically separated into modules and functions, and reasonably efficient?
Are your free response answers thoughtful?
We will use Gradescope for homework submissions and grading.

### Final Exam

One final exam is planned.
Exams are closed-book, but you may bring a single-sided cheat sheet.
Exams will be graded on Gradescope.

I reserve the right to curve exams (this can only help you). That means, for example, if 100 points are possible, it may be entered as out of a smaller number of points like 95 or 85 for the purposes of calculating your grade, if the average score of the class was low or if there were unexpectedly difficult questions.

### Final Project

There will be a final project (including a project proposal and a presentation) to put the concepts studied in class to practice.
I will announce more details in class!

### Final Grade

For the final (letter) grade, the following are minimum cutoffs. That is, if your grade is above the given percentage, you will definitely receive at least the given grade. However, I reserve the right to lower the cutoffs (i.e. you might get a better grade than what the table says).
I will use this to help students who may be on the boundary between two grades at the end of the quarter.

| Percentage | Minimum Grade |
| --- | --- |
| 93 | A  |
| 90 | A- |
| 87 | B+ |
| 83 | B  |
| 80 | B- |
| 77 | C+ |
| 73 | C  |
| 70 | C- |
| 67 | D+ |
| 63 | D  |
| 60 | D- |

## Policies

### AI Policy

I do allow AI use on homework assignments, but with some discretion.
You should not use AI to generate entire solutions, but in a targeted way to provide solutions matching your own requirements, and to assist with your own understanding.
Use your critical thinking skills!
Exams are held in-class and closed-book.

### Collaboration Policy and Academic Integrity

This class will encourage collaboration; however, each person should complete their own version of the assignment. **You should not directly copy code from someone else, even within your study group,** but you can receive help and give help on individual parts of an assignment.

You may work on homework assignments in groups of up to 3 people, as long as everyone is working on their own copy of the code. If you do this, please list the names of your collaborators at the top of your submission.

Beyond the above: please use common sense!
This course strictly follows the [UC Davis Code of Academic Integrity](https://ossja.ucdavis.edu/academic-integrity).

### Late Policy

In-class participation points (polls) can be made up at any point during the quarter (prior to the last day of class), by submitting the Google form link. The forms will remain open and can be found by viewing the lecture recording or lecture notes for that day.
I will not be able to provide a consolidated list of the form links, as their purpose is to encourage you to attend and
watch the lectures.

Homeworks will generally be due at 11:59pm on the due date.
**I cannot guarantee that late homeworks will be graded.**
However, I encourage you to update your assignments even after the deadline -- Gradescope will allow you to upload late work up to a few days after the assignment deadline.
At grading time, we may choose to grade the more recent version at our discretion.

### Job Scams

Communication from the instructor will only be sent through official UC Davis email addresses and channels. Please be vigilant of job scams impersonating faculty members. For more information, visit [UC Davis Job Scams Prevention](https://careercenter.ucdavis.edu/job-and-internship-search/job-scams).

You can view and report job scam emails at the [Phish Bowl](https://phishbowl.ucdavis.edu/).

### Be Nice!

Please be kind to each other!
UC Davis has [policies against harassment and discrimination](https://hr.ucdavis.edu/departments/elr/preventing-discrimination-harassment).
Be inclusive of your classmates in group study, in group work and projects, and in your questions and answers in class.
If you need to, you may reach me by email to report an issue with a classmate.

If you or another graduate student you know needs help, please look at the [GGCS support form](https://ucdavis.co1.qualtrics.com/jfe/form/SV_bCLSRFqZ1lLKgM6). It can be filled out to get help, and contains many links to support resources.

### Recommendation letter and research requests

I am happy to write recommendation letters for students.
However, please note that I generally reserve recommendation letters for students who have done particularly well in a course (A or A+), or for students I have interacted more directly with through either research or office hours.
If you would like to request a recommendation letter from me, please wait until after the quarter to do so
(in rare cases, I may be able to make an exception to this rule if you have a deadline during the quarter).

If you are asking to get involved in research, a similar rule applies - please wait until after the quarter to ask.

## Contact and Office Hours

**Please use the Piazza for questions related to course material.**
If you send me an email, I will most likely respond to post your question on Piazza :)

On Piazza, please ask all questions publicly, with only one exception: if your post contains a large snippet of code then make it private. I encourage you to ask anonymously if you feel more comfortable.

The instructor and TAs will be available during office hours for additional support on course material and assignments. The schedule of office hours will be posted in a pinned note on Piazza.

If you have a question that is more sensitive or unrelated to the course, please email me: `cdstanford` `ucdavis` `edu`.
