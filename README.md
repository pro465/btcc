# btcc
a [Boolfuck](https://esolangs.org/wiki/Boolfuck) To [Ctfuck](https://esolangs.org/wiki/CTFuck) Compiler.

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

finally, it compiles this to CTF.
