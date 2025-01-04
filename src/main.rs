use std::iter::repeat;

use iwgenpass::GenPass;

mod args;
use args::Requset;

fn main() {
    let req = Requset::new();
    let mut gp = GenPass::new();
    if req.digits {
        gp.add_interval('0', '9');
    }
    if req.lower {
        gp.add_interval('a', 'z');
    }
    if req.upper {
        gp.add_interval('A', 'Z');
    }
    if let Some(ref special) = req.special {
        gp.add_list(&special);
    }

    let passwords = repeat(())
        .take(req.num)
        .map(
            |_| {
                {
                    if req.all {
                        gp.generate_all(req.length)
                    } else {
                        gp.generate(req.length)
                    }
                }
                .unwrap_or_else(
                    |msg| {
                        req.brag_and_exit(&msg);
                        String::new()
                    }
                )
            }
        )
        .collect::<Vec<_>>()
        .join("\n");
    // TODO: Print trailing newline only if writing to a terminal
    println!("{}", passwords)
}
