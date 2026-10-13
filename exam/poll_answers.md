# Poll answers

Sep 23: (Is it a programming language?)
We'll discuss this more in class. Correct answers:
Python, Fortran, SQL, Turing Machine: Yes
Microsoft Excel, ChatGPT: probably not. Though Excel contains a programming language inside it,
so we could say that Excel is "more than just a programming language" as it also is primarily about the UI.

Sep 25: N/A

Sep 28:
Some possible answers: print does not say whether displaying text should end with a newline;
not clear if program counter is an int or a float

Sep 30:
B and C
(A and D are examples of ambiguous syntax)

Oct 2: B and E

Oct 5:
    A and E
    D would often be recursive too, for another language, but not for this one.

Oct 7:
    A, B, C, E are valid

Oct 12:
1) there should be 4 possible outputs. tt, tf, ft, ff (or some variant of this)
2) Yes
3) No

For Q2, one way to do it is to say
(INPUT, prog, OUTPUT) is allowed iff OUTPUT is the output of *at least one* run of reference_interpreter.

Note that I updated the wording of Q3 to the following:
"Can we use any deterministic formal semantics to define the semantics of RandProg programs in a way that agrees with our reference interpreter?"

The point here is that any deterministic semantics -- or deterministic reference interpreter (used as the semantics directly) -- will not work because it allows only one output.
