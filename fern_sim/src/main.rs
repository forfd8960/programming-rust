mod plant_structures;
mod spores;

use plant_structures::{leaves, roots, stems};

fn main() {
    println!("Hello, world!");

    /*
        call spores...
    calling fn1 in Spore
    only visiable in spore module
    calling pub crate fn2...
    only visiable in spore module
        */
    println!("call spores...");
    spores::fn1();
    spores::fn2();

    /*
        call plant_structures...
    init leave: Leaves
    init roots: Roots
    init stems: Stems
        */
    println!("call plant_structures...");
    let leave = leaves::Leaves::new();
    println!("init leave: {:?}", leave);

    let roots = roots::Roots::new();
    println!("init roots: {:?}", roots);

    let stms = stems::Stems::new();
    println!("init stems: {:?}", stms);
}
