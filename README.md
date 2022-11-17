# btcc
a [Boolfuck](https://esolangs.org/wiki/Boolfuck) To [Ctfuck](https://esolangs.org/wiki/CTFuck) Compiler, based on [this thesis](https://www.ini.uzh.ch/~tneary/tneary_Thesis.pdf) (page 86, section 5.2.1).
reading it, i found that CTFuck was much closer to a Clockwise Turing Machine than a Cyclic Tag system. still it does not make the name outdated, as they have the same initials...

# working
this works by first compiling BF to a restricted-but-not-really version of BF, which has no `>`. instead, every command except `<` moves one cell to the right after doing it's operation.
below is the table of BF commands and their equivalent in resricted BF:

| BF | "restricted" BF |
| -- | -------------- |
| `.` | `.<` |
| `,` | `,<` |
| `[` | `[<` |
| `]` | `]<` |
| `+` | `+<` |
| `<` | `<` |
| `>` | `+<+` |

finally, it compiles this to CTF. each bit in boolfuck uses 4 bits in CTF. the first two bits tell if this is the R marker or the L marker. the thrid bit stores the value if the current cell is neither, 0 otherwise.

the fourth bit is the Y symbol marker used to move left by one cell..

since CTF is atleast as efficient as a clockwise turing machine, this means that the compiled CTF will take O(t^2) time to run for the source BF program running in O(t), as shown in the above thesis.
