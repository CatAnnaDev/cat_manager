use std::fmt::{Display, Formatter};
use rand::{distributions::{Distribution, Standard}, Rng};

#[derive(Clone, Copy, Default)]
pub enum ColorType {
    #[default]
    CodeBarre,
    Blanc,
    Noir,
    Roux,
    Gris,
    Tigre,
    Calico,
    Bleu,
    Marron,
    Tricolore,
    EcailleDeTortue,
    Champagne,
    Lilas,
    Creme,
    Cannelle,
    Beige,
    Argente,
    Dore,
}

impl Distribution<ColorType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ColorType {
        match rng.gen_range(0..=17) {
            0 => ColorType::CodeBarre,
            1 => ColorType::Blanc,
            2 => ColorType::Noir,
            3 => ColorType::Roux,
            4 => ColorType::Gris,
            5 => ColorType::Tigre,
            6 => ColorType::Calico,
            7 => ColorType::Bleu,
            8 => ColorType::Marron,
            9 => ColorType::Tricolore,
            10 => ColorType::EcailleDeTortue,
            11 => ColorType::Champagne,
            12 => ColorType::Lilas,
            13 => ColorType::Creme,
            14 => ColorType::Cannelle,
            15 => ColorType::Beige,
            16 => ColorType::Argente,
            17 => ColorType::Dore,
            _ => unreachable!(),
        }
    }
}

impl Display for ColorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorType::CodeBarre => write!(f, "Code barre"),
            ColorType::Blanc => write!(f, "Blanc"),
            ColorType::Noir => write!(f, "Noir"),
            ColorType::Roux => write!(f, "Roux"),
            ColorType::Gris => write!(f, "Gris"),
            ColorType::Tigre => write!(f, "Tigré"),
            ColorType::Calico => write!(f, "Calico"),
            ColorType::Bleu => write!(f, "Bleu"),
            ColorType::Marron => write!(f, "Marron"),
            ColorType::Tricolore => write!(f, "Tricolore"),
            ColorType::EcailleDeTortue => write!(f, "Écaille de Tortue"),
            ColorType::Champagne => write!(f, "Champagne"),
            ColorType::Lilas => write!(f, "Lilas"),
            ColorType::Creme => write!(f, "Crème"),
            ColorType::Cannelle => write!(f, "Cannelle"),
            ColorType::Beige => write!(f, "Beige"),
            ColorType::Argente => write!(f, "Argenté"),
            ColorType::Dore => write!(f, "Doré"),
        }
    }
}
