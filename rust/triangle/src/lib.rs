#[derive(Debug)]
pub struct Triangle<T> {
	sides: [T; 3],
}

impl<T: Default> Triangle<T>
where
	T: Copy + PartialOrd + std::ops::Add<Output = T> + PartialEq,
{
	pub fn build(sides: [T; 3]) -> Option<Self> {
		Self::is_triangle(&sides).then_some(Self { sides })
	}

	pub fn is_equilateral(&self) -> bool {
		let [a, b, c] = self.sides;
		a == b && b == c
	}

	pub fn is_scalene(&self) -> bool {
		!self.is_isosceles() && !self.is_equilateral()
	}

	pub fn is_isosceles(&self) -> bool {
		let [a, b, c] = self.sides;
		a == b || b == c || a == c
	}

	fn is_triangle(sides: &[T; 3]) -> bool {
		if sides.iter().any(|&x| x <= T::default()) {
			return false;
		}

		let [a, b, c] = *sides;
		a + b > c && a + c > b && b + c > a
	}
}
