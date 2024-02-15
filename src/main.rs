use genpass::GenPass;

fn main() {
    let mut gp = GenPass::new();
    gp.add_interval('0', '9');
    gp.add_interval('a', 'z');
    gp.add_interval('A', 'Z');
    gp.add_list("!$%@#");
    println!("{:?}", gp.generate(16));
}
