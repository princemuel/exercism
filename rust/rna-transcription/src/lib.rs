use ::core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Dna {
	strand: Vec<Nucleotide>,
}

impl Dna {
	pub fn new(strand: &str) -> Result<Self, usize> {
		let nucleotides = Nucleotide::parse(strand, MoleculeType::Dna)?;

		for (i, &nucleotide) in nucleotides.iter().enumerate() {
			if !nucleotide.is_dna() {
				return Err(i);
			}
		}

		Ok(Self { strand: nucleotides })
	}
	pub fn into_rna(&self) -> Rna {
		Rna { strand: self.strand.iter().map(|&n| n.to_rna_complement()).collect() }
	}
}

impl fmt::Display for Dna {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for nucleotide in &self.strand {
			write!(f, "{}", char::from(*nucleotide))?;
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Eq)]
pub struct Rna {
	strand: Vec<Nucleotide>,
}

impl Rna {
	pub fn new(strand: &str) -> Result<Self, usize> {
		let nucleotides = Nucleotide::parse(strand, MoleculeType::Rna)?;

		for (i, &nucleotide) in nucleotides.iter().enumerate() {
			if !nucleotide.is_rna() {
				return Err(i);
			}
		}

		Ok(Self { strand: nucleotides })
	}

	pub fn into_dna(self) -> Dna {
		Dna {
			strand: self
				.strand
				.into_iter()
				.map(|n| n.to_dna_complement())
				.collect(),
		}
	}
}

impl fmt::Display for Rna {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for nucleotide in &self.strand {
			write!(f, "{}", char::from(*nucleotide))?;
		}
		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nucleotide {
	Adenine,  // A
	Cytosine, // C
	Guanine,  // G
	Thymine,  // T
	Uracil,   // U
}

impl Nucleotide {
	pub fn parse(s: &str, molecule_type: MoleculeType) -> Result<Vec<Self>, usize> {
		let mut nucleotides = Vec::with_capacity(s.len());
		let is_valid = molecule_type.predicate();

		for (i, c) in s.chars().enumerate() {
			match Self::try_from(c) {
				Ok(n) if is_valid(n) => nucleotides.push(n),
				_ => return Err(i),
			}
		}

		Ok(nucleotides)
	}

	pub const fn to_rna_complement(self) -> Self {
		match self {
			Self::Guanine => Self::Cytosine,
			Self::Cytosine => Self::Guanine,
			Self::Thymine => Self::Adenine,
			Self::Adenine => Self::Uracil,
			Self::Uracil => panic!("Uracil is not valid in DNA"),
		}
	}

	pub const fn to_dna_complement(self) -> Self {
		match self {
			Self::Guanine => Self::Cytosine,
			Self::Cytosine => Self::Guanine,
			Self::Uracil => Self::Adenine,
			Self::Adenine => Self::Thymine,
			Self::Thymine => panic!("Thymine is not valid in RNA"),
		}
	}

	pub const fn is_dna(self) -> bool {
		!matches!(self, Self::Uracil)
	}

	pub const fn is_rna(self) -> bool {
		!matches!(self, Self::Thymine)
	}
}

impl TryFrom<char> for Nucleotide {
	type Error = ();
	fn try_from(c: char) -> Result<Self, Self::Error> {
		match c {
			'A' => Ok(Self::Adenine),
			'C' => Ok(Self::Cytosine),
			'G' => Ok(Self::Guanine),
			'T' => Ok(Self::Thymine),
			'U' => Ok(Self::Uracil),
			_ => Err(()),
		}
	}
}

impl From<Nucleotide> for char {
	fn from(n: Nucleotide) -> char {
		match n {
			Nucleotide::Adenine => 'A',
			Nucleotide::Cytosine => 'C',
			Nucleotide::Guanine => 'G',
			Nucleotide::Thymine => 'T',
			Nucleotide::Uracil => 'U',
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub enum MoleculeType {
	Dna,
	Rna,
}

impl MoleculeType {
	fn predicate(&self) -> fn(Nucleotide) -> bool {
		match self {
			MoleculeType::Dna => Nucleotide::is_dna,
			MoleculeType::Rna => Nucleotide::is_rna,
		}
	}
}
