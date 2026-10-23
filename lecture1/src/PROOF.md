Claim: If S2 is backwards compatible with S1, and a interpreter I2 implements S2, then I2 implements S1.

PROOF:

    Let S2 be backwards compatible with S1. So we know:

        S2 ⊆ S1

    Let I2 implement S2. Let M be the metalanguage semantics.
    Therefore we know:

        for all INPUT, PROG, OUTPUT,  if

            ((INPUT, PROG), I2, OUTPUT) ∈ M

        then

            (INPUT, PROG, OUTPUT) ∈ S2

        (1)

    We want to show that I2 implements S1. So we need to show:

        for all INPUT, PROG, OUTPUT,  if

            ((INPUT, PROG), I2, OUTPUT) ∈ M

        then

            (INPUT, PROG, OUTPUT) ∈ S1.

    But now let (INPUT, PROG, OUTPUT) by any triple such that

        ((INPUT, PROG), I2, OUTPUT) ∈ M

    by (1), we know

        (INPUT, PROG, OUTPUT) ∈ S2

    and since S2 ⊆ S1, we have

        (INPUT, PROG, OUTPUT) ∈ S1.

QED.
