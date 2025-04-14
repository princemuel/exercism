pub fn egg_count(n: u32) -> usize {
    format!("{:b}", n).chars().filter(|&c| c == '1').count()
}


