const FIRST_SQUARE: u32 = 1;
const LAST_SQUARE: u32 = 64;

pub fn square(s: u32) -> u64 {
    if !(FIRST_SQUARE <= s && s <= LAST_SQUARE) {
        panic!(format!(
            "Square must be between {} and {}",
            FIRST_SQUARE, LAST_SQUARE
        ));
    }

    (2 as u64).pow(s - 1)
}

pub fn total() -> u64 {
    (FIRST_SQUARE..=LAST_SQUARE).map(|n| square(n)).sum::<u64>()
}
