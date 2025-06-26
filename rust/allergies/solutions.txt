#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Allergen {
    Eggs         = 1,
    Peanuts      = 2,
    Shellfish    = 4,
    Strawberries = 8,
    Tomatoes     = 16,
    Chocolate    = 32,
    Pollen       = 64,
    Cats         = 128,
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
        self.score & (*allergen as u8) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result = Vec::with_capacity(8);
        let mut remaining = self.score;
        let mut bit_pos = 0u8;

        while remaining != 0 {
            if remaining & 1 != 0 {
                if let Some(allergen) = Self::bit_to_allergen(bit_pos) {
                    result.push(allergen);
                }
            }

            remaining >>= 1;
            bit_pos += 1;
        }

        result
    }

    #[inline]
    fn bit_to_allergen(pos: u8) -> Option<Allergen> {
        match pos {
            0 => Some(Allergen::Eggs),
            1 => Some(Allergen::Peanuts),
            2 => Some(Allergen::Shellfish),
            3 => Some(Allergen::Strawberries),
            4 => Some(Allergen::Tomatoes),
            5 => Some(Allergen::Chocolate),
            6 => Some(Allergen::Pollen),
            7 => Some(Allergen::Cats),
            _ => None,
        }
    }
}
