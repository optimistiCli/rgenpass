use std::iter::repeat;
use iwrtb::Braggard;

use genpass::GenPass;

mod args;
use args::Args;

fn main() {
    let args: Args = argh::from_env();
    let usage = args.usage.clone().unwrap();
    let brg = Braggard::new(&usage);

    let req = args.check();
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
    if let Some(special) = req.special {
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
                        brg.brag_and_exit(&msg);
                        String::new()
                    }
                )
            }
        )
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", passwords)

    // let mut gp = GenPass::new();
    // gp.add_interval('0', '9');
    // gp.add_interval('a', 'z');
    // gp.add_interval('A', 'Z');
    // gp.add_list("!$%@#");
    // println!("{:?}", gp.generate(4));
    // println!("{:?}", gp.generate(16));
    // println!("{:?}", gp.generate_all(4));
    // println!("{:?}", gp.generate_all(16));
}
