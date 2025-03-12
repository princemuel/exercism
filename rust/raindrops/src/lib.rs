pub fn raindrops(n: u32) -> String {
    let result: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter_map(|&(k, v)| (n % k == 0).then_some(v))
        .collect();

    result.is_empty().then(|| n.to_string()).unwrap_or(result)
}
