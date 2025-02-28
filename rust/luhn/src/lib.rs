pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|c| !c.is_whitespace())
        .try_rfold((0, 0), |(count, sum), c| {
            c.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (count + 1, sum + num))
        })
        .is_some_and(|(count, sum)| count > 1 && sum % 10 == 0)
}
