//! Core ternary logic types and traits

/// The fundamental ternary value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Trit {
    False = 0,
    Unknown = 1,
    True = 2,
}

impl Trit {
    pub fn is_known(self) -> bool { self != Trit::Unknown }
    pub fn is_true(self) -> bool { self == Trit::True }
    pub fn is_false(self) -> bool { self == Trit::False }

    pub fn not(self) -> Self {
        match self {
            Trit::True => Trit::False,
            Trit::False => Trit::True,
            Trit::Unknown => Trit::Unknown,
        }
    }

    pub fn and(self, other: Self) -> Self {
        use Trit::*;
        match (self, other) {
            (False, _) | (_, False) => False,
            (Unknown, _) | (_, Unknown) => Unknown,
            (True, True) => True,
        }
    }

    pub fn or(self, other: Self) -> Self {
        use Trit::*;
        match (self, other) {
            (True, _) | (_, True) => True,
            (Unknown, _) | (_, Unknown) => Unknown,
            (False, False) => False,
        }
    }

    /// Kleene consensus: agree if both agree, unknown otherwise
    pub fn consensus(self, other: Self) -> Self {
        if self == other { self } else { Trit::Unknown }
    }
}

/// Trait for types that can be evaluated as ternary
pub trait TernaryEval {
    fn eval(&self) -> Trit;
}

impl TernaryEval for Trit {
    fn eval(&self) -> Trit { *self }
}

impl TernaryEval for bool {
    fn eval(&self) -> Trit {
        if *self { Trit::True } else { Trit::False }
    }
}

/// Tryte: 6 trits (common ternary word size)
#[derive(Debug, Clone, Copy)]
pub struct Tryte(pub [Trit; 6]);

impl Tryte {
    pub fn zero() -> Self { Tryte([Trit::Unknown; 6]) }

    pub fn to_balance(&self) -> i32 {
        self.0.iter().enumerate()
            .map(|(i, t)| (1i32 - *t as i32) * 3i32.pow(i as u32))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trit_logic() {
        assert!(Trit::True.and(Trit::False).is_false());
        assert!(Trit::True.or(Trit::False).is_true());
        assert!(Trit::True.not().is_false());
        assert_eq!(Trit::True.consensus(Trit::False), Trit::Unknown);
    }

    #[test]
    fn tryte_balance() {
        let t = Tryte::zero();
        assert_eq!(t.to_balance(), 0);
    }
}
