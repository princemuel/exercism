pub fn square(s: u32) -> u64 {
    if !(1..=64).contains(&s) {
        panic!("square must be between 1 and 64");
    }

    1 << (s - 1)
}

pub fn total() -> u64 {
    // (1u64 << 63) + ((1u64 << 63) - 1)
    // ((1_u128 << 64) - 1).try_into().unwrap()
    u64::MAX
}
