use std::io::Write;
use std::{env, fs};

fn main() {
    let mut tr = Translator::new();

    let src = fs::read(
        fs::canonicalize(env::args().nth(1).unwrap_or_else(|| help()))
            .expect("could not canonicalize source file"),
    )
    .expect("source file should be readable");

    let mut dest = fs::File::create(
        env::args().nth(2).unwrap_or_else(|| help()), //     .expect("could not canonicalize target file"),
    )
    .expect("could not create file for compilation");

    tr.translate_prog(&turn_into_rbf(&src));

    dest.write_all(turn_into_ctf(&tr.res).as_bytes())
        .expect("target file should be writable");
}

fn help() -> ! {
    println!(
        "usage: {} <src> <target>",
        std::env::current_exe().unwrap_or("btc".into()).display()
    );
    std::process::exit(-1);
}

fn turn_into_rbf(c: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    for &i in c {
        if b"[]+,;".contains(&i) {
            ret.push(i);
            ret.push(b'<');
        } else if i == b'>' {
            if let Some(b'<') = ret.last() {
                ret.pop();
            } else {
                ret.extend_from_slice(b"+<+");
            }
        } else if i == b'<' {
            ret.push(b'<');
        }
    }

    ret
}

fn matching_brace<'a>(c: &'a [u8], i: &mut usize) -> &'a [u8] {
    let start = *i + 1;
    let mut count = 1;
    let mut curr = start;

    while count > 1 || c[curr] != b']' {
        match c[curr] {
            b'[' => count += 1,
            b']' => count -= 1,
            _ => {}
        }
        curr += 1;
    }

    *i = curr;

    &c[start..curr]
}

fn turn_into_ctf(prog: &[Command]) -> String {
    let mut ret = String::new();
    for i in prog {
        ret.push_str(match i {
            Nl => "\n",
            Pop => "$",
            Input => ",",
            Output => ".",
            Push(x) => {
                if *x {
                    "1"
                } else {
                    "0"
                }
            }
            Jmp(x, y) => {
                let f = |&x: &usize| if x == 0 { String::new() } else { x.to_string() };
                ret.extend(format!("[{}|{}]", f(x), f(y)).chars());
                continue;
            }
        });
    }
    ret
}

#[repr(usize)]
enum Command {
    Nl,
    Pop,
    Input,
    Output,
    Push(bool),
    Jmp(usize, usize),
}

use Command::*;

struct Translator {
    line: usize,
    res: Vec<Command>,
}

impl Translator {
    fn new() -> Self {
        let mut ret = Self {
            line: 1,
            res: vec![],
        };

        ret.push(&[1, 0, 0, 0, 0, 1, 0]);
        ret
    }

    fn translate_prog(&mut self, prog: &[u8]) {
        let mut idx = 0;
        while idx < prog.len() {
            self.translate(prog, &mut idx);
        }
    }

    fn translate(&mut self, prog: &[u8], idx: &mut usize) {
        match prog[*idx] {
            b'+' => {
                self.handle_edges(1);
                self.push(&[0, 1, 1, 1]);
                self.jmp(2, 2);
                self.nl();
                self.push(&[0, 1, 1, 0]);
                self.pop();
                self.pop();
                self.nl();
            }
            b',' => {
                self.handle_edges(1);
                self.push(&[0, 1, 1]);
                self.input();
                self.jmp(2, 2);
                self.nl();
                self.push(&[0, 1, 1]);
                self.input();
                self.pop();
                self.pop();
                self.nl();
            }

            b'[' => {
                self.nl();
                let start = self.line;
                self.handle_edges(1);
                self.push(&[0, 1, 1, 0]);

                let jmp_idx = self.res.len();
                self.jmp(0, 0);
                self.nl();

                self.push(&[0, 1, 1, 1]);
                self.pop();
                self.pop();

                self.translate_prog(matching_brace(prog, idx));

                self.res.push(Jmp(start, start));
                self.nl();

                let end = self.line;

                self.res[jmp_idx] = Jmp(end, end);
            }

            b';' => {
                self.jmp(0, 4);
                {
                    self.push(&[0, 1]);
                    self.pop();
                    self.jmp(0, 3);
                    {
                        self.push(&[1]);
                        self.pop();
                        self.dup();
                        self.output();
                        self.pop();
                        self.pop();
                        self.jmp(3, 3);
                    }

                    {
                        self.nl();
                        self.output();
                        self.pop();
                        self.pop();
                        self.pop();
                        self.push(&[0, 0]);
                        self.jmp(2, 2);
                    }
                }

                {
                    self.nl();
                    self.output();
                }
                self.nl();
            }
            b'<' => {
                self.jmp(0, 4);
                self.pop();
                self.jmp(0, 3);
                self.pop();
                self.push(&[1, 0, 1, 1]);
                self.dup();
                self.pop();
                self.pop();
                self.jmp(3, 3);

                self.nl();

                self.push(&[0, 1, 0, 0, 1, 0, 1, 1, 0]);

                self.pop();
                self.pop();
                self.pop();
                self.jmp(2, 2);

                self.nl();

                self.push(&[1]);

                self.nl();

                let start = self.line;

                self.jmp(0, 1);
                self.pop();
                self.jmp(0, 4);
                self.pop();
                self.jmp(7, 10);

                self.nl();
                self.pop();
                self.pop();
                self.pop();
                self.dup();
                self.push(&[0, 1, 0]);
                self.jmp(10, 0);
                self.pop();
                self.res.push(Jmp(start, start));

                self.nl();
                self.pop();
                self.pop();
                self.dup();
                self.push(&[1, 0, 0]);
                self.jmp(7, 0);
                self.pop();
                self.res.push(Jmp(start, start));

                self.nl();
                self.pop();
                self.dup();
                self.push(&[1, 1, 1]);
                self.jmp(4, 0);
                self.pop();
                self.res.push(Jmp(start, start));

                self.nl();
                self.pop();
                self.dup();
                self.push(&[1, 1, 0]);
                self.jmp(1, 0);
                self.pop();
                self.res.push(Jmp(start, start));

                self.nl();
                self.pop();

                self.nl();

                let start = self.line;

                self.jmp(7, 0);
                self.push(&[0]);
                self.pop();

                self.dup();
                self.pop();

                self.dup();
                self.pop();

                self.dup();
                self.pop();

                self.res.push(Jmp(start, start));

                self.nl();
                self.pop();
            }

            _ => {}
        }

        *idx += 1;
    }

    fn dup(&mut self) {
        self.jmp(0, 1);
        self.push(&[1]);
        self.jmp(2, 2);
        self.nl();
        self.push(&[0]);
        self.nl();
    }

    fn handle_edges(&mut self, one: usize) {
        self.jmp(0, 2);
        self.pop();
        self.jmp(1, 0);
        self.push(&[0, 1, 0, 0]);
        self.nl();

        self.pop();
        self.jmp(one + 1, 0);
        self.pop();
        self.pop();

        self.nl();
    }

    fn nl(&mut self) {
        self.line += 1;
        self.res.push(Nl);
    }

    fn output(&mut self) {
        self.res.push(Output);
    }

    fn input(&mut self) {
        self.res.push(Input);
    }

    fn pop(&mut self) {
        self.res.push(Pop);
    }

    fn push(&mut self, x: &[u8]) {
        self.res.extend(x.iter().map(|&i| Push(i > 0)));
    }

    fn jmp(&mut self, if_: usize, else_: usize) {
        let f = |x| if x > 0 { self.line + x } else { 0 };
        self.res.push(Jmp(f(if_), f(else_)))
    }
}
