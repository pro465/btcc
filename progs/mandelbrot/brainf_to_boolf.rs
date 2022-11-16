use std::io::prelude::*;

fn main() {
    std::io::stdin()
        .bytes()
        .map(Result::unwrap)
        .map(|i| {
            Some(match i {
                b'+' => ">[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<",
                b'-' => ">>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<",
                b'[' => ">>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]",
                b']' => ">>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]",
                b',' => ">,>,>,>,>,>,>,>,<<<<<<<<",
                b'.' => ">;>;>;>;>;>;>;>;<<<<<<<<",
                b'<' => "<<<<<<<<<",
                b'>' => ">>>>>>>>>",
                _ => return None,
            })
        })
        .for_each(|i| {
            i.map(|i| println!("{i}"));
        });
}
