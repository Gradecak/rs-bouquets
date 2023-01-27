pub mod assembler;
pub mod types;

use std::io;
use std::str::FromStr;

fn is_impossible_design(design: &types::DesignSpec) -> bool {
    let design_maximum = design
        .stems
        .iter()
        .fold(0u16, |acc, stem| acc + stem.amount as u16);

    return !(design.stems.len() > design.total.into() || design_maximum < design.total);
}

fn main() {
    let mut stdin = io::stdin().lines();
    let lines_iter = stdin.by_ref();

    let designs: Vec<types::DesignSpec> = lines_iter
        .take_while(|l| !l.as_ref().unwrap().is_empty())
        .map(|l| types::DesignSpec::from_str(l.unwrap().as_str()).unwrap())
        .filter(is_impossible_design)
        .collect();

    let (large_designs, small_designs) = designs
        .into_iter()
        .partition(|design| matches!(design.size, types::StemSize::L));

    let mut large_assembler = assembler::Assember::new(&large_designs);
    let mut small_assembler = assembler::Assember::new(&small_designs);
    let flowers = lines_iter.map(|l| types::Flower::from_str(l.unwrap().as_str()).unwrap());

    for flower in flowers {
        let assembler: &mut assembler::Assember = if matches!(flower.size, types::StemSize::L) {
            &mut large_assembler
        } else {
            &mut small_assembler
        };

        match assembler.add_flower(flower) {
            Some(bouquet) => println!("{}", bouquet),
            _ => (),
        }
    }
}
