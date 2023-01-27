use std::error::Error;
use std::fmt;
use std::str::FromStr;

// #[derive(Debug, Clone, Copy)]
// pub enum DesignCode {
//     A,
//     B,
// }

// impl fmt::Display for DesignCode {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match *self {
//             Self::A => write!(f, "A"),
//             Self::B => write!(f, "B"),
//         }
//     }
// }

// impl FromStr for DesignCode {
//     type Err = ();

//     fn from_str(input: &str) -> Result<DesignCode, Self::Err> {
//         match input {
//             "A" => Ok(DesignCode::A),
//             "B" => Ok(DesignCode::B),
//             _ => Err(()),
//         }
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum StemSize {
    S,
    L,
}

impl fmt::Display for StemSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::S => write!(f, "S"),
            Self::L => write!(f, "L"),
        }
    }
}

impl FromStr for StemSize {
    type Err = ();

    fn from_str(input: &str) -> Result<StemSize, Self::Err> {
        match input {
            "S" => Ok(StemSize::S),
            "L" => Ok(StemSize::L),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DesignStem {
    pub amount: u8,
    pub name: char,
}

impl fmt::Display for DesignStem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.amount, self.name)
    }
}

impl FromStr for DesignStem {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<DesignStem, Self::Err> {
        let mut iter = input.chars().rev();
        let name = iter.next();
        if !name.unwrap().is_lowercase() {
            return Err("Name is not lowercase")?;
        }
        let digits = iter.rev().collect::<String>().parse::<u8>();
        match (digits, name) {
            (Err(e), _) => Err(e)?,
            (_, None) => Err("Name not found")?,
            (Ok(amount), Some(name)) => Ok(DesignStem { amount, name }),
        }
    }
}

#[derive(Debug)]
pub struct DesignSpec {
    pub design: char,
    pub size: StemSize,
    pub stems: Vec<DesignStem>,
    pub total: u16,
}

impl DesignSpec {
    pub fn has_stem(&self, stem_name: char) -> bool {
        self.stems
            .iter()
            .find(|stem| stem.name == stem_name)
            .is_some()
    }

    pub fn to_bouqet(self, configuration: Vec<DesignStem>) -> Bouquet {
        return Bouquet {
            design: self.design,
            size: self.size,
            stems: configuration,
            total: self.total,
        };
    }
}

impl FromStr for DesignSpec {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut input = s.chars().clone();
        let design = input.next().unwrap();
        let size = input
            .next()
            .map(|c| StemSize::from_str(c.to_string().as_str()).unwrap())
            .unwrap();

        let mut stems: Vec<DesignStem> = Vec::new();
        let mut design_part = String::new();
        for c in input {
            design_part.push(c);
            if c.is_alphabetic() {
                match DesignStem::from_str(design_part.as_str()) {
                    Ok(stem) => stems.push(stem),
                    Err(e) => return Err(e),
                }
                design_part = String::new();
            }
        }
        let total = design_part.parse::<u16>()?;

        return Ok(DesignSpec {
            design,
            size,
            stems,
            total,
        });
    }
}

#[derive(Debug)]
pub struct Flower {
    pub name: char,
    pub size: StemSize,
}

impl FromStr for Flower {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        return Ok(Flower {
            name: iter.next().unwrap(),
            size: StemSize::from_str(iter.next().unwrap().to_string().as_str()).unwrap(),
        });
    }
}

pub type Bouquet = DesignSpec;

impl fmt::Display for Bouquet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stems = self.stems.iter().fold(String::new(), |mut acc, stem| {
            acc.push_str(&format!("{}", stem));
            return acc;
        });
        write!(f, "{}{}{}", self.design, self.size, stems)
    }
}
