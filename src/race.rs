use std::fmt::{Display, Formatter};
use rand::{distributions::{Distribution, Standard}, Rng};

#[derive(Clone, Copy, Default)]
pub enum Race {
    #[default]
    Persan,
    Siamois,
    MaineCoon,
    Sphynx,
    Bengal,
    Europeen,
    Birman,
    Chartreux,
    Ragdoll,
    Abyssin,
    ScottishFold,
    AmericanShorthair,
    ExoticShorthair,
    Oriental,
    Norvegien,
    AngoraTurc,
    Burmese,
    Manx,
    Korat,
}

impl Distribution<Race> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Race {
        match rng.gen_range(0..=18) {
            0 => Race::Persan,
            1 => Race::Siamois,
            2 => Race::MaineCoon,
            3 => Race::Sphynx,
            4 => Race::Bengal,
            5 => Race::Europeen,
            6 => Race::Birman,
            7 => Race::Chartreux,
            8 => Race::Ragdoll,
            9 => Race::Abyssin,
            10 => Race::ScottishFold,
            11 => Race::AmericanShorthair,
            12 => Race::ExoticShorthair,
            13 => Race::Oriental,
            14 => Race::Norvegien,
            15 => Race::AngoraTurc,
            16 => Race::Burmese,
            17 => Race::Manx,
            18 => Race::Korat,
            _ => unreachable!(),
        }
    }
}

impl Display for Race {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Race::Persan => write!(f, "Persan"),
            Race::Siamois => write!(f, "Siamois"),
            Race::MaineCoon => write!(f, "Maine Coon"),
            Race::Sphynx => write!(f, "Sphynx"),
            Race::Bengal => write!(f, "Bengal"),
            Race::Europeen => write!(f, "Européen"),
            Race::Birman => write!(f, "Birman"),
            Race::Chartreux => write!(f, "Chartreux"),
            Race::Ragdoll => write!(f, "Ragdoll"),
            Race::Abyssin => write!(f, "Abyssin"),
            Race::ScottishFold => write!(f, "Scottish Fold"),
            Race::AmericanShorthair => write!(f, "American Shorthair"),
            Race::ExoticShorthair => write!(f, "Exotic Shorthair"),
            Race::Oriental => write!(f, "Oriental"),
            Race::Norvegien => write!(f, "Norvégien"),
            Race::AngoraTurc => write!(f, "Angora Turc"),
            Race::Burmese => write!(f, "Burmese"),
            Race::Manx => write!(f, "Manx"),
            Race::Korat => write!(f, "Korat"),
        }
    }
}
