#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Allergen {
    Eggs         = 1 << 0u8,
    Peanuts      = 1 << 1u8,
    Shellfish    = 1 << 2u8,
    Strawberries = 1 << 3u8,
    Tomatoes     = 1 << 4u8,
    Chocolate    = 1 << 5u8,
    Pollen       = 1 << 6u8,
    Cats         = 1 << 7u8,
}

pub struct Allergies {
    score: u8,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self {
            score: (score & 0xff) as u8,
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.score & *allergen as u8 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut allergens = Vec::with_capacity(8);
        for allergen in [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ] {
            if self.is_allergic_to(&allergen) {
                allergens.push(allergen);
            }
        }
        allergens
    }
}
