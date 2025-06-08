use ::core::fmt::Display;

pub trait Luhn {
	fn valid_luhn(&self) -> bool;
}

impl<T: Display> Luhn for T {
	fn valid_luhn(&self) -> bool {
		self.to_string()
			.bytes()
			.rev()
			.filter(|&b| b != b' ')
			.try_fold((0u32, 0u32), |(total, count), b| {
				if b.is_ascii_digit() {
					let digit = (b - b'0') as u32;
					let doubled = if count % 2 == 0 { digit } else { digit * 2 };
					let adjusted = if doubled > 9 { doubled - 9 } else { doubled };
					Some((total + adjusted, count + 1))
				} else {
					None
				}
			})
			.is_some_and(|(total, count)| total % 10 == 0 && count > 1)
	}
}
