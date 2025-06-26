#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Allergen {
    Eggs         = 1 << 0,
    Peanuts      = 1 << 1,
    Shellfish    = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes     = 1 << 4,
    Chocolate    = 1 << 5,
    Pollen       = 1 << 6,
    Cats         = 1 << 7,
}

pub struct Allergies {
    score: u8,
}
use self::Allergen::*;

impl Allergies {
    const ALLERGIES: [Allergen; 8] = [
        Eggs,
        Peanuts,
        Shellfish,
        Strawberries,
        Tomatoes,
        Chocolate,
        Pollen,
        Cats,
    ];

    pub fn new(score: u32) -> Self {
        Self {
            score: (score & 0xff) as u8,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u8 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Self::ALLERGIES
            .into_iter()
            .filter(|allergen| self.is_allergic_to(allergen))
            .collect()
    }
}
