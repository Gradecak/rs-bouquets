use crate::types;
use std::collections::HashMap;

pub struct Assember<'a> {
    designs: &'a Vec<types::DesignSpec>,
    inventory: HashMap<char, u16>,
}

impl<'a> Assember<'a> {
    pub fn new(designs: &'a Vec<types::DesignSpec>) -> Self {
        return Assember {
            designs,
            inventory: HashMap::new(),
        };
    }

    pub fn add_flower(&mut self, flower: types::Flower) -> Option<types::Bouquet> {
        self.inventory
            .entry(flower.name)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);

        let assembled_bouquet = self
            .designs
            .iter()
            .filter(|design| design.has_stem(flower.name))
            .find_map(|design| {
                let mut bouquet_stems: Vec<types::DesignStem> = design
                    .stems
                    .iter()
                    .filter_map(|stem| {
                        let inv_amount = self.inventory.get(&stem.name).unwrap_or(&0u16);
                        let stem_max: u16 = stem.amount.into();
                        let amount = std::cmp::min(stem_max, *inv_amount);
                        if amount > 0 {
                            return Some(types::DesignStem {
                                amount: amount as u8,
                                name: stem.name,
                            });
                        }
                        return None;
                    })
                    .collect();
                let bouquet_total = bouquet_stems
                    .iter()
                    .fold(0u16, |acc, s| acc + s.amount as u16);
                // ensure we have enough stems in the inventory to make the bouquet
                if bouquet_stems.len() != design.stems.len() || bouquet_total < design.total {
                    return None;
                }

                // at this point we know that we have a bouquet, but we need to account
                // for having more stems available than the design total allows for
                let mut overflow = bouquet_total - design.total;
                'search: loop {
                    for mut stem in bouquet_stems.iter_mut() {
                        if overflow < 1 {
                            break 'search;
                        }
                        if stem.amount > 1 {
                            stem.amount -= 1;
                            overflow -= 1;
                        }
                    }
                }

                return Some(types::Bouquet {
                    design: design.design,
                    size: design.size,
                    stems: bouquet_stems,
                    total: design.total,
                });
            });

        // if we've found a bouquet, deduct it from the inventory
        if let Some(bouquet) = assembled_bouquet.as_ref() {
            for stem in bouquet.stems.iter() {
                self.inventory
                    .entry(stem.name)
                    .and_modify(|c| *c -= stem.amount as u16);
            }
        }
        return assembled_bouquet;
    }
}
