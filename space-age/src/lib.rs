#[derive(Debug)]
pub struct Duration {
    earth_years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_years: s as f64 / ONE_EARTH_SECONDS,
        }
    }
}

const ONE_EARTH_SECONDS: f64 = 31_557_600.0;

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::ORBITAL_PERIOD
    }
}

macro_rules! planet {
    ($pln: ident, $period: expr) => {
        pub struct $pln;
        impl Planet for $pln {
            const ORBITAL_PERIOD: f64 = $period;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
