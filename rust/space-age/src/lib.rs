const EARTH_YEAR_IN_SECS: f64 = 31_557_600.0;

#[derive(Debug)]
pub struct Duration {
    seconds: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self { seconds: s as f64 }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.seconds / (Self::ORBITAL_PERIOD * EARTH_YEAR_IN_SECS)
    }
}

macro_rules! impl_planet {
    ($planet:ident, $period:expr) => {
        pub struct $planet;
        impl Planet for $planet {
            const ORBITAL_PERIOD: f64 = $period;
        }
    };
}

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);
