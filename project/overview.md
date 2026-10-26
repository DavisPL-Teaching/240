# ECS 240 Project Overview

The goal of the project will be to build a programming language-based tool for a toy constructed language of your choosing; or for a small (bug significant) subset of an existing language.
You will also be asked to draft the formal syntax and semantics of your language on paper as part of your
final report.

Tool options:

- Interpreter,
    (careful - there should be some complexity beyond just "run the program")

- Compiler,
    (or Transpiler)

- Static analysis tool,

- Dynamic analysis tool, or

- Type system.

You will need to build all the primary relevant technology for the project,
though you may incorporate some existing tools if required (these should be detailed in your proposal).
I recommend (but do not require) that you develop the project in Rust.

## Group size

Please form groups of size 2-4 for the project. I encourage larger groups, 3 or 4 people if possible.

As previously announced,
I will allow a limited number of 1-person groups by request for exception.
You should send me an email explaining why you want a 1 person group. The main exception would be
if you are working on an MS or PhD thesis and your project is directly related to your research project.

## Deliverables

There are three deliverables for the project: the proposal, the presentation, and the final report.
Please refer to `proposal.md`, `presentation.md` (forthcoming), and `report.md` (forthcoming) for further details.

## Topics or Applications

Programming languages is a highly versatile field: PL can be defined for any application domain of interest.
The world is your oyster!

An important part of the project will be identifying the toy language or subset that you are interested in working with.
I recommend choosing something that is relevant to your own interests or research areas!
For example, design a simple language but extend it with some primitives relevant to your interests.
Some ideas:

- Operating systems and architecture: primitives related to specialized CPU/GPU instructions?
  primitives for OS interaction, syscalls?

- Networking and distributed systems: Network primitives? Concensus protocols as part of your langauge?
  Fault tolerance as part of your semantics?

- Theory: primitives for algorithmic complexity tracking? for: termination proofs, interactive proofs, zero knowledge proofs, cryptography?

- ML: primitives for training? gradient descent? (Talk to me about differentiable programming)
  For statistical sampling, probabilistic programming, etc.?
  Data processing languages - MapReduce, dataflow programming (also related to DB below)

  https://en.wikipedia.org/wiki/Differentiable_programming
  https://en.wikipedia.org/wiki/Probabilistic_programming

- Databases: Logic programming constructs? Primitives for data invariants (like functional dependencies,
  cardinality constraints). Type systems are a good fit here.

- Software engineering: Language features: OOP, inheritance, software design patterns, encapsulation/privacy, modules, etc.

- HCI: user interfaces as programming constructs; display and markup languages, scientific experiments as programs, etc.

- Security: Primitives for access control, cryptographic primitives, data integrity/privacy, availability, etc.

## Timeline

All due dates are 11:59pm.

**Project proposal:** due Friday, November 13

**Final project presentations:**
In class Nov 23, Nov 25, Nov 30, Dec 2, Dec 4
(Sign up will be posted after the project proposal)

**Final exam:** Tuesday, Dec 8, 3:30-5:30pm.

**Final project report:** due last day of finals period (Dec 11)
(pending poll on Piazza).
