# Project proposal

The project proposal should be 2-4 pages, typed,
following the following sections and subsections.
I recommend (but do not require) that the proposal be written in LaTeX.

**Report length:**
Minimum 2 full pages, maximum 4 pages (not including references) -- these are hard cutoffs! If you need more material, you may include it only in an "appendix" after the main document. An appendix may or may not be graded at our discretion.
Do not change the font, margins or do other spacing tricks to fit within the page limit.

**Deadline:**
Friday, November 13 at 11:59pm.

## Disclaimer about ML-based tools

I will have to issue a warning up front about ML-based tools, as I anticipate some projects
looking to use ML as a core component of their project.
**Using machine learning as a core component of your static analysis, compiler, dynamic analysis tool, or type system, etc. is likely going to lead to an out-of-scope project.**
For the purposes of this project, I want you to understand how to design and build traditional programming languages tools with hard-line formal guarantees. LLMs and other ML techniques are a black box which would run contrary to this goal. You can use LLMs or ML components as a piece of a project as long as the overall project can be articulated in terms of concrete guarantees, like formal syntax and semantics, proofs that programs behave the way they are supposed to, etc. If you're not sure, please ask me about your project after class or discuss in office hours.

That doesn't mean you can't do a project about ML! (See: differentiable programming, gradient descent as a language
primitive, data processing languages like MapReduce, etc.) Talk to me about ideas.

## Sections and subsections

### Title and group members

At the top of your proposal, list the title of your project and all of your group members.

Please form groups of size 2-4 (and encourage group sizes of 3-4), unless you have received an explicit
exception from me by email.

### 1. Introduction and motivation

1.1 **Problem domain and motivation.**
State your application domain. What problem or set of problems is
your project intending to solve, and why it is interesting?
This should include at least 3 citations, including at least 2 research papers related
to your chosen research area or domain
(see related work, below).

1.2. **Proposed language.**
Explain whether you will design your own toy language, or a limited but significant subset of an existing language.
If you chose your own language, explain what it will contain and what it will do.
If you choose an existing language, explain what subset of keyword(s) and feature(s) you will select or what part of the language you will focus on.

Give us some example syntax! It doesn't have to be complete at this stage.
Also, you should give a draft of the grammar
of what commands/programs are allowed - this can go under "Syntax and Semantics" below.

1.3. **Proposed tool.**
You should describe at least a basic interpreter for your language that can be used
to run programs, plus one of the following 4
options: compiler, static analysis tool, dynamic analysis tool, or type system.
(If you have another idea, please talk to me about it to get approval.)
Describe:
What is the input and output to your tool?
How will a user interact with it?

1.4. **Scope and related work.** Is the solution to the problem already known and built?
(Ideally not! However, this is not strictly a requirement.)
What existing tools are there to solve a similar problem?

### 2. Goals

What are the high-level goals of the project? What do you hope to have working by the end of the project?
Please divide your goals into three categories:

- **Minimum goal:** This is the minimum version of your code that you **must** have working by the end of the quarter. This could be, for example, a minimum version of your language with 4-5 constructs, and a working version of your tool but all it does is 1 or 2 things, not any additional fancy features.

- **Target goal:** This is the version of your code that you **plan** to have working by the end of the quarter, as a target deliverable. This could, for example, have your compiler, together with a few important optimizations.

- **Reach goal:** This is the version of your code that has all the additional features you might want, if you have extra time. This may include additional features, etc. as needed, but none of them on the "critical path" to make a working version of your project.

These should be described at a high level. The specific component(s) that are included in each goal may refer to the following section and should be described in more detail below.

### 3. Theory part: Syntax and Semantics

Give us some example syntax! It doesn't have to be complete at this stage.
Provide a draft of your formal grammar of what commands/programs are allowed.
This exercise will help push you to be more concrete about your language and what it contains.

In at most a paragraph, give a rough sketch of the how the semantics will go.
(set of `(INPUT, prog, OUTPUT)` triples).
This doesn't have to be a complete description; I want to see that you have thought
about what the syntax and semantics might look like.
You will be asked to provide a complete syntax and semantics (on paper) for the final report.

### 4. Implementation plan

2.1. **Tool and language selection.** What tools and languages will you use to develop your project?

2.2 **Incorporating existing tools.**
Will your project be developed from scratch, or will it
require using or interacting with any existing software or already developed code bases?
If the latter - please describe these and give justification for why you need each existing tool.

2.3. **Architecture of your code.**
Lay out an expected architecture for how your tool will work. What component(s) are there and how do they interact? Consider drawing a diagram. This can be tentative at this stage!

2.4. **Development effort.** Which functions, components, or modules will you need to develop to achieve the above components?

- It may help to refer to your architecture diagram from above.

- Is there anything or any additional components or features that are specifically *out of scope* for your project?

2.5. **Challenges.** Do you foresee any challenges coming up during the project? What could go wrong when attempting to build the above components? What do you plan to do if any of these steps becomes difficult?

#### 5. Collaboration plan

State your plan to collaborate to develop the project.

- How will you divide different components among your team members that can each be developed separately?

- What are your plans for group meetings?
  I suggest meeting at least twice a week.
  What platform will you use to communicate with your group?

- **Timeline:** Give a specific weekly timeline with which components will be developed in each week. This timeline can just be "expected": I won't be holding you to these deadlines, but I want to see that you are thinking about what components need to be worked on and how the pieces fit together.

- **Commitment statement:** Lastly, please list a statement for each group member that each person commits to work and contribute equally to the project. The following statement should be included once, for each group member: "I commit to work and contribute equally to this project" with the group member's name signed.

This sectino is not required for groups of size 1.

#### 6. Conclusion and questions for feedback

This section can be short (e.g., 1-2 paragraphs).
State what will happen if the project is successful. What is exciting about it? What is interesting about it? Why is your application domain important?

Do you have any questions about the project? What are some things you are uncertain about that should be resolved, what would be helpful to get feedback on?

### References cited

References cited is mandatory!
Include any papers or websites related to what your project is going to do and citations at the bottom of your document. Please include at least 3 citations at a minimum,
and at least 2 of these to research papers.
